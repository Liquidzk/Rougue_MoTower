use serde::{Deserialize, Serialize};

use crate::game::{BossBonus, CardId, MonsterId};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Language {
    Chinese,
    English,
}

impl Default for Language {
    fn default() -> Self {
        Self::Chinese
    }
}

impl Language {
    pub fn toggled(self) -> Self {
        match self {
            Self::Chinese => Self::English,
            Self::English => Self::Chinese,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TextKey {
    AppTitle,
    AppSubtitle,
    MenuNewGame,
    MenuLoadGame,
    MenuSettings,
    MenuExit,
    MenuHint,
    MenuNoSave,
    MenuSaved,
    SettingsTitle,
    SettingsLanguage,
    SettingsBack,
    SettingsHint,
    ExploreTitle,
    StatsFloor,
    StatsHp,
    StatsGold,
    StatsYellowKeys,
    StatsBlueKeys,
    StatsDeck,
    StatsDamageBonus,
    ControlsTitle,
    ControlsMove,
    ControlsPlayCard,
    ControlsEndTurn,
    ControlsRestart,
    ControlsBackToMenu,
    CombatTitle,
    CombatHint,
    PlayerLabel,
    BlockLabel,
    EnergyLabel,
    DrawLabel,
    DiscardLabel,
    IntentLabel,
    RewardChooseHint,
    VictoryTitle,
    VictoryDemoComplete,
    GameOverTitle,
    ButtonEndTurn,
    ButtonSkipReward,
    ButtonRestart,
    ButtonMenu,
}

pub fn text(language: Language, key: TextKey) -> &'static str {
    match language {
        Language::Chinese => match key {
            TextKey::AppTitle => "魔塔牌塔",
            TextKey::AppSubtitle => "魔塔探索 + 牌组战斗 Demo",
            TextKey::MenuNewGame => "开始新游戏",
            TextKey::MenuLoadGame => "加载上次保存",
            TextKey::MenuSettings => "设定",
            TextKey::MenuExit => "退出游戏",
            TextKey::MenuHint => "鼠标点击或方向键 / W S 选择，Enter / Space 确认",
            TextKey::MenuNoSave => "还没有可加载的存档。",
            TextKey::MenuSaved => "已保存当前进度。",
            TextKey::SettingsTitle => "设定",
            TextKey::SettingsLanguage => "语言",
            TextKey::SettingsBack => "返回",
            TextKey::SettingsHint => "鼠标点击或 Enter / Space 切换，Esc 返回",
            TextKey::ExploreTitle => "魔塔牌塔",
            TextKey::StatsFloor => "楼层",
            TextKey::StatsHp => "生命",
            TextKey::StatsGold => "金币",
            TextKey::StatsYellowKeys => "黄钥匙",
            TextKey::StatsBlueKeys => "蓝钥匙",
            TextKey::StatsDeck => "牌组",
            TextKey::StatsDamageBonus => "伤害加成",
            TextKey::ControlsTitle => "操作",
            TextKey::ControlsMove => "WASD / 方向键：移动",
            TextKey::ControlsPlayCard => "1-5：战斗中出牌",
            TextKey::ControlsEndTurn => "Space / Enter：结束回合或跳过",
            TextKey::ControlsRestart => "R：重新开始",
            TextKey::ControlsBackToMenu => "Esc：保存并返回菜单",
            TextKey::CombatTitle => "卡牌战斗",
            TextKey::CombatHint => "按 1-5 打出手牌，Space / Enter 结束回合。",
            TextKey::PlayerLabel => "玩家",
            TextKey::BlockLabel => "格挡",
            TextKey::EnergyLabel => "能量",
            TextKey::DrawLabel => "抽牌堆",
            TextKey::DiscardLabel => "弃牌堆",
            TextKey::IntentLabel => "意图",
            TextKey::RewardChooseHint => "按 1-3 加入一张牌，Space / Enter 跳过。",
            TextKey::VictoryTitle => "第一层已清理",
            TextKey::VictoryDemoComplete => "Demo 完成。",
            TextKey::GameOverTitle => "游戏结束",
            TextKey::ButtonEndTurn => "结束回合",
            TextKey::ButtonSkipReward => "跳过奖励",
            TextKey::ButtonRestart => "重新开始",
            TextKey::ButtonMenu => "返回菜单",
        },
        Language::English => match key {
            TextKey::AppTitle => "Rougue MoTower",
            TextKey::AppSubtitle => "Magic Tower exploration + deck-builder battles demo",
            TextKey::MenuNewGame => "Start New Game",
            TextKey::MenuLoadGame => "Load Last Save",
            TextKey::MenuSettings => "Settings",
            TextKey::MenuExit => "Exit Game",
            TextKey::MenuHint => {
                "Click or use Arrow keys / W S to choose, Enter / Space to confirm"
            }
            TextKey::MenuNoSave => "No saved game found.",
            TextKey::MenuSaved => "Current progress saved.",
            TextKey::SettingsTitle => "Settings",
            TextKey::SettingsLanguage => "Language",
            TextKey::SettingsBack => "Back",
            TextKey::SettingsHint => "Click or press Enter / Space to toggle, Esc to go back",
            TextKey::ExploreTitle => "Rougue MoTower",
            TextKey::StatsFloor => "Floor",
            TextKey::StatsHp => "HP",
            TextKey::StatsGold => "Gold",
            TextKey::StatsYellowKeys => "Yellow Keys",
            TextKey::StatsBlueKeys => "Blue Keys",
            TextKey::StatsDeck => "Deck",
            TextKey::StatsDamageBonus => "Damage Bonus",
            TextKey::ControlsTitle => "Controls",
            TextKey::ControlsMove => "WASD / Arrows: move",
            TextKey::ControlsPlayCard => "1-5: play card in battle",
            TextKey::ControlsEndTurn => "Space / Enter: end turn or skip",
            TextKey::ControlsRestart => "R: restart",
            TextKey::ControlsBackToMenu => "Esc: save and return to menu",
            TextKey::CombatTitle => "Card Battle",
            TextKey::CombatHint => "Press 1-5 to play a hand card. Space / Enter ends turn.",
            TextKey::PlayerLabel => "Player",
            TextKey::BlockLabel => "Block",
            TextKey::EnergyLabel => "Energy",
            TextKey::DrawLabel => "Draw",
            TextKey::DiscardLabel => "Discard",
            TextKey::IntentLabel => "Intent",
            TextKey::RewardChooseHint => "Choose 1-3 to add a card. Space / Enter skips.",
            TextKey::VictoryTitle => "Floor 1 Cleared",
            TextKey::VictoryDemoComplete => "Demo complete.",
            TextKey::GameOverTitle => "Game Over",
            TextKey::ButtonEndTurn => "End Turn",
            TextKey::ButtonSkipReward => "Skip Reward",
            TextKey::ButtonRestart => "Restart",
            TextKey::ButtonMenu => "Menu",
        },
    }
}

pub fn language_name(language: Language) -> &'static str {
    match language {
        Language::Chinese => "中文",
        Language::English => "English",
    }
}

pub fn card_name(language: Language, card: CardId) -> &'static str {
    match language {
        Language::Chinese => match card {
            CardId::Strike => "攻击",
            CardId::Guard => "防御",
            CardId::HeavySlash => "重斩",
            CardId::Spark => "火花",
            CardId::ShieldBash => "盾击",
            CardId::FirstAid => "急救",
        },
        Language::English => match card {
            CardId::Strike => "Strike",
            CardId::Guard => "Guard",
            CardId::HeavySlash => "Heavy Slash",
            CardId::Spark => "Spark",
            CardId::ShieldBash => "Shield Bash",
            CardId::FirstAid => "First Aid",
        },
    }
}

