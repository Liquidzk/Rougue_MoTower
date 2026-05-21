# Rougue_MoTower

Bevy prototype for a hybrid Magic Tower map crawler and deck-builder card battle game.

## Run

```bash
cargo run
```

If the shell is not opened inside the desktop session, pass the active display explicitly:

```bash
DISPLAY=:10 cargo run
```

Chinese UI text needs a CJK font. The game tries common Windows, macOS, and Linux font paths automatically. To override it:

```bash
ROUGUE_MOTOWER_FONT=/path/to/chinese-font.ttf cargo run
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

The stage 1 demo currently has 20 floors split into four difficulty bands. Floors include doors, keys, potions, chests, several monsters, mini bosses, floor bosses, card rewards, and a clear condition at the final stairs.

Player combat stats currently include HP, Attack, Defense, Gold, EXP, keys, and deck size. Attack scales damage cards; Defense scales block/healing cards and reduces incoming enemy damage.

The game auto-saves to `save/last_save.json` after in-game actions and when returning to the start menu.

See [docs/TODO.md](docs/TODO.md) for the current node plan.
