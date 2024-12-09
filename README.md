# ducki

Study flashcards from the comfort of your terminal to minimize distractions.
Schedules cards using the Free Spaced Repetition Scheduler (FSRS) for maximized
learning.

NOTE: This project us very much a WIP and commands/config files are subject to
change before the first full release.

## Commands

- `list|ls` List all decks
- `init|i [slug]` Add a new deck
- `add [path]` Add an existing deck
- `remove|rm [slug]` Remove a deck
- `study [slug]` Study a deck
- `deck`
  - `add [slug] [id] [front] [back]` Add a card to a deck
  - `remove|rm [slug] [id]` Remove a card from a deck
  - `help [command]` display help for command

## Get started

1. Create a new deck with `ducki init`
2. Add a card to the deck with `ducki deck add`
3. Study the new deck with `ducki study`

# Building

## Host-only
```bash
cargo build
```

## Cross-platform
```bash
cargo install cross --git https://github.com/cross-rs/cross
./build.sh
```

## TODOS:

- [x] Migrate from Deno to Rust (90MB for a SFE???) [#1](https://github.com/youknowedo/ducki/issues/1)
- [ ] Better command structure
      (`ducki <deckSlug> add [id] -f [front] -b [back]`) [#2](https://github.com/youknowedo/ducki/issues/2)
- [ ] Logging & Undo [#3](https://github.com/youknowedo/ducki/issues/3)
- [ ] Import cards to deck (CSV, JSON, etc.) [#4](https://github.com/youknowedo/ducki/issues/4)
- [ ] VIM-like navigation [#5](https://github.com/youknowedo/ducki/issues/5)
- [ ] Full-screen interface (get rid of `npm:prompts`) [#6](https://github.com/youknowedo/ducki/issues/6)

## License

Copyright &copy; 2024 Sigfredo. Licensed under the [MIT License](./LICENSE)
