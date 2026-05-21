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
- Mouse: click any reachable map tile to move there instantly; blocked paths do nothing. Click a reachable monster to enter combat, click cards to play or pick rewards, and click on-screen buttons for menu/end turn/skip.
- `1`-`5`: play cards in battle.
- `Space` / `Enter`: end combat turn or skip a reward.
- `R`: restart after win/loss or at any time.
- `Esc`: save the current run and return to the start menu.

The stage 1 demo currently has 20 floors split into four difficulty bands. Floors include doors, keys, potions, chests, several monsters, mini bosses, floor bosses, card rewards, and a clear condition at the final stairs.

Player combat stats currently include HP, Attack, Defense, Gold, EXP, keys, and deck size. Attack scales damage cards; Defense scales block/healing cards and reduces incoming enemy damage.

Warrior cards now use color-coded rarities: common white, advanced blue, rare purple, legendary gold, and special red. Normal monsters, mini bosses, bosses, and shops each draw from different reward pools.

Shops appear on floors 4, 11, and 17 and spend gold for card rewards. Sages appear on floors 5, 12, and 18 and spend EXP for Attack, Defense, or healing.

The game auto-saves to `save/last_save.json` after in-game actions and when returning to the start menu.

See [docs/TODO.md](docs/TODO.md) for the current node plan.
