use std::collections::VecDeque;

use serde::{Deserialize, Serialize};

use crate::localization::{self, Language};

pub const MAP_WIDTH: usize = 12;
pub const MAP_HEIGHT: usize = 9;
pub const STARTING_ENERGY: i32 = 3;
pub const HAND_SIZE: usize = 5;

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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Player {
    pub pos: Pos,
    pub hp: i32,
    pub max_hp: i32,
    pub attack_bonus: i32,
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
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Monster {
    pub kind: MonsterId,
    pub max_hp: i32,
    pub hp: i32,
    pub attack: i32,
    pub gold: i32,
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
            message: localization::initial_message(language),
        };

        game.load_demo_floor();
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
                self.player.pos = target;
                self.mode = Mode::Victory;
                self.message = localization::message_floor_clear(language);
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
            | Tile::Chest => true,
            Tile::YellowDoor => self.player.yellow_keys > 0,
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

        let blocked = damage.min(combat.player_block);
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
                Some(BossBonus::MiniBossMaxHp)
            }
            MonsterRank::Boss => {
                self.player.attack_bonus += 1;
                Some(BossBonus::BossAttackBonus)
            }
        };

        self.reward = Some(RewardState {
            monster_kind,
            gold,
            offers: reward_cards(index, rank),
            boss_bonus,
        });
        self.mode = Mode::Reward;
        self.message = localization::message_defeated_choose_reward(language, monster_kind);
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
            let damage = 6 + player.attack_bonus;
            combat.deal_enemy_damage(damage);
            combat
                .log
                .push(localization::log_card(language, card, damage));
        }
        CardId::Guard => {
            combat.player_block += 6;
            combat.log.push(localization::log_card(language, card, 0));
        }
        CardId::HeavySlash => {
            let damage = 13 + player.attack_bonus;
            combat.deal_enemy_damage(damage);
            combat
                .log
                .push(localization::log_card(language, card, damage));
        }
        CardId::Spark => {
            let damage = 4 + player.attack_bonus;
            combat.deal_enemy_damage(damage);
            combat.deal_enemy_damage(damage);
            combat
                .log
                .push(localization::log_card(language, card, damage));
        }
        CardId::ShieldBash => {
            let damage = 5 + player.attack_bonus;
            combat.deal_enemy_damage(damage);
            combat.player_block += 4;
            combat
                .log
                .push(localization::log_card(language, card, damage));
        }
        CardId::FirstAid => {
            player.hp = (player.hp + 5).min(player.max_hp);
            combat.log.push(localization::log_card(language, card, 0));
        }
    }
}

fn demo_monsters() -> Vec<Monster> {
    vec![
        Monster {
            kind: MonsterId::GreenSlime,
            max_hp: 18,
            hp: 18,
            attack: 4,
            gold: 5,
            rank: MonsterRank::Normal,
            defeated: false,
        },
        Monster {
            kind: MonsterId::CaveBat,
            max_hp: 24,
            hp: 24,
            attack: 6,
            gold: 7,
            rank: MonsterRank::Normal,
            defeated: false,
        },
        Monster {
            kind: MonsterId::BoneGuard,
            max_hp: 34,
            hp: 34,
            attack: 8,
            gold: 10,
            rank: MonsterRank::Normal,
            defeated: false,
        },
        Monster {
            kind: MonsterId::IronCaptain,
            max_hp: 54,
            hp: 54,
            attack: 10,
            gold: 18,
            rank: MonsterRank::MiniBoss,
            defeated: false,
        },
        Monster {
            kind: MonsterId::FloorGuardian,
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
}
