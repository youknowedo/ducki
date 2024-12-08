import { join } from "@std/path";
import prompts from "prompts";
import { Card, createEmptyCard, FSRS, IPreview, RecordLogItem } from "ts-fsrs";
import { getConfig } from "../../config.ts";
import { Deck } from "../../types.d.ts";

export const study = async (slug?: string) => {
    const now = Date.now();
    const config = await getConfig();

    if (Object.keys(config.decks).length === 0) {
        return console.log("No decks found");
    }

    slug = slug ??
        ((
            await prompts({
                type: "autocomplete",
                name: "slug",
                message: "Select a deck to study",
                choices: Object.keys(config.decks).map((slug) => ({
                    title: slug,
                    value: slug,
                })),
            })
        ).slug as string);

    if (!config.decks[slug]) {
        console.log("Deck not found");
        return;
    }

    let deck: Deck;
    try {
        deck = JSON.parse(
            await Deno.readTextFile(join(config.decks[slug].path, "deck.json")),
        );
    } catch (err) {
        if (err instanceof Deno.errors.NotFound) {
            console.log("Deck not found at path");
            return;
        }
        throw err;
    }

    // read .progess.json in the deck folder. if not, create it.
    const progressPath = join(config.decks[slug].path, ".progress.json");

    let progress: {
        [key: string]: Card;
    } = {};

    try {
        progress = JSON.parse(await Deno.readTextFile(progressPath));
    } catch (err) {
        if (!(err instanceof Deno.errors.NotFound)) {
            throw err;
        }
    }

    for (const slug in progress) {
        if (Object.prototype.hasOwnProperty.call(progress, slug)) {
            if (!deck.cards[slug]) {
                delete progress[slug];
            }
        }
    }

    const cards = Object.entries(progress)
        .filter(([, card]) => new Date(card.due).getTime() <= now)
        .map(([slug, card]) => ({ slug, ...card }));

    for (const slug in deck.cards) {
        if (Object.prototype.hasOwnProperty.call(deck.cards, slug)) {
            if (!progress[slug]) {
                progress[slug] = createEmptyCard();
                cards.push({ slug, ...progress[slug] });
            }
        }
    }

    await Deno.writeTextFile(progressPath, JSON.stringify(progress));

    if (cards.length === 0) {
        console.log("No cards to study");
        return;
    }

    const cardIndex = Math.floor(Math.random() * cards.length);
    const card = cards[cardIndex];

    const f = new FSRS({});
    const schedule = f.repeat(card, now);

    const response = await prompts([
        {
            type: "text",
            name: "answer",
            message: deck.cards[card.slug].front,
        },
        {
            type: "select",
            name: "rating",
            message: `Correct answer was "${
                deck.cards[card.slug].back
            }". How did you do?`,
            choices: [
                { title: "Again", value: 1 },
                { title: "Hard", value: 2 },
                { title: "Good", value: 3 },
                { title: "Easy", value: 4 },
            ],
        },
    ]);

    const rating = response.rating as keyof IPreview;

    const logItem = schedule[rating] as RecordLogItem;

    progress[card.slug] = logItem.card;

    await Deno.writeTextFile(progressPath, JSON.stringify(progress));

    // TODO: Add logging
};