pub fn card_text(language: Language, card: CardId) -> &'static str {
    match language {
        Language::Chinese => match card {
            CardId::Strike => "造成 6 点伤害。",
            CardId::Guard => "获得 6 点格挡。",
            CardId::HeavySlash => "造成 13 点伤害。",
            CardId::Spark => "造成 4 点伤害两次。",
            CardId::ShieldBash => "造成 5 点伤害。\n获得 4 点格挡。",
            CardId::FirstAid => "回复 5 点生命。",
        },
        Language::English => match card {
            CardId::Strike => "Deal 6 damage.",
            CardId::Guard => "Gain 6 block.",
            CardId::HeavySlash => "Deal 13 damage.",
            CardId::Spark => "Deal 4 damage twice.",
            CardId::ShieldBash => "Deal 5 damage. Gain 4 block.",
            CardId::FirstAid => "Heal 5 HP.",
        },
    }
}

pub fn monster_name(language: Language, monster: MonsterId) -> &'static str {
    match language {
        Language::Chinese => match monster {
            MonsterId::GreenSlime => "绿史莱姆",
            MonsterId::CaveBat => "洞穴蝙蝠",
            MonsterId::BoneGuard => "骸骨守卫",
            MonsterId::IronCaptain => "铁甲队长",
            MonsterId::FloorGuardian => "楼层守护者",
            MonsterId::RedSlime => "红史莱姆",
            MonsterId::VampireBat => "吸血蝙蝠",
            MonsterId::StoneGuard => "石像守卫",
            MonsterId::RuneCaptain => "符文队长",
            MonsterId::TowerWarden => "高塔典狱长",
            MonsterId::DarkSlime => "暗影史莱姆",
            MonsterId::Warlock => "咒术师",
            MonsterId::IronGolem => "钢铁魔像",
            MonsterId::AbyssKnight => "深渊骑士",
            MonsterId::DemonLord => "恶魔领主",
            MonsterId::FlameSlime => "烈焰史莱姆",
            MonsterId::Dragonling => "幼龙",
            MonsterId::GoldenGuard => "黄金守卫",
            MonsterId::ChaosKnight => "混沌骑士",
            MonsterId::AncientDragon => "远古巨龙",
        },
        Language::English => match monster {
            MonsterId::GreenSlime => "Green Slime",
            MonsterId::CaveBat => "Cave Bat",
            MonsterId::BoneGuard => "Bone Guard",
            MonsterId::IronCaptain => "Iron Captain",
            MonsterId::FloorGuardian => "Floor Guardian",
            MonsterId::RedSlime => "Red Slime",
            MonsterId::VampireBat => "Vampire Bat",
            MonsterId::StoneGuard => "Stone Guard",
            MonsterId::RuneCaptain => "Rune Captain",
            MonsterId::TowerWarden => "Tower Warden",
            MonsterId::DarkSlime => "Dark Slime",
            MonsterId::Warlock => "Warlock",
            MonsterId::IronGolem => "Iron Golem",
            MonsterId::AbyssKnight => "Abyss Knight",
            MonsterId::DemonLord => "Demon Lord",
            MonsterId::FlameSlime => "Flame Slime",
            MonsterId::Dragonling => "Dragonling",
            MonsterId::GoldenGuard => "Golden Guard",
            MonsterId::ChaosKnight => "Chaos Knight",
            MonsterId::AncientDragon => "Ancient Dragon",
        },
    }
}

