export type Config = {
    decks: {
        [slug: string]: {
            description?: string;
            path: string;
        };
    };
};

export const getConfig = async () => {
    try {
        return JSON.parse(
            await Deno.readTextFile(Deno.env.get("HOME") + "/.ducki.json"),
        ) as Config;
    } catch (err) {
        if (err instanceof Deno.errors.NotFound) {
            return { decks: {} } as Config;
        }

        throw err;
    }
};

export const updateConfig = async (
    newConfig: Config | ((current: Config) => Promise<Config> | Config),
) => {
    let config: Config;

    try {
        config = JSON.parse(
            await Deno.readTextFile(Deno.env.get("HOME") + "/.ducki.json"),
        );
    } catch (err) {
        if (!(err instanceof Deno.errors.NotFound)) {
            throw err;
        }

        config = { decks: {} };
    }

    config = typeof newConfig === "function"
        ? await newConfig(config)
        : newConfig;

    await Deno.writeTextFile(
        Deno.env.get("HOME") + "/.ducki.json",
        JSON.stringify(config, null, 4),
    );
};
