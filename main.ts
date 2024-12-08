import { Command } from "commander";
import { createCardsCommands } from "./commands/cards/mod.ts";
import { createDecksCommands } from "./commands/decks/mod.ts";

const program = new Command();

program
    .name("ducki")
    .description("Train with flashcards from the command line")
    .version("0.0.1");

createDecksCommands(program);
createCardsCommands(program);

program.parse(Deno.args, { from: "user" });
