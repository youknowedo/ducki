import { join } from "@std/path";
import prompts from "prompts";
import { getConfig } from "../../config.ts";
import { Deck } from "../../types.d.ts";

export const remove = async (deckSlug?: string, id?: string) => {
    const config = await getConfig();

    deckSlug = deckSlug ??
        ((
            await prompts({
                type: "autocomplete",
                name: "slug",
                message: "In which deck do you want to remove a card?",
                choices: Object.keys(config.decks).map((slug) => ({
                    title: slug,
                    value: slug,
                })),
            })
        ).slug as string);

    let deck: Deck;
    try {
        deck = JSON.parse(
            await Deno.readTextFile(
                join(config.decks[deckSlug].path, "deck.json"),
            ),
        );
    } catch (err) {
        if (err instanceof Deno.errors.NotFound) {
            console.log("Deck not found at path");
            return;
        }
        throw err;
    }

    if (!config.decks[deckSlug]) {
        console.log("Deck not found");
        return;
    }

    id = id ??
        ((
            await prompts({
                type: "autocomplete",
                name: "id",
                message: "Card ID",
                choices: Object.keys(deck.cards).map((id) => ({
                    title: id,
                    value: id,
                })),
            })
        ).id as string);

    if (!deck.cards[id]) {
        console.log("Card not found");
        return;
    }

    delete deck.cards[id];

    const continueWithRemove = (
        await prompts({
            type: "confirm",
            name: "continue",
            message: "Are you sure you want to remove this card?",
        })
    ).continue;

    if (!continueWithRemove) return;

    await Deno.writeTextFile(
        join(config.decks[deckSlug].path, "deck.json"),
        JSON.stringify(deck, null, 4),
    );
};