pub fn boss_bonus_text(language: Language, bonus: BossBonus) -> &'static str {
    match language {
        Language::Chinese => match bonus {
            BossBonus::MiniBossMaxHp => "小 Boss 奖励：最大生命 +6。",
            BossBonus::BossAttackBonus => "Boss 奖励：卡牌伤害 +1。",
        },
        Language::English => match bonus {
            BossBonus::MiniBossMaxHp => "Mini boss bonus: +6 max HP.",
            BossBonus::BossAttackBonus => "Boss bonus: +1 card damage.",
        },
    }
}

pub fn initial_message(language: Language) -> String {
    match language {
        Language::Chinese => "探索魔塔。使用 WASD 或方向键移动。".to_string(),
        Language::English => "Explore the tower. Move with WASD or arrows.".to_string(),
    }
}

pub fn message_wall_bounds(language: Language) -> String {
    match language {
        Language::Chinese => "魔塔边界挡住了去路。".to_string(),
        Language::English => "The tower wall blocks the way.".to_string(),
    }
}

pub fn message_solid_wall(language: Language) -> String {
    match language {
        Language::Chinese => "坚固的墙挡住了去路。".to_string(),
        Language::English => "A solid wall blocks the way.".to_string(),
    }
}

pub fn message_stage_clear(language: Language) -> String {
    match language {
        Language::Chinese => "阶段 1 Demo 已清理。".to_string(),
        Language::English => "Stage 1 demo cleared.".to_string(),
    }
}

