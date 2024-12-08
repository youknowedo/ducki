import { getConfig } from "../../config.ts";

export const list = async () => {
    const config = await getConfig();

    for (const slug in config.decks) {
        if (Object.prototype.hasOwnProperty.call(config.decks, slug)) {
            console.log(slug);
        }
    }
};
