import { join } from "@std/path";
import prompts from "prompts";
import { getConfig } from "../../config.ts";
import { Deck } from "../../types.d.ts";

export const add = async (
    deckSlug?: string,
    id?: string,
    front?: string,
    back?: string,
) => {
    const config = await getConfig();

    deckSlug = deckSlug ??
        ((
            await prompts({
                type: "autocomplete",
                name: "slug",
                message: "What deck do you want to add a card to?",
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
        ((await prompts({ type: "text", name: "id", message: "Card ID" }))
            .id as string);

    front = front ??
        ((await prompts({ type: "text", name: "front", message: "Front" }))
            .front as string);

    back = back ??
        ((await prompts({ type: "text", name: "back", message: "Back" }))
            .back as string);

    deck.cards[id] = { front, back };

    await Deno.writeTextFile(
        join(config.decks[deckSlug].path, "deck.json"),
        JSON.stringify(deck, null, 4),
    );

    console.log("");
};