pub fn message_enter_floor(language: Language, floor: i32) -> String {
    match language {
        Language::Chinese => format!("进入第 {floor} 层。"),
        Language::English => format!("Entered floor {floor}."),
    }
}

pub fn message_yellow_door_opened(language: Language) -> String {
    match language {
        Language::Chinese => "黄色门已打开。".to_string(),
        Language::English => "Yellow door opened.".to_string(),
    }
}

pub fn message_need_yellow_key(language: Language) -> String {
    match language {
        Language::Chinese => "需要一把黄钥匙。".to_string(),
        Language::English => "Need a yellow key.".to_string(),
    }
}

pub fn message_blue_door_opened(language: Language) -> String {
    match language {
        Language::Chinese => "蓝色门已打开。".to_string(),
        Language::English => "Blue door opened.".to_string(),
    }
}

pub fn message_need_blue_key(language: Language) -> String {
    match language {
        Language::Chinese => "需要一把蓝钥匙。".to_string(),
        Language::English => "Need a blue key.".to_string(),
    }
}

pub fn message_yellow_key(language: Language) -> String {
    match language {
        Language::Chinese => "拾取了一把黄钥匙。".to_string(),
        Language::English => "Picked up a yellow key.".to_string(),
    }
}

pub fn message_blue_key(language: Language) -> String {
    match language {
        Language::Chinese => "拾取了一把蓝钥匙。后续楼层会用到它。".to_string(),
        Language::English => "Picked up a blue key. Later floors will spend it.".to_string(),
    }
}

pub fn message_potion(language: Language) -> String {
    match language {
        Language::Chinese => "药水回复 18 点生命。".to_string(),
        Language::English => "Potion restored 18 HP.".to_string(),
    }
}

pub fn message_chest(language: Language) -> String {
    match language {
        Language::Chinese => "宝箱：+12 金币，并将盾击加入牌组。".to_string(),
        Language::English => "Chest: +12 gold and Shield Bash added to deck.".to_string(),
    }
}

pub fn message_battle(language: Language, monster: MonsterId) -> String {
    match language {
        Language::Chinese => format!("战斗：{}", monster_name(language, monster)),
        Language::English => format!("Battle: {}", monster_name(language, monster)),
    }
}

pub fn message_no_card(language: Language) -> String {
    match language {
        Language::Chinese => "该位置没有手牌。".to_string(),
        Language::English => "No card in that slot.".to_string(),
    }
}

pub fn message_not_enough_energy(language: Language, card: CardId) -> String {
    match language {
        Language::Chinese => format!("能量不足，无法打出{}。", card_name(language, card)),
        Language::English => format!("Not enough energy for {}.", card_name(language, card)),
    }
}

pub fn message_fell(language: Language) -> String {
    match language {
        Language::Chinese => "你倒在了塔里。按 R 重新开始。".to_string(),
        Language::English => "You fell in the tower. Press R to restart.".to_string(),
    }
}

pub fn message_added_card(language: Language, card: CardId) -> String {
    match language {
        Language::Chinese => format!("已将{}加入牌组。继续探索。", card_name(language, card)),
        Language::English => format!(
            "Added {} to deck. Continue exploring.",
            card_name(language, card)
        ),
    }
}

pub fn message_skipped_reward(language: Language) -> String {
    match language {
        Language::Chinese => "跳过奖励。继续探索。".to_string(),
        Language::English => "Skipped reward. Continue exploring.".to_string(),
    }
}

pub fn message_defeated_choose_reward(language: Language, monster: MonsterId) -> String {
    match language {
        Language::Chinese => format!(
            "{}已被击败。选择一张卡牌奖励。",
            monster_name(language, monster)
        ),
        Language::English => format!(
            "{} defeated. Choose a card reward.",
            monster_name(language, monster)
        ),
    }
}

