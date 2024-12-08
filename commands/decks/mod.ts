import type { Command } from "commander";
import { add } from "./add.ts";
import { init } from "./init.ts";
import { list } from "./list.ts";
import { remove } from "./remove.ts";

export const createDecksCommands = (program: Command) => {
    program
        .command("list")
        .alias("ls")
        .description("List all decks")
        .action(list);

    program
        .command("init")
        .alias("i")
        .description("Add a new deck")
        .argument("[slug]", "Deck slug")
        .action(init);

    program
        .command("add")
        .description("Add an existing deck")
        .argument("[path]", "Deck path")
        .action(add);

    program
        .command("remove")
        .alias("rm")
        .description("Remove a deck")
        .argument("[slug]", "Deck slug")
        .action(remove);
};
