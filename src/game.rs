pub const MAP_WIDTH: usize = 12;
pub const MAP_HEIGHT: usize = 9;
pub const STARTING_ENERGY: i32 = 3;
pub const HAND_SIZE: usize = 5;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    Explore,
    Combat,
    Reward,
    Victory,
    GameOver,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Pos {
    pub x: usize,
    pub y: usize,
}

impl Pos {
    fn offset(self, dx: i32, dy: i32) -> Option<Self> {
        let x = self.x as i32 + dx;
        let y = self.y as i32 + dy;

        if x < 0 || y < 0 || x >= MAP_WIDTH as i32 || y >= MAP_HEIGHT as i32 {
            return None;
        }

        Some(Self {
            x: x as usize,
            y: y as usize,
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tile {
    Floor,
    Wall,
    Stairs,
    YellowDoor,
    YellowKey,
    BlueKey,
    SmallPotion,
    Chest,
    Monster(usize),
}

#[derive(Clone, Debug)]
pub struct Player {
    pub pos: Pos,
    pub hp: i32,
    pub max_hp: i32,
    pub attack_bonus: i32,
    pub gold: i32,
    pub yellow_keys: i32,
    pub blue_keys: i32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MonsterRank {
    Normal,
    MiniBoss,
    Boss,
}

#[derive(Clone, Debug)]
pub struct Monster {
    pub name: &'static str,
    pub marker: &'static str,
    pub max_hp: i32,
    pub hp: i32,
    pub attack: i32,
    pub gold: i32,
    pub rank: MonsterRank,
    pub defeated: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CardId {
    Strike,
    Guard,
    HeavySlash,
    Spark,
    ShieldBash,
    FirstAid,
}

#[derive(Clone, Copy, Debug)]
pub struct CardDef {
    pub name: &'static str,
    pub cost: i32,
    pub text: &'static str,
}

pub fn card_def(id: CardId) -> CardDef {
    match id {
        CardId::Strike => CardDef {
            name: "Strike",
            cost: 1,
            text: "Deal 6 damage.",
        },
        CardId::Guard => CardDef {
            name: "Guard",
            cost: 1,
            text: "Gain 6 block.",
        },
        CardId::HeavySlash => CardDef {
            name: "Heavy Slash",
            cost: 2,
            text: "Deal 13 damage.",
        },
        CardId::Spark => CardDef {
            name: "Spark",
            cost: 1,
            text: "Deal 4 damage twice.",
        },
        CardId::ShieldBash => CardDef {
            name: "Shield Bash",
            cost: 1,
            text: "Deal 5 damage. Gain 4 block.",
        },
        CardId::FirstAid => CardDef {
            name: "First Aid",
            cost: 1,
            text: "Heal 5 HP.",
        },
    }
}

#[derive(Clone, Debug)]
pub struct CombatState {
    pub monster_index: usize,
    pub monster_name: &'static str,
    pub monster_rank: MonsterRank,
    pub enemy_hp: i32,
    pub enemy_max_hp: i32,
    pub enemy_attack: i32,
    pub enemy_block: i32,
    pub enemy_gold: i32,
    pub turn: i32,
    pub energy: i32,
    pub player_block: i32,
    pub draw_pile: Vec<CardId>,
    pub discard_pile: Vec<CardId>,
    pub hand: Vec<CardId>,
    pub log: Vec<String>,
}

impl CombatState {
    fn new(monster_index: usize, monster: &Monster, deck: &[CardId]) -> Self {
        let mut combat = Self {
            monster_index,
            monster_name: monster.name,
            monster_rank: monster.rank,
            enemy_hp: monster.hp,
            enemy_max_hp: monster.max_hp,
            enemy_attack: monster.attack,
            enemy_block: 0,
            enemy_gold: monster.gold,
            turn: 1,
            energy: STARTING_ENERGY,
            player_block: 0,
            draw_pile: rotated_deck(deck, monster_index),
            discard_pile: Vec::new(),
            hand: Vec::new(),
            log: vec![format!("{} appears.", monster.name)],
        };
        combat.draw_cards(HAND_SIZE);
        combat
    }

    pub fn enemy_intent(&self) -> String {
        let (damage, block) = self.next_enemy_action();

        match (damage, block) {
            (damage, 0) => format!("Attack {damage}"),
            (0, block) => format!("Block {block}"),
            (damage, block) => format!("Attack {damage}, Block {block}"),
        }
    }

    fn next_enemy_action(&self) -> (i32, i32) {
        match self.monster_rank {
            MonsterRank::Normal => (self.enemy_attack, 0),
            MonsterRank::MiniBoss => {
                if self.turn % 2 == 0 {
                    (self.enemy_attack - 2, 6)
                } else {
                    (self.enemy_attack + 2, 0)
                }
            }
            MonsterRank::Boss => {
                if self.turn % 3 == 0 {
                    (self.enemy_attack + 6, 0)
                } else if self.turn % 2 == 0 {
                    (self.enemy_attack - 2, 8)
                } else {
                    (self.enemy_attack, 0)
                }
            }
        }
    }

    fn draw_cards(&mut self, count: usize) {
        for _ in 0..count {
            if self.draw_pile.is_empty() {
                if self.discard_pile.is_empty() {
                    return;
                }

                self.draw_pile = self.discard_pile.drain(..).rev().collect();
                self.log.push("Discard pile reshuffled.".to_string());
            }

            if let Some(card) = self.draw_pile.pop() {
                self.hand.push(card);
            }
        }
    }

    fn deal_enemy_damage(&mut self, amount: i32) {
        let blocked = amount.min(self.enemy_block);
        self.enemy_block -= blocked;
        self.enemy_hp -= amount - blocked;
    }
}

#[derive(Clone, Debug)]
pub struct RewardState {
    pub monster_name: &'static str,
    pub gold: i32,
    pub offers: Vec<CardId>,
    pub boss_bonus: Option<&'static str>,
}

#[derive(Clone, Debug)]
pub struct Game {
    pub mode: Mode,
    pub floor: i32,
    pub tiles: Vec<Tile>,
    pub player: Player,
    pub monsters: Vec<Monster>,
    pub deck: Vec<CardId>,
    pub combat: Option<CombatState>,
    pub reward: Option<RewardState>,
    pub pending_monster_pos: Option<Pos>,
    pub message: String,
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}

impl Game {
    pub fn new() -> Self {
        let mut game = Self {
            mode: Mode::Explore,
            floor: 1,
            tiles: vec![Tile::Floor; MAP_WIDTH * MAP_HEIGHT],
            player: Player {
                pos: Pos { x: 1, y: 1 },
                hp: 72,
                max_hp: 72,
                attack_bonus: 0,
                gold: 0,
                yellow_keys: 0,
                blue_keys: 0,
            },
            monsters: demo_monsters(),
            deck: starter_deck(),
            combat: None,
            reward: None,
            pending_monster_pos: None,
            message: "Explore the tower. Move with WASD or arrows.".to_string(),
        };

        game.load_demo_floor();
        game
    }

    pub fn restart(&mut self) {
        *self = Self::new();
    }

    pub fn tile_at(&self, pos: Pos) -> Tile {
        self.tiles[pos.y * MAP_WIDTH + pos.x]
    }

    fn set_tile(&mut self, pos: Pos, tile: Tile) {
        self.tiles[pos.y * MAP_WIDTH + pos.x] = tile;
    }

    pub fn try_move(&mut self, dx: i32, dy: i32) {
        if self.mode != Mode::Explore {
            return;
        }

        let Some(target) = self.player.pos.offset(dx, dy) else {
            self.message = "The tower wall blocks the way.".to_string();
            return;
        };

        match self.tile_at(target) {
            Tile::Floor => self.player.pos = target,
            Tile::Wall => {
                self.message = "A solid wall blocks the way.".to_string();
            }
            Tile::Stairs => {
                self.player.pos = target;
                self.mode = Mode::Victory;
                self.message = "Floor 1 cleared. The next floor is a TODO node.".to_string();
            }
            Tile::YellowDoor => {
                if self.player.yellow_keys > 0 {
                    self.player.yellow_keys -= 1;
                    self.set_tile(target, Tile::Floor);
                    self.player.pos = target;
                    self.message = "Yellow door opened.".to_string();
                } else {
                    self.message = "Need a yellow key.".to_string();
                }
            }
            Tile::YellowKey => {
                self.player.yellow_keys += 1;
                self.set_tile(target, Tile::Floor);
                self.player.pos = target;
                self.message = "Picked up a yellow key.".to_string();
            }
            Tile::BlueKey => {
                self.player.blue_keys += 1;
                self.set_tile(target, Tile::Floor);
                self.player.pos = target;
                self.message = "Picked up a blue key. Later floors will spend it.".to_string();
            }
            Tile::SmallPotion => {
                self.player.hp = (self.player.hp + 18).min(self.player.max_hp);
                self.set_tile(target, Tile::Floor);
                self.player.pos = target;
                self.message = "Potion restored 18 HP.".to_string();
            }
            Tile::Chest => {
                self.player.gold += 12;
                self.deck.push(CardId::ShieldBash);
                self.set_tile(target, Tile::Floor);
                self.player.pos = target;
                self.message = "Chest: +12 gold and Shield Bash added to deck.".to_string();
            }
            Tile::Monster(index) => self.start_combat(index, target),
        }
    }

    pub fn play_card(&mut self, hand_index: usize) {
        if self.mode != Mode::Combat {
            return;
        }

        let Some(combat) = self.combat.as_mut() else {
            return;
        };

        if hand_index >= combat.hand.len() {
            combat.log.push("No card in that slot.".to_string());
            return;
        }

        let card = combat.hand[hand_index];
        let def = card_def(card);

        if combat.energy < def.cost {
            combat
                .log
                .push(format!("Not enough energy for {}.", def.name));
            return;
        }

        combat.energy -= def.cost;
        combat.hand.remove(hand_index);
        apply_card(card, &mut self.player, combat);
        combat.discard_pile.push(card);

        if combat.enemy_hp <= 0 {
            self.win_combat();
        }
    }

    pub fn end_turn(&mut self) {
        if self.mode != Mode::Combat {
            return;
        }

        let Some(combat) = self.combat.as_mut() else {
            return;
        };

        combat.discard_pile.append(&mut combat.hand);

        let (damage, block) = combat.next_enemy_action();
        combat.enemy_block += block;

        let blocked = damage.min(combat.player_block);
        let taken = damage - blocked;
        self.player.hp -= taken;
        combat.log.push(format!(
            "{} attacks for {damage}; blocked {blocked}, took {taken}.",
            combat.monster_name
        ));

        if block > 0 {
            combat
                .log
                .push(format!("{} gains {block} block.", combat.monster_name));
        }

        if self.player.hp <= 0 {
            self.player.hp = 0;
            self.mode = Mode::GameOver;
            self.message = "You fell in the tower. Press R to restart.".to_string();
            return;
        }

        combat.turn += 1;
        combat.energy = STARTING_ENERGY;
        combat.player_block = 0;
        combat.draw_cards(HAND_SIZE);
    }

    pub fn choose_reward(&mut self, index: Option<usize>) {
        if self.mode != Mode::Reward {
            return;
        }

        let Some(reward) = self.reward.take() else {
            return;
        };

        if let Some(index) = index {
            if let Some(card) = reward.offers.get(index).copied() {
                self.deck.push(card);
                self.message =
                    format!("Added {} to deck. Continue exploring.", card_def(card).name);
            } else {
                self.message = "Skipped reward. Continue exploring.".to_string();
            }
        } else {
            self.message = "Skipped reward. Continue exploring.".to_string();
        }

        self.mode = Mode::Explore;
    }

    fn start_combat(&mut self, monster_index: usize, pos: Pos) {
        if self.monsters[monster_index].defeated {
            self.set_tile(pos, Tile::Floor);
            self.player.pos = pos;
            return;
        }

        self.pending_monster_pos = Some(pos);
        self.combat = Some(CombatState::new(
            monster_index,
            &self.monsters[monster_index],
            &self.deck,
        ));
        self.mode = Mode::Combat;
        self.message = format!("Battle: {}", self.monsters[monster_index].name);
    }

    fn win_combat(&mut self) {
        let Some(combat) = self.combat.take() else {
            return;
        };

        let index = combat.monster_index;
        let monster_name = combat.monster_name;
        let gold = combat.enemy_gold;
        let rank = combat.monster_rank;

        self.monsters[index].hp = 0;
        self.monsters[index].defeated = true;
        self.player.gold += gold;

        if let Some(pos) = self.pending_monster_pos.take() {
            self.set_tile(pos, Tile::Floor);
            self.player.pos = pos;
        }

        let boss_bonus = match rank {
            MonsterRank::Normal => None,
            MonsterRank::MiniBoss => {
                self.player.max_hp += 6;
                self.player.hp = (self.player.hp + 6).min(self.player.max_hp);
                Some("Mini boss bonus: +6 max HP.")
            }
            MonsterRank::Boss => {
                self.player.attack_bonus += 1;
                Some("Boss bonus: +1 card damage.")
            }
        };

        self.reward = Some(RewardState {
            monster_name,
            gold,
            offers: reward_cards(index, rank),
            boss_bonus,
        });
        self.mode = Mode::Reward;
        self.message = format!("{monster_name} defeated. Choose a card reward.");
    }

    fn load_demo_floor(&mut self) {
        let layout = [
            "############",
            "#P..a..y...#",
            "#.#.###.##.#",
            "#.#k..b.#..#",
            "#.#.###..C##",
            "#...p...#..#",
            "###Y#####..#",
            "#c..d..E..S#",
            "############",
        ];

        for (y, row) in layout.iter().enumerate() {
            for (x, ch) in row.chars().enumerate() {
                let tile = match ch {
                    '#' => Tile::Wall,
                    'P' => {
                        self.player.pos = Pos { x, y };
                        Tile::Floor
                    }
                    'S' => Tile::Stairs,
                    'Y' => Tile::YellowDoor,
                    'y' => Tile::YellowKey,
                    'k' => Tile::BlueKey,
                    'p' => Tile::SmallPotion,
                    'c' => Tile::Chest,
                    'a' => Tile::Monster(0),
                    'b' => Tile::Monster(1),
                    'C' => Tile::Monster(2),
                    'd' => Tile::Monster(3),
                    'E' => Tile::Monster(4),
                    _ => Tile::Floor,
                };
                self.set_tile(Pos { x, y }, tile);
            }
        }
    }
}

fn apply_card(card: CardId, player: &mut Player, combat: &mut CombatState) {
    match card {
        CardId::Strike => {
            let damage = 6 + player.attack_bonus;
            combat.deal_enemy_damage(damage);
            combat.log.push(format!("Strike deals {damage}."));
        }
        CardId::Guard => {
            combat.player_block += 6;
            combat.log.push("Guard adds 6 block.".to_string());
        }
        CardId::HeavySlash => {
            let damage = 13 + player.attack_bonus;
            combat.deal_enemy_damage(damage);
            combat.log.push(format!("Heavy Slash deals {damage}."));
        }
        CardId::Spark => {
            let damage = 4 + player.attack_bonus;
            combat.deal_enemy_damage(damage);
            combat.deal_enemy_damage(damage);
            combat.log.push(format!("Spark deals {damage} twice."));
        }
        CardId::ShieldBash => {
            let damage = 5 + player.attack_bonus;
            combat.deal_enemy_damage(damage);
            combat.player_block += 4;
            combat
                .log
                .push(format!("Shield Bash deals {damage} and adds 4 block."));
        }
        CardId::FirstAid => {
            player.hp = (player.hp + 5).min(player.max_hp);
            combat.log.push("First Aid heals 5 HP.".to_string());
        }
    }
}

fn demo_monsters() -> Vec<Monster> {
    vec![
        Monster {
            name: "Green Slime",
            marker: "S",
            max_hp: 18,
            hp: 18,
            attack: 4,
            gold: 5,
            rank: MonsterRank::Normal,
            defeated: false,
        },
        Monster {
            name: "Cave Bat",
            marker: "B",
            max_hp: 24,
            hp: 24,
            attack: 6,
            gold: 7,
            rank: MonsterRank::Normal,
            defeated: false,
        },
        Monster {
            name: "Bone Guard",
            marker: "G",
            max_hp: 34,
            hp: 34,
            attack: 8,
            gold: 10,
            rank: MonsterRank::Normal,
            defeated: false,
        },
        Monster {
            name: "Iron Captain",
            marker: "M",
            max_hp: 54,
            hp: 54,
            attack: 10,
            gold: 18,
            rank: MonsterRank::MiniBoss,
            defeated: false,
        },
        Monster {
            name: "Floor Guardian",
            marker: "F",
            max_hp: 82,
            hp: 82,
            attack: 13,
            gold: 30,
            rank: MonsterRank::Boss,
            defeated: false,
        },
    ]
}

fn starter_deck() -> Vec<CardId> {
    vec![
        CardId::Strike,
        CardId::Strike,
        CardId::Strike,
        CardId::Strike,
        CardId::Guard,
        CardId::Guard,
        CardId::Guard,
        CardId::Spark,
        CardId::HeavySlash,
        CardId::FirstAid,
    ]
}

fn rotated_deck(deck: &[CardId], seed: usize) -> Vec<CardId> {
    if deck.is_empty() {
        return Vec::new();
    }

    let offset = seed % deck.len();
    let mut cards = deck.to_vec();
    cards.rotate_left(offset);
    cards.reverse();
    cards
}

fn reward_cards(monster_index: usize, rank: MonsterRank) -> Vec<CardId> {
    match rank {
        MonsterRank::Normal => match monster_index % 3 {
            0 => vec![CardId::Strike, CardId::Guard, CardId::ShieldBash],
            1 => vec![CardId::Spark, CardId::Guard, CardId::FirstAid],
            _ => vec![CardId::HeavySlash, CardId::ShieldBash, CardId::Strike],
        },
        MonsterRank::MiniBoss => vec![CardId::HeavySlash, CardId::Spark, CardId::FirstAid],
        MonsterRank::Boss => vec![CardId::HeavySlash, CardId::ShieldBash, CardId::Spark],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn yellow_door_requires_key() {
        let mut game = Game::new();
        game.player.pos = Pos { x: 3, y: 5 };
        game.try_move(0, 1);
        assert_eq!(game.player.pos, Pos { x: 3, y: 5 });
        assert_eq!(game.tile_at(Pos { x: 3, y: 6 }), Tile::YellowDoor);

        game.player.yellow_keys = 1;
        game.try_move(0, 1);
        assert_eq!(game.player.pos, Pos { x: 3, y: 6 });
        assert_eq!(game.tile_at(Pos { x: 3, y: 6 }), Tile::Floor);
        assert_eq!(game.player.yellow_keys, 0);
    }

    #[test]
    fn strike_can_finish_combat_and_open_tile() {
        let mut game = Game::new();
        let monster_pos = Pos { x: 4, y: 1 };
        game.start_combat(0, monster_pos);
        game.combat.as_mut().unwrap().enemy_hp = 6;

        let slot = game
            .combat
            .as_ref()
            .unwrap()
            .hand
            .iter()
            .position(|card| *card == CardId::Strike)
            .unwrap();

        game.play_card(slot);

        assert_eq!(game.mode, Mode::Reward);
        assert_eq!(game.tile_at(monster_pos), Tile::Floor);
        assert_eq!(game.player.pos, monster_pos);
        assert!(game.monsters[0].defeated);
    }
}