pub fn message_loaded(language: Language) -> String {
    match language {
        Language::Chinese => "已加载上次保存的游戏。".to_string(),
        Language::English => "Loaded the last saved game.".to_string(),
    }
}

pub fn message_load_failed(language: Language, error: &str) -> String {
    match language {
        Language::Chinese => format!("加载失败：{error}"),
        Language::English => format!("Load failed: {error}"),
    }
}

pub fn message_save_failed(language: Language, error: &str) -> String {
    match language {
        Language::Chinese => format!("保存失败：{error}"),
        Language::English => format!("Save failed: {error}"),
    }
}

pub fn message_language_changed(language: Language) -> String {
    match language {
        Language::Chinese => "语言已切换为中文。".to_string(),
        Language::English => "Language changed to English.".to_string(),
    }
}

pub fn log_monster_appears(language: Language, monster: MonsterId) -> String {
    match language {
        Language::Chinese => format!("{}出现了。", monster_name(language, monster)),
        Language::English => format!("{} appears.", monster_name(language, monster)),
    }
}

pub fn log_discard_reshuffled(language: Language) -> String {
    match language {
        Language::Chinese => "弃牌堆已重洗。".to_string(),
        Language::English => "Discard pile reshuffled.".to_string(),
    }
}

pub fn log_enemy_attack(
    language: Language,
    monster: MonsterId,
    damage: i32,
    blocked: i32,
    taken: i32,
) -> String {
    match language {
        Language::Chinese => format!(
            "{}攻击 {damage} 点；格挡 {blocked} 点，受到 {taken} 点伤害。",
            monster_name(language, monster)
        ),
        Language::English => format!(
            "{} attacks for {damage}; blocked {blocked}, took {taken}.",
            monster_name(language, monster)
        ),
    }
}

pub fn log_enemy_block(language: Language, monster: MonsterId, block: i32) -> String {
    match language {
        Language::Chinese => format!("{}获得 {block} 点格挡。", monster_name(language, monster)),
        Language::English => format!("{} gains {block} block.", monster_name(language, monster)),
    }
}

pub fn log_card(language: Language, card: CardId, damage: i32) -> String {
    match language {
        Language::Chinese => match card {
            CardId::Strike => format!("攻击造成 {damage} 点伤害。"),
            CardId::Guard => "防御获得 6 点格挡。".to_string(),
            CardId::HeavySlash => format!("重斩造成 {damage} 点伤害。"),
            CardId::Spark => format!("火花造成两次 {damage} 点伤害。"),
            CardId::ShieldBash => format!("盾击造成 {damage} 点伤害，并获得 4 点格挡。"),
            CardId::FirstAid => "急救回复 5 点生命。".to_string(),
        },
        Language::English => match card {
            CardId::Strike => format!("Strike deals {damage}."),
            CardId::Guard => "Guard adds 6 block.".to_string(),
            CardId::HeavySlash => format!("Heavy Slash deals {damage}."),
            CardId::Spark => format!("Spark deals {damage} twice."),
            CardId::ShieldBash => format!("Shield Bash deals {damage} and adds 4 block."),
            CardId::FirstAid => "First Aid heals 5 HP.".to_string(),
        },
    }
}

pub fn enemy_intent(language: Language, damage: i32, block: i32) -> String {
    match language {
        Language::Chinese => match (damage, block) {
            (damage, 0) => format!("攻击 {damage}"),
            (0, block) => format!("格挡 {block}"),
            (damage, block) => format!("攻击 {damage}，格挡 {block}"),
        },
        Language::English => match (damage, block) {
            (damage, 0) => format!("Attack {damage}"),
            (0, block) => format!("Block {block}"),
            (damage, block) => format!("Attack {damage}, Block {block}"),
        },
    }
}

pub fn reward_gold(language: Language, gold: i32) -> String {
    match language {
        Language::Chinese => format!("获得 {gold} 金币。"),
        Language::English => format!("Gained {gold} gold."),
    }
}
