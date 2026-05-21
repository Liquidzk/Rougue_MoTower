use std::collections::VecDeque;

use serde::{Deserialize, Serialize};

use crate::localization::{self, Language};

pub const MAP_WIDTH: usize = 12;
pub const MAP_HEIGHT: usize = 9;
pub const STARTING_ENERGY: i32 = 3;
pub const HAND_SIZE: usize = 5;
pub const FINAL_FLOOR: i32 = 20;
const MONSTER_SLOTS_PER_FLOOR: usize = 5;
const STARTING_ATTACK: i32 = 4;
const STARTING_DEFENSE: i32 = 2;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Mode {
    Explore,
    Combat,
    Reward,
    Victory,
    GameOver,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
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

fn default_attack() -> i32 {
    STARTING_ATTACK
}

fn default_defense() -> i32 {
    STARTING_DEFENSE
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Tile {
    Floor,
    Wall,
    Stairs,
    YellowDoor,
    BlueDoor,
    YellowKey,
    BlueKey,
    SmallPotion,
    Chest,
    Shop,
    Sage,
    Monster(usize),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Player {
    pub pos: Pos,
    pub hp: i32,
    pub max_hp: i32,
    #[serde(default = "default_attack")]
    pub attack: i32,
    #[serde(default = "default_defense")]
    pub defense: i32,
    #[serde(default)]
    pub experience: i32,
    pub gold: i32,
    pub yellow_keys: i32,
    pub blue_keys: i32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum MonsterRank {
    Normal,
    MiniBoss,
    Boss,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum MonsterId {
    GreenSlime,
    CaveBat,
    BoneGuard,
    IronCaptain,
    FloorGuardian,
    RedSlime,
    VampireBat,
    StoneGuard,
    RuneCaptain,
    TowerWarden,
    DarkSlime,
    Warlock,
    IronGolem,
    AbyssKnight,
    DemonLord,
    FlameSlime,
    Dragonling,
    GoldenGuard,
    ChaosKnight,
    AncientDragon,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Monster {
    pub kind: MonsterId,
    pub max_hp: i32,
    pub hp: i32,
    pub attack: i32,
    pub gold: i32,
    #[serde(default)]
    pub experience: i32,
    pub rank: MonsterRank,
    pub defeated: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum CardId {
    Strike,
    Guard,
    HeavySlash,
    Spark,
    ShieldBash,
    FirstAid,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BossBonus {
    MiniBossMaxHp,
    BossAttackBonus,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum SageBoon {
    Attack(i32),
    Defense(i32),
    Heal(i32),
}

pub fn card_cost(id: CardId) -> i32 {
    match id {
        CardId::Strike => 1,
        CardId::Guard => 1,
        CardId::HeavySlash => 2,
        CardId::Spark => 1,
        CardId::ShieldBash => 1,
        CardId::FirstAid => 1,
    }
}

pub fn monster_marker(kind: MonsterId) -> &'static str {
    match kind {
        MonsterId::GreenSlime => "S",
        MonsterId::CaveBat => "B",
        MonsterId::BoneGuard => "G",
        MonsterId::IronCaptain => "M",
        MonsterId::FloorGuardian => "F",
        MonsterId::RedSlime => "R",
        MonsterId::VampireBat => "V",
        MonsterId::StoneGuard => "T",
        MonsterId::RuneCaptain => "C",
        MonsterId::TowerWarden => "W",
        MonsterId::DarkSlime => "D",
        MonsterId::Warlock => "L",
        MonsterId::IronGolem => "O",
        MonsterId::AbyssKnight => "A",
        MonsterId::DemonLord => "N",
        MonsterId::FlameSlime => "F",
        MonsterId::Dragonling => "Q",
        MonsterId::GoldenGuard => "G",
        MonsterId::ChaosKnight => "H",
        MonsterId::AncientDragon => "X",
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CombatState {
    pub monster_index: usize,
    pub monster_kind: MonsterId,
    pub monster_rank: MonsterRank,
    pub enemy_hp: i32,
    pub enemy_max_hp: i32,
    pub enemy_attack: i32,
    pub enemy_block: i32,
    pub enemy_gold: i32,
    #[serde(default)]
    pub enemy_experience: i32,
    pub turn: i32,
    pub energy: i32,
    pub player_block: i32,
    pub draw_pile: Vec<CardId>,
    pub discard_pile: Vec<CardId>,
    pub hand: Vec<CardId>,
    pub log: Vec<String>,
}

impl CombatState {
    fn new(monster_index: usize, monster: &Monster, deck: &[CardId], language: Language) -> Self {
        let mut combat = Self {
            monster_index,
            monster_kind: monster.kind,
            monster_rank: monster.rank,
            enemy_hp: monster.hp,
            enemy_max_hp: monster.max_hp,
            enemy_attack: monster.attack,
            enemy_block: 0,
            enemy_gold: monster.gold,
            enemy_experience: monster.experience,
            turn: 1,
            energy: STARTING_ENERGY,
            player_block: 0,
            draw_pile: rotated_deck(deck, monster_index),
            discard_pile: Vec::new(),
            hand: Vec::new(),
            log: vec![localization::log_monster_appears(language, monster.kind)],
        };
        combat.draw_cards(HAND_SIZE, language);
        combat
    }

    pub fn enemy_intent(&self, language: Language) -> String {
        let (damage, block) = self.next_enemy_action();
        localization::enemy_intent(language, damage, block)
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

    fn draw_cards(&mut self, count: usize, language: Language) {
        for _ in 0..count {
            if self.draw_pile.is_empty() {
                if self.discard_pile.is_empty() {
                    return;
                }

                self.draw_pile = self.discard_pile.drain(..).rev().collect();
                self.log
                    .push(localization::log_discard_reshuffled(language));
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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RewardState {
    pub monster_kind: MonsterId,
    pub gold: i32,
    #[serde(default)]
    pub experience: i32,
    pub offers: Vec<CardId>,
    pub boss_bonus: Option<BossBonus>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
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
        Self::new_with_language(Language::default())
    }

    pub fn new_with_language(language: Language) -> Self {
        let mut game = Self {
            mode: Mode::Explore,
            floor: 1,
            tiles: vec![Tile::Floor; MAP_WIDTH * MAP_HEIGHT],
            player: Player {
                pos: Pos { x: 1, y: 1 },
                hp: 72,
                max_hp: 72,
                attack: STARTING_ATTACK,
                defense: STARTING_DEFENSE,
                experience: 0,
                gold: 0,
                yellow_keys: 0,
                blue_keys: 0,
            },
            monsters: demo_monsters(),
            deck: starter_deck(),
            combat: None,
            reward: None,
            pending_monster_pos: None,
            message: localization::initial_message(language),
        };

        game.load_floor(1);
        game
    }

    pub fn restart(&mut self, language: Language) {
        *self = Self::new_with_language(language);
    }

    pub fn set_message(&mut self, message: String) {
        self.message = message;
    }

    pub fn tile_at(&self, pos: Pos) -> Tile {
        self.tiles[pos.y * MAP_WIDTH + pos.x]
    }

    fn set_tile(&mut self, pos: Pos, tile: Tile) {
        self.tiles[pos.y * MAP_WIDTH + pos.x] = tile;
    }

    pub fn try_move(&mut self, dx: i32, dy: i32, language: Language) {
        if self.mode != Mode::Explore {
            return;
        }

        let Some(target) = self.player.pos.offset(dx, dy) else {
            self.message = localization::message_wall_bounds(language);
            return;
        };

        self.enter_tile(target, language);
    }

    pub fn try_click_tile(&mut self, target: Pos, language: Language) -> bool {
        if self.mode != Mode::Explore || target == self.player.pos {
            return false;
        }

        match self.tile_at(target) {
            Tile::Monster(index) => {
                let Some(stand_pos) = self.reachable_monster_stand_pos(target) else {
                    return false;
                };

                self.player.pos = stand_pos;
                self.start_combat(index, target, language);
                true
            }
            tile => {
                if !self.is_click_destination(tile) || !self.can_reach_tile(target) {
                    return false;
                }

                self.enter_tile(target, language)
            }
        }
    }

    fn enter_tile(&mut self, target: Pos, language: Language) -> bool {
        match self.tile_at(target) {
            Tile::Floor => {
                self.player.pos = target;
                true
            }
            Tile::Wall => {
                self.message = localization::message_solid_wall(language);
                false
            }
            Tile::Stairs => {
                if self.floor >= FINAL_FLOOR {
                    self.player.pos = target;
                    self.mode = Mode::Victory;
                    self.message = localization::message_stage_clear(language);
                } else {
                    self.advance_floor(language);
                }
                true
            }
            Tile::YellowDoor => {
                if self.player.yellow_keys > 0 {
                    self.player.yellow_keys -= 1;
                    self.set_tile(target, Tile::Floor);
                    self.player.pos = target;
                    self.message = localization::message_yellow_door_opened(language);
                    true
                } else {
                    self.message = localization::message_need_yellow_key(language);
                    false
                }
            }
            Tile::BlueDoor => {
                if self.player.blue_keys > 0 {
                    self.player.blue_keys -= 1;
                    self.set_tile(target, Tile::Floor);
                    self.player.pos = target;
                    self.message = localization::message_blue_door_opened(language);
                    true
                } else {
                    self.message = localization::message_need_blue_key(language);
                    false
                }
            }
            Tile::YellowKey => {
                self.player.yellow_keys += 1;
                self.set_tile(target, Tile::Floor);
                self.player.pos = target;
                self.message = localization::message_yellow_key(language);
                true
            }
            Tile::BlueKey => {
                self.player.blue_keys += 1;
                self.set_tile(target, Tile::Floor);
                self.player.pos = target;
                self.message = localization::message_blue_key(language);
                true
            }
            Tile::SmallPotion => {
                self.player.hp = (self.player.hp + 18).min(self.player.max_hp);
                self.set_tile(target, Tile::Floor);
                self.player.pos = target;
                self.message = localization::message_potion(language);
                true
            }
            Tile::Chest => {
                self.player.gold += 12;
                self.deck.push(CardId::ShieldBash);
                self.set_tile(target, Tile::Floor);
                self.player.pos = target;
                self.message = localization::message_chest(language);
                true
            }
            Tile::Shop => {
                self.player.pos = target;
                self.visit_shop(language);
                true
            }
            Tile::Sage => {
                self.player.pos = target;
                self.visit_sage(language);
                true
            }
            Tile::Monster(index) => {
                self.start_combat(index, target, language);
                true
            }
        }
    }

    fn is_click_destination(&self, tile: Tile) -> bool {
        match tile {
            Tile::Floor
            | Tile::Stairs
            | Tile::YellowKey
            | Tile::BlueKey
            | Tile::SmallPotion
            | Tile::Chest
            | Tile::Shop
            | Tile::Sage => true,
            Tile::YellowDoor => self.player.yellow_keys > 0,
            Tile::BlueDoor => self.player.blue_keys > 0,
            Tile::Wall | Tile::Monster(_) => false,
        }
    }

    fn can_reach_tile(&self, target: Pos) -> bool {
        self.find_reachable_pos(
            |pos| pos == target,
            |pos| pos == target || self.is_clear_path_tile(self.tile_at(pos)),
        )
        .is_some()
    }

    fn reachable_monster_stand_pos(&self, monster_pos: Pos) -> Option<Pos> {
        self.find_reachable_pos(
            |pos| are_adjacent(pos, monster_pos),
            |pos| self.is_clear_path_tile(self.tile_at(pos)),
        )
    }

    fn find_reachable_pos(
        &self,
        is_goal: impl Fn(Pos) -> bool,
        can_enter: impl Fn(Pos) -> bool,
    ) -> Option<Pos> {
        let mut visited = [false; MAP_WIDTH * MAP_HEIGHT];
        let mut queue = VecDeque::new();

        visited[self.player.pos.y * MAP_WIDTH + self.player.pos.x] = true;
        queue.push_back(self.player.pos);

        while let Some(pos) = queue.pop_front() {
            if is_goal(pos) {
                return Some(pos);
            }

            for next in neighbors(pos) {
                let index = next.y * MAP_WIDTH + next.x;
                if visited[index] || !can_enter(next) {
                    continue;
                }

                visited[index] = true;
                queue.push_back(next);
            }
        }

        None
    }

    fn is_clear_path_tile(&self, tile: Tile) -> bool {
        matches!(tile, Tile::Floor)
    }

    fn visit_shop(&mut self, language: Language) {
        let cost = shop_cost(self.floor);
        if self.player.gold < cost {
            self.message = localization::message_shop_need_gold(language, cost);
            return;
        }

        let card = shop_card(self.floor, self.deck.len());
        self.player.gold -= cost;
        self.deck.push(card);
        self.message = localization::message_shop_card(language, cost, card);
    }

    fn visit_sage(&mut self, language: Language) {
        let cost = sage_cost(self.floor);
        if self.player.experience < cost {
            self.message = localization::message_sage_need_experience(language, cost);
            return;
        }

        self.player.experience -= cost;
        match sage_boon(self.floor) {
            SageBoon::Attack(amount) => {
                self.player.attack += amount;
                self.message = localization::message_sage_attack(language, cost, amount);
            }
            SageBoon::Defense(amount) => {
                self.player.defense += amount;
                self.message = localization::message_sage_defense(language, cost, amount);
            }
            SageBoon::Heal(amount) => {
                self.player.hp = (self.player.hp + amount).min(self.player.max_hp);
                self.message = localization::message_sage_heal(language, cost, amount);
            }
        }
    }

    pub fn play_card(&mut self, hand_index: usize, language: Language) {
        if self.mode != Mode::Combat {
            return;
        }

        let Some(combat) = self.combat.as_mut() else {
            return;
        };

        if hand_index >= combat.hand.len() {
            combat.log.push(localization::message_no_card(language));
            return;
        }

        let card = combat.hand[hand_index];
        let cost = card_cost(card);

        if combat.energy < cost {
            combat
                .log
                .push(localization::message_not_enough_energy(language, card));
            return;
        }

        combat.energy -= cost;
        combat.hand.remove(hand_index);
        apply_card(card, &mut self.player, combat, language);
        combat.discard_pile.push(card);

        if combat.enemy_hp <= 0 {
            self.win_combat(language);
        }
    }

    pub fn end_turn(&mut self, language: Language) {
        if self.mode != Mode::Combat {
            return;
        }

        let Some(combat) = self.combat.as_mut() else {
            return;
        };

        combat.discard_pile.append(&mut combat.hand);

        let (damage, block) = combat.next_enemy_action();
        combat.enemy_block += block;

        let card_blocked = damage.min(combat.player_block);
        let defense_blocked = (damage - card_blocked).min(self.player.defense);
        let blocked = card_blocked + defense_blocked;
        let taken = damage - blocked;
        self.player.hp -= taken;
        combat.log.push(localization::log_enemy_attack(
            language,
            combat.monster_kind,
            damage,
            blocked,
            taken,
        ));

        if block > 0 {
            combat.log.push(localization::log_enemy_block(
                language,
                combat.monster_kind,
                block,
            ));
        }

        if self.player.hp <= 0 {
            self.player.hp = 0;
            self.mode = Mode::GameOver;
            self.message = localization::message_fell(language);
            return;
        }

        combat.turn += 1;
        combat.energy = STARTING_ENERGY;
        combat.player_block = 0;
        combat.draw_cards(HAND_SIZE, language);
    }

    pub fn choose_reward(&mut self, index: Option<usize>, language: Language) {
        if self.mode != Mode::Reward {
            return;
        }

        let Some(reward) = self.reward.take() else {
            return;
        };

        if let Some(index) = index {
            if let Some(card) = reward.offers.get(index).copied() {
                self.deck.push(card);
                self.message = localization::message_added_card(language, card);
            } else {
                self.message = localization::message_skipped_reward(language);
            }
        } else {
            self.message = localization::message_skipped_reward(language);
        }

        self.mode = Mode::Explore;
    }

    fn start_combat(&mut self, monster_index: usize, pos: Pos, language: Language) {
        if self.monsters[monster_index].defeated {
            self.set_tile(pos, Tile::Floor);
            self.player.pos = pos;
            return;
        }

        let monster_kind = self.monsters[monster_index].kind;
        self.pending_monster_pos = Some(pos);
        self.combat = Some(CombatState::new(
            monster_index,
            &self.monsters[monster_index],
            &self.deck,
            language,
        ));
        self.mode = Mode::Combat;
        self.message = localization::message_battle(language, monster_kind);
    }

    fn win_combat(&mut self, language: Language) {
        let Some(combat) = self.combat.take() else {
            return;
        };

        let index = combat.monster_index;
        let monster_kind = combat.monster_kind;
        let gold = combat.enemy_gold;
        let experience = combat.enemy_experience;
        let rank = combat.monster_rank;

        self.monsters[index].hp = 0;
        self.monsters[index].defeated = true;
        self.player.gold += gold;
        self.player.experience += experience;

        if let Some(pos) = self.pending_monster_pos.take() {
            self.set_tile(pos, Tile::Floor);
            self.player.pos = pos;
        }

        let boss_bonus = match rank {
            MonsterRank::Normal => None,
            MonsterRank::MiniBoss => {
                self.player.max_hp += 6;
                self.player.hp = (self.player.hp + 6).min(self.player.max_hp);
                Some(BossBonus::MiniBossMaxHp)
            }
            MonsterRank::Boss => {
                self.player.attack += 1;
                Some(BossBonus::BossAttackBonus)
            }
        };

        self.reward = Some(RewardState {
            monster_kind,
            gold,
            experience,
            offers: reward_cards(index, rank),
            boss_bonus,
        });
        self.mode = Mode::Reward;
        self.message = localization::message_defeated_choose_reward(language, monster_kind);
    }

    fn advance_floor(&mut self, language: Language) {
        self.floor += 1;
        self.mode = Mode::Explore;
        self.combat = None;
        self.reward = None;
        self.pending_monster_pos = None;
        self.load_floor(self.floor);
        self.message = localization::message_enter_floor(language, self.floor);
    }

    fn load_floor(&mut self, floor: i32) {
        self.tiles.fill(Tile::Floor);

        for (y, row) in floor_layout(floor).iter().enumerate() {
            for (x, ch) in row.chars().enumerate() {
                let tile = match ch {
                    '#' => Tile::Wall,
                    'P' => {
                        self.player.pos = Pos { x, y };
                        Tile::Floor
                    }
                    'S' => Tile::Stairs,
                    'Y' => Tile::YellowDoor,
                    'U' => Tile::BlueDoor,
                    'y' => Tile::YellowKey,
                    'k' => Tile::BlueKey,
                    'p' => Tile::SmallPotion,
                    'c' => Tile::Chest,
                    's' => Tile::Shop,
                    'o' => Tile::Sage,
                    'a' => Tile::Monster(monster_index_for_floor(floor, 0)),
                    'b' => Tile::Monster(monster_index_for_floor(floor, 1)),
                    'C' => Tile::Monster(monster_index_for_floor(floor, 2)),
                    'd' => Tile::Monster(monster_index_for_floor(floor, 3)),
                    'E' => Tile::Monster(monster_index_for_floor(floor, 4)),
                    _ => Tile::Floor,
                };
                self.set_tile(Pos { x, y }, tile);
            }
        }
    }
}

fn floor_layout(floor: i32) -> [&'static str; MAP_HEIGHT] {
    match floor {
        1 => [
            "############",
            "#P..a..y...#",
            "#.#.###.##.#",
            "#.#k..b.#..#",
            "#.#.###..C##",
            "#...p...#..#",
            "###Y#####..#",
            "#c..d..E..S#",
            "############",
        ],
        2 => [
            "############",
            "#P..a..y...#",
            "#.##.##.##.#",
            "#..b..p.#..#",
            "###Y###.#C##",
            "#k..c...#..#",
            "#.####U##..#",
            "#..d...E..S#",
            "############",
        ],
        3 => [
            "############",
            "#P.a...p..S#",
            "#.###.####.#",
            "#...b..y...#",
            "###.###Y####",
            "#k..C......#",
            "#.#####.##.#",
            "#c..d..E...#",
            "############",
        ],
        4 => [
            "############",
            "#P..a..s...#",
            "#.##.####..#",
            "#..b...y#..#",
            "####Y##.#C##",
            "#p.....k#..#",
            "#.#######..#",
            "#..d..E...S#",
            "############",
        ],
        5 => [
            "############",
            "#P..a..y...#",
            "#.##Y#####.#",
            "#..b..p...C#",
            "####.#######",
            "#k..o......#",
            "#.#######..#",
            "#..d..E..S.#",
            "############",
        ],
        6 => [
            "############",
            "#P.a....y..#",
            "#.####.###.#",
            "#..b..p#...#",
            "##.###.#C###",
            "#k...c.#...#",
            "#.##Y###.U##",
            "#..d..E..S.#",
            "############",
        ],
        7 => [
            "############",
            "#P..a..p...#",
            "#.#######..#",
            "#..b...y#C.#",
            "####Y##.#..#",
            "#k..c...#..#",
            "#.####U##..#",
            "#..d..E..S.#",
            "############",
        ],
        8 => [
            "############",
            "#P.a..y....#",
            "#.##.#######",
            "#..b..p...C#",
            "####.####Y##",
            "#k..c......#",
            "#.####U###.#",
            "#..d..E..S.#",
            "############",
        ],
        9 => [
            "############",
            "#P..a...c..#",
            "#.######.#.#",
            "#..b..y#.#C#",
            "##.##Y##.#.#",
            "#k..p....#.#",
            "#.#####U##.#",
            "#..d..E..S.#",
            "############",
        ],
        10 => [
            "############",
            "#P.a..y...C#",
            "#.##Y#######",
            "#..b..p....#",
            "####.####..#",
            "#k..c...#..#",
            "#.####U##..#",
            "#..d..E..S.#",
            "############",
        ],
        11 => [
            "############",
            "#P..a..y...#",
            "#.###.####.#",
            "#..b..p.#C.#",
            "##.##Y#.#..#",
            "#k..s...#..#",
            "#.####U##..#",
            "#..d..E..S.#",
            "############",
        ],
        12 => [
            "############",
            "#P.a...p..C#",
            "#.#######..#",
            "#..b..y.#..#",
            "####Y##.#..#",
            "#k..o...#..#",
            "#.####U##..#",
            "#..d..E..S.#",
            "############",
        ],
        13 => [
            "############",
            "#P..a..c...#",
            "#.##.#####.#",
            "#..b..y..C.#",
            "####Y#######",
            "#k..p......#",
            "#.####U###.#",
            "#..d..E..S.#",
            "############",
        ],
        14 => [
            "############",
            "#P.a..y....#",
            "#.####.###.#",
            "#..b..p#C..#",
            "##.##Y##...#",
            "#k..c...#..#",
            "#.####U##..#",
            "#..d..E..S.#",
            "############",
        ],
        15 => [
            "############",
            "#P..a..y..C#",
            "#.##Y#######",
            "#..b..p....#",
            "####.####..#",
            "#k..c...#..#",
            "#.####U##..#",
            "#..d..E..S.#",
            "############",
        ],
        16 => [
            "############",
            "#P.a..y..C.#",
            "#.#######..#",
            "#..b..p.#..#",
            "####Y##.#..#",
            "#k..c...#..#",
            "#.####U##..#",
            "#..d..E..S.#",
            "############",
        ],
        17 => [
            "############",
            "#P..a..p...#",
            "#.##.#####.#",
            "#..b..y#C..#",
            "##.##Y##...#",
            "#k..s...#..#",
            "#.####U##..#",
            "#..d..E..S.#",
            "############",
        ],
        18 => [
            "############",
            "#P.a...c..C#",
            "#.#######..#",
            "#..b..y.#..#",
            "####Y##.#..#",
            "#k..o...#..#",
            "#.####U##..#",
            "#..d..E..S.#",
            "############",
        ],
        19 => [
            "############",
            "#P..a..y...#",
            "#.######.#.#",
            "#..b..p#.#C#",
            "##.##Y##.#.#",
            "#k..c....#.#",
            "#.####U###.#",
            "#..d..E..S.#",
            "############",
        ],
        20 => [
            "############",
            "#P.a..y..C.#",
            "#.##Y#######",
            "#..b..p....#",
            "####.####..#",
            "#k..c...#..#",
            "#.####U##..#",
            "#..d..E..S.#",
            "############",
        ],
        _ => [
            "############",
            "#P........S#",
            "#..........#",
            "#..........#",
            "#..........#",
            "#..........#",
            "#..........#",
            "#..........#",
            "############",
        ],
    }
}

fn monster_index_for_floor(floor: i32, slot: usize) -> usize {
    ((floor - 1).max(0) as usize * MONSTER_SLOTS_PER_FLOOR) + slot
}

fn shop_cost(floor: i32) -> i32 {
    match floor {
        1..=5 => 18,
        6..=12 => 42,
        _ => 78,
    }
}

fn shop_card(floor: i32, deck_len: usize) -> CardId {
    let offers = match floor {
        1..=5 => [CardId::ShieldBash, CardId::Spark, CardId::HeavySlash],
        6..=12 => [CardId::HeavySlash, CardId::Spark, CardId::FirstAid],
        _ => [CardId::HeavySlash, CardId::ShieldBash, CardId::FirstAid],
    };
    offers[deck_len % offers.len()]
}

fn sage_cost(floor: i32) -> i32 {
    match floor {
        1..=5 => 16,
        6..=12 => 42,
        _ => 78,
    }
}

fn sage_boon(floor: i32) -> SageBoon {
    match floor {
        1..=5 => SageBoon::Attack(1),
        6..=12 => SageBoon::Defense(1),
        _ => SageBoon::Heal(32),
    }
}

fn neighbors(pos: Pos) -> impl Iterator<Item = Pos> {
    [(0, -1), (0, 1), (-1, 0), (1, 0)]
        .into_iter()
        .filter_map(move |(dx, dy)| pos.offset(dx, dy))
}

fn are_adjacent(a: Pos, b: Pos) -> bool {
    a.x.abs_diff(b.x) + a.y.abs_diff(b.y) == 1
}

fn apply_card(card: CardId, player: &mut Player, combat: &mut CombatState, language: Language) {
    match card {
        CardId::Strike => {
            let damage = 5 + player.attack;
            combat.deal_enemy_damage(damage);
            combat
                .log
                .push(localization::log_card(language, card, damage));
        }
        CardId::Guard => {
            let block = 5 + player.defense;
            combat.player_block += block;
            combat
                .log
                .push(localization::log_card(language, card, block));
        }
        CardId::HeavySlash => {
            let damage = 10 + player.attack * 2;
            combat.deal_enemy_damage(damage);
            combat
                .log
                .push(localization::log_card(language, card, damage));
        }
        CardId::Spark => {
            let damage = 3 + player.attack / 2;
            combat.deal_enemy_damage(damage);
            combat.deal_enemy_damage(damage);
            combat
                .log
                .push(localization::log_card(language, card, damage));
        }
        CardId::ShieldBash => {
            let damage = 4 + player.attack;
            let block = 3 + player.defense;
            combat.deal_enemy_damage(damage);
            combat.player_block += block;
            combat
                .log
                .push(localization::log_shield_bash(language, damage, block));
        }
        CardId::FirstAid => {
            let healing = 5 + player.defense / 2;
            player.hp = (player.hp + healing).min(player.max_hp);
            combat
                .log
                .push(localization::log_card(language, card, healing));
        }
    }
}

fn demo_monsters() -> Vec<Monster> {
    (1..=FINAL_FLOOR)
        .flat_map(|floor| {
            (0..MONSTER_SLOTS_PER_FLOOR).map(move |slot| monster_for_floor_slot(floor, slot))
        })
        .collect()
}

fn monster_for_floor_slot(floor: i32, slot: usize) -> Monster {
    let kind = monster_kind_for_floor_slot(floor, slot);
    let rank = if slot == 4 {
        if floor % 5 == 0 || floor >= 11 {
            MonsterRank::Boss
        } else {
            MonsterRank::MiniBoss
        }
    } else if slot == 3 {
        MonsterRank::MiniBoss
    } else {
        MonsterRank::Normal
    };

    let base_hp = [18, 24, 34, 54, 82][slot];
    let base_attack = [4, 6, 8, 10, 13][slot];
    let base_gold = [5, 7, 10, 18, 30][slot];
    let base_experience = [3, 4, 6, 10, 16][slot];
    let tier = ((floor - 1) / 5).max(0);
    let floor_step = floor - 1;
    let rank_hp = match rank {
        MonsterRank::Normal => 0,
        MonsterRank::MiniBoss => 14 + tier * 8,
        MonsterRank::Boss => 34 + tier * 18,
    };
    let rank_attack = match rank {
        MonsterRank::Normal => 0,
        MonsterRank::MiniBoss => 2 + tier,
        MonsterRank::Boss => 4 + tier * 2,
    };

    let max_hp = base_hp + floor_step * 9 + tier * 24 + rank_hp;
    let attack = base_attack + floor_step * 2 + tier * 3 + rank_attack;
    let gold = base_gold + floor * 2 + slot as i32 * 2 + tier * 8;
    let experience = base_experience + floor + tier * 4 + slot as i32;

    Monster {
        kind,
        max_hp,
        hp: max_hp,
        attack,
        gold,
        experience,
        rank,
        defeated: false,
    }
}

fn monster_kind_for_floor_slot(floor: i32, slot: usize) -> MonsterId {
    let tier = ((floor - 1) / 5).clamp(0, 3) as usize;
    const KINDS: [[MonsterId; MONSTER_SLOTS_PER_FLOOR]; 4] = [
        [
            MonsterId::GreenSlime,
            MonsterId::CaveBat,
            MonsterId::BoneGuard,
            MonsterId::IronCaptain,
            MonsterId::FloorGuardian,
        ],
        [
            MonsterId::RedSlime,
            MonsterId::VampireBat,
            MonsterId::StoneGuard,
            MonsterId::RuneCaptain,
            MonsterId::TowerWarden,
        ],
        [
            MonsterId::DarkSlime,
            MonsterId::Warlock,
            MonsterId::IronGolem,
            MonsterId::AbyssKnight,
            MonsterId::DemonLord,
        ],
        [
            MonsterId::FlameSlime,
            MonsterId::Dragonling,
            MonsterId::GoldenGuard,
            MonsterId::ChaosKnight,
            MonsterId::AncientDragon,
        ],
    ];

    KINDS[tier][slot]
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
        let mut game = Game::new_with_language(Language::English);
        game.player.pos = Pos { x: 3, y: 5 };
        game.try_move(0, 1, Language::English);
        assert_eq!(game.player.pos, Pos { x: 3, y: 5 });
        assert_eq!(game.tile_at(Pos { x: 3, y: 6 }), Tile::YellowDoor);

        game.player.yellow_keys = 1;
        game.try_move(0, 1, Language::English);
        assert_eq!(game.player.pos, Pos { x: 3, y: 6 });
        assert_eq!(game.tile_at(Pos { x: 3, y: 6 }), Tile::Floor);
        assert_eq!(game.player.yellow_keys, 0);
    }

    #[test]
    fn strike_can_finish_combat_and_open_tile() {
        let mut game = Game::new_with_language(Language::English);
        let monster_pos = Pos { x: 4, y: 1 };
        game.start_combat(0, monster_pos, Language::English);
        game.combat.as_mut().unwrap().enemy_hp = 6;

        let slot = game
            .combat
            .as_ref()
            .unwrap()
            .hand
            .iter()
            .position(|card| *card == CardId::Strike)
            .unwrap();

        game.play_card(slot, Language::English);

        assert_eq!(game.mode, Mode::Reward);
        assert_eq!(game.tile_at(monster_pos), Tile::Floor);
        assert_eq!(game.player.pos, monster_pos);
        assert!(game.monsters[0].defeated);
    }

    #[test]
    fn attack_attribute_scales_card_damage() {
        let mut game = Game::new_with_language(Language::English);
        game.start_combat(0, Pos { x: 4, y: 1 }, Language::English);
        game.player.attack = 7;
        let combat = game.combat.as_mut().unwrap();
        combat.enemy_hp = 20;
        combat.hand = vec![CardId::Strike];
        combat.energy = STARTING_ENERGY;

        game.play_card(0, Language::English);

        assert_eq!(game.combat.as_ref().unwrap().enemy_hp, 8);
    }

    #[test]
    fn defense_attribute_reduces_incoming_damage() {
        let mut game = Game::new_with_language(Language::English);
        game.start_combat(0, Pos { x: 4, y: 1 }, Language::English);
        game.player.defense = 3;
        game.player.hp = 30;
        game.combat.as_mut().unwrap().player_block = 0;

        game.end_turn(Language::English);

        assert_eq!(game.player.hp, 29);
    }

    #[test]
    fn combat_rewards_gold_and_experience() {
        let mut game = Game::new_with_language(Language::English);
        let expected_gold = game.monsters[0].gold;
        let expected_experience = game.monsters[0].experience;
        game.start_combat(0, Pos { x: 4, y: 1 }, Language::English);
        let combat = game.combat.as_mut().unwrap();
        combat.enemy_hp = 1;
        combat.hand = vec![CardId::Strike];
        combat.energy = STARTING_ENERGY;

        game.play_card(0, Language::English);

        assert_eq!(game.player.gold, expected_gold);
        assert_eq!(game.player.experience, expected_experience);
    }

    #[test]
    fn click_reachable_item_moves_and_collects_it() {
        let mut game = Game::new_with_language(Language::English);

        let moved = game.try_click_tile(Pos { x: 3, y: 3 }, Language::English);

        assert!(moved);
        assert_eq!(game.player.pos, Pos { x: 3, y: 3 });
        assert_eq!(game.player.blue_keys, 1);
        assert_eq!(game.tile_at(Pos { x: 3, y: 3 }), Tile::Floor);
    }

    #[test]
    fn click_reachable_monster_moves_next_to_it_and_starts_combat() {
        let mut game = Game::new_with_language(Language::English);

        let moved = game.try_click_tile(Pos { x: 4, y: 1 }, Language::English);

        assert!(moved);
        assert_eq!(game.player.pos, Pos { x: 3, y: 1 });
        assert_eq!(game.mode, Mode::Combat);
        assert_eq!(game.pending_monster_pos, Some(Pos { x: 4, y: 1 }));
    }

    #[test]
    fn click_blocked_monster_does_not_move() {
        let mut game = Game::new_with_language(Language::English);

        let moved = game.try_click_tile(Pos { x: 7, y: 7 }, Language::English);

        assert!(!moved);
        assert_eq!(game.player.pos, Pos { x: 1, y: 1 });
        assert_eq!(game.mode, Mode::Explore);
        assert!(game.combat.is_none());
    }

    #[test]
    fn game_state_can_roundtrip_json() {
        let game = Game::new_with_language(Language::English);
        let data = serde_json::to_string(&game).unwrap();
        let loaded: Game = serde_json::from_str(&data).unwrap();

        assert_eq!(loaded.mode, game.mode);
        assert_eq!(loaded.player.pos, game.player.pos);
        assert_eq!(loaded.deck, game.deck);
        assert_eq!(loaded.tiles, game.tiles);
    }

    #[test]
    fn all_stage_one_floor_layouts_are_well_formed() {
        for floor in 1..=FINAL_FLOOR {
            let layout = floor_layout(floor);
            assert_eq!(layout.len(), MAP_HEIGHT);

            let mut starts = 0;
            let mut stairs = 0;
            for row in layout {
                assert_eq!(row.chars().count(), MAP_WIDTH, "floor {floor}: {row}");
                starts += row.chars().filter(|ch| *ch == 'P').count();
                stairs += row.chars().filter(|ch| *ch == 'S').count();
            }

            assert_eq!(starts, 1, "floor {floor} must have one start");
            assert_eq!(stairs, 1, "floor {floor} must have one stair");
        }
    }

    #[test]
    fn stage_one_has_monster_instances_for_every_floor_slot() {
        let game = Game::new_with_language(Language::English);

        assert_eq!(
            game.monsters.len(),
            FINAL_FLOOR as usize * MONSTER_SLOTS_PER_FLOOR
        );
        assert_eq!(
            game.monsters[monster_index_for_floor(20, 4)].rank,
            MonsterRank::Boss
        );
        assert!(
            game.monsters[monster_index_for_floor(20, 4)].max_hp
                > game.monsters[monster_index_for_floor(1, 4)].max_hp
        );
    }

    #[test]
    fn stairs_advance_until_final_floor_then_win() {
        let mut game = Game::new_with_language(Language::English);
        game.player.pos = Pos { x: 9, y: 7 };

        game.try_move(1, 0, Language::English);

        assert_eq!(game.floor, 2);
        assert_eq!(game.mode, Mode::Explore);
        assert_eq!(game.player.pos, Pos { x: 1, y: 1 });

        game.floor = FINAL_FLOOR;
        game.load_floor(FINAL_FLOOR);
        game.player.pos = Pos { x: 8, y: 7 };

        game.try_move(1, 0, Language::English);

        assert_eq!(game.mode, Mode::Victory);
    }

    #[test]
    fn shop_spends_gold_and_adds_a_card() {
        let mut game = Game::new_with_language(Language::English);
        game.floor = 4;
        game.load_floor(4);
        game.player.gold = shop_cost(4);
        let deck_len = game.deck.len();

        let entered = game.enter_tile(Pos { x: 7, y: 1 }, Language::English);

        assert!(entered);
        assert_eq!(game.player.gold, 0);
        assert_eq!(game.deck.len(), deck_len + 1);
    }

    #[test]
    fn sage_spends_experience_and_grants_boon() {
        let mut game = Game::new_with_language(Language::English);
        game.floor = 5;
        game.load_floor(5);
        game.player.experience = sage_cost(5);
        let attack = game.player.attack;

        let entered = game.enter_tile(Pos { x: 4, y: 5 }, Language::English);

        assert!(entered);
        assert_eq!(game.player.experience, 0);
        assert_eq!(game.player.attack, attack + 1);
    }
}
