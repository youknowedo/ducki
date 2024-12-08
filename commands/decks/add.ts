import { basename } from "@std/path";
import prompts from "prompts";
import { getConfig, updateConfig } from "../../config.ts";

export const add = async (path = ".") => {
    const slug = basename(path);
    const config = await getConfig();

    if (config.decks[slug]) {
        const overwrite = await prompts({
            type: "confirm",
            name: "continue",
            message:
                "Deck with that slug already exists. Do you want to continue?",
        });
        if (!overwrite.continue) return;
    }

    config.decks[slug] = { path };
    await updateConfig(config);
};
