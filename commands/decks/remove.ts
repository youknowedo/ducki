import prompts from "prompts";
import { getConfig, updateConfig } from "../../config.ts";

export const remove = async (slug?: string) => {
    const config = await getConfig();

    if (Object.keys(config.decks).length === 0) {
        console.log("No decks to remove");
        return;
    }

    if (!slug) {
        slug = (
            await prompts({
                type: "autocomplete",
                name: "slug",
                message: "Select a deck to remove",
                choices: Object.keys(config.decks).map((slug) => ({
                    title: slug,
                    value: slug,
                })),
            })
        ).slug as string;
    } else if (!config.decks[slug]) {
        console.log("Deck not found");
        return;
    }

    const removePath = (
        await prompts({
            type: "confirm",
            name: "removePath",
            message: "Do you want to remove the deck folder?",
        })
    ).removePath;

    if (removePath) {
        await Deno.remove(config.decks[slug].path, { recursive: true });
    }

    await updateConfig((config) => {
        delete config.decks[slug];
        return config;
    });
};
