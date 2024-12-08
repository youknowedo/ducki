import type { Command } from "commander";
import { add } from "./add.ts";
import { remove } from "./remove.ts";
import { study } from "./study.ts";

export const createCardsCommands = (program: Command) => {
    program
        .command("study")
        .description("Study a deck")
        .argument("[slug]", "Deck slug")
        .action(study);

    const deck = program.command("deck");

    deck.command("add")
        .description("Add a card to a deck")
        .argument("[slug]", "Deck slug")
        .argument("[id]", "Card ID")
        .argument("[front]", "Front of the card")
        .argument("[back]", "Back of the card")
        .action(add);

    deck.command("remove")
        .alias("rm")
        .description("Remove a card from a deck")
        .argument("[slug]", "Deck slug")
        .argument("[id]", "Card ID")
        .action(remove);
};
