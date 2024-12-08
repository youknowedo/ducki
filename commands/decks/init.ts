import { basename, join } from "@std/path";
import prompts from "prompts";
import { updateConfig } from "../../config.ts";
import { Deck } from "../../types.d.ts";

export const init = async (slug?: string) => {
    slug = slug ??
        ((
            await prompts({
                type: "text",
                name: "slug",
                initial: ".",
                message: "Deck slug",
            })
        ).slug as string);

    const path = slug === "." ? Deno.cwd() : join(Deno.cwd(), slug);
    slug = basename(path);

    try {
        await Deno.mkdir(path, { recursive: true });
    } catch (err) {
        if (!(err instanceof Deno.errors.AlreadyExists)) {
            throw err;
        }
    }

    try {
        await Deno.lstat(join(path, "deck.json"));

        const overwrite = await prompts({
            type: "confirm",
            slug: "continue",
            message: "Deck already exists at path. Do you want to continue?",
        });

        if (!overwrite.continue) return;
    } catch (err) {
        if (!(err instanceof Deno.errors.NotFound)) {
            throw err;
        }
    }

    await updateConfig(async (config) => {
        if (config.decks[slug]) {
            const overwrite = await prompts({
                type: "confirm",
                slug: "continue",
                message:
                    "Deck with that slug already exists. Do you want to continue?",
            });

            if (!overwrite.continue) return config;
        }
        config.decks[slug] = { path };

        return config;
    });

    const deck: Deck = {
        slug,
        description: "",
        cards: {},
    };

    await Deno.writeTextFile(
        join(path, "deck.json"),
        JSON.stringify(deck, null, 4),
    );

    console.log(`Deck "${slug}" created at ${path}`);
};
