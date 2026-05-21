# Rougue_MoTower

Bevy prototype for a hybrid Magic Tower map crawler and deck-builder card battle game.

## Run

```bash
cargo run
```

## Demo Controls

- Start menu: click an option, or use arrow keys / `W` / `S` to choose and `Enter` / `Space` to confirm.
- Settings: click or press `Enter` / `Space` to switch between Chinese and English UI text.
- `WASD` / arrow keys: move on the tower map.
- Mouse: click adjacent map tiles to move, click cards to play or pick rewards, click on-screen buttons for menu/end turn/skip.
- `1`-`5`: play cards in battle.
- `Space` / `Enter`: end combat turn or skip a reward.
- `R`: restart after win/loss or at any time.
- `Esc`: save the current run and return to the start menu.

The first demo floor includes doors, keys, potion, chest, several monsters, a mini boss, a floor boss, card rewards, and a clear condition at the stairs.

The game auto-saves to `save/last_save.json` after in-game actions and when returning to the start menu.

See [docs/TODO.md](docs/TODO.md) for the current node plan.
