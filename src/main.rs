mod game;

use bevy::prelude::*;
use bevy::sprite::Anchor;
use game::{card_def, CardId, Game, Mode, MonsterRank, Tile, HAND_SIZE, MAP_HEIGHT, MAP_WIDTH};

const TILE_SIZE: f32 = 48.0;
const MAP_ORIGIN_X: f32 = -455.0;
const MAP_ORIGIN_Y: f32 = 205.0;
const CARD_W: f32 = 150.0;
const CARD_H: f32 = 104.0;

#[derive(Component)]
struct SceneEntity;

#[derive(Resource)]
struct GameState {
    game: Game,
    dirty: bool,
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.05, 0.06, 0.08)))
        .insert_resource(GameState {
            game: Game::new(),
            dirty: true,
        })
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Rougue MoTower - Bevy Demo".to_string(),
                resolution: (1280.0, 720.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup_camera)
        .add_systems(Update, (handle_keyboard, render_scene).chain())
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn handle_keyboard(keys: Res<ButtonInput<KeyCode>>, mut state: ResMut<GameState>) {
    let mut changed = false;

    if keys.just_pressed(KeyCode::KeyR) {
        state.game.restart();
        changed = true;
    }

    match state.game.mode {
        Mode::Explore => {
            let movement = if keys.just_pressed(KeyCode::ArrowUp)
                || keys.just_pressed(KeyCode::KeyW)
            {
                Some((0, -1))
            } else if keys.just_pressed(KeyCode::ArrowDown) || keys.just_pressed(KeyCode::KeyS) {
                Some((0, 1))
            } else if keys.just_pressed(KeyCode::ArrowLeft) || keys.just_pressed(KeyCode::KeyA) {
                Some((-1, 0))
            } else if keys.just_pressed(KeyCode::ArrowRight) || keys.just_pressed(KeyCode::KeyD) {
                Some((1, 0))
            } else {
                None
            };

            if let Some((dx, dy)) = movement {
                state.game.try_move(dx, dy);
                changed = true;
            }
        }
        Mode::Combat => {
            if keys.just_pressed(KeyCode::Digit1) {
                state.game.play_card(0);
                changed = true;
            } else if keys.just_pressed(KeyCode::Digit2) {
                state.game.play_card(1);
                changed = true;
            } else if keys.just_pressed(KeyCode::Digit3) {
                state.game.play_card(2);
                changed = true;
            } else if keys.just_pressed(KeyCode::Digit4) {
                state.game.play_card(3);
                changed = true;
            } else if keys.just_pressed(KeyCode::Digit5) {
                state.game.play_card(4);
                changed = true;
            } else if keys.just_pressed(KeyCode::Space) || keys.just_pressed(KeyCode::Enter) {
                state.game.end_turn();
                changed = true;
            }
        }
        Mode::Reward => {
            if keys.just_pressed(KeyCode::Digit1) {
                state.game.choose_reward(Some(0));
                changed = true;
            } else if keys.just_pressed(KeyCode::Digit2) {
                state.game.choose_reward(Some(1));
                changed = true;
            } else if keys.just_pressed(KeyCode::Digit3) {
                state.game.choose_reward(Some(2));
                changed = true;
            } else if keys.just_pressed(KeyCode::Space) || keys.just_pressed(KeyCode::Enter) {
                state.game.choose_reward(None);
                changed = true;
            }
        }
        Mode::Victory | Mode::GameOver => {}
    }

    if changed {
        state.dirty = true;
    }
}

fn render_scene(
    mut commands: Commands,
    mut state: ResMut<GameState>,
    rendered: Query<Entity, With<SceneEntity>>,
) {
    if !state.dirty {
        return;
    }

    for entity in &rendered {
        commands.entity(entity).despawn_recursive();
    }

    match state.game.mode {
        Mode::Explore => render_explore(&mut commands, &state.game),
        Mode::Combat => render_combat(&mut commands, &state.game),
        Mode::Reward => render_reward(&mut commands, &state.game),
        Mode::Victory => render_victory(&mut commands, &state.game),
        Mode::GameOver => render_game_over(&mut commands, &state.game),
    }

    state.dirty = false;
}

fn render_explore(commands: &mut Commands, game: &Game) {
    spawn_label(
        commands,
        "Rougue MoTower",
        Vec3::new(-470.0, 320.0, 5.0),
        34.0,
        Color::srgb(0.92, 0.88, 0.72),
        Anchor::CenterLeft,
    );

    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            let pos = game::Pos { x, y };
            let tile = game.tile_at(pos);
            let (color, marker) = tile_visual(game, tile);
            let center = tile_center(x, y);
            spawn_rect(commands, center, Vec2::splat(TILE_SIZE - 3.0), color, 0.0);

            if !marker.is_empty() {
                spawn_label(
                    commands,
                    marker,
                    center + Vec3::new(0.0, -1.0, 3.0),
                    24.0,
                    Color::srgb(0.96, 0.95, 0.88),
                    Anchor::Center,
                );
            }
        }
    }

    let player_center = tile_center(game.player.pos.x, game.player.pos.y);
    spawn_rect(
        commands,
        player_center + Vec3::new(0.0, 0.0, 2.0),
        Vec2::splat(TILE_SIZE - 12.0),
        Color::srgb(0.18, 0.62, 0.95),
        2.0,
    );
    spawn_label(
        commands,
        "@",
        player_center + Vec3::new(0.0, -1.0, 4.0),
        28.0,
        Color::srgb(1.0, 1.0, 1.0),
        Anchor::Center,
    );

    let stats = format!(
        "Floor {}\nHP {}/{}\nGold {}\nYellow Keys {}\nBlue Keys {}\nDeck {} cards\nDamage Bonus +{}",
        game.floor,
        game.player.hp,
        game.player.max_hp,
        game.player.gold,
        game.player.yellow_keys,
        game.player.blue_keys,
        game.deck.len(),
        game.player.attack_bonus
    );

    spawn_panel(
        commands,
        Vec3::new(330.0, 120.0, 0.0),
        Vec2::new(350.0, 420.0),
    );
    spawn_label(
        commands,
        &stats,
        Vec3::new(190.0, 270.0, 5.0),
        24.0,
        Color::srgb(0.91, 0.93, 0.92),
        Anchor::TopLeft,
    );
    spawn_label(
        commands,
        "Controls\nWASD / Arrows: move\n1-5: play card in battle\nSpace / Enter: end turn or skip\nR: restart",
        Vec3::new(190.0, -20.0, 5.0),
        20.0,
        Color::srgb(0.72, 0.78, 0.82),
        Anchor::TopLeft,
    );
    spawn_label(
        commands,
        &game.message,
        Vec3::new(-470.0, -290.0, 5.0),
        22.0,
        Color::srgb(0.95, 0.88, 0.62),
        Anchor::CenterLeft,
    );
}

fn render_combat(commands: &mut Commands, game: &Game) {
    let combat = game
        .combat
        .as_ref()
        .expect("combat mode must have combat state");

    spawn_label(
        commands,
        "Card Battle",
        Vec3::new(-565.0, 315.0, 5.0),
        34.0,
        Color::srgb(0.92, 0.88, 0.72),
        Anchor::CenterLeft,
    );

    spawn_panel(
        commands,
        Vec3::new(-345.0, 100.0, 0.0),
        Vec2::new(330.0, 350.0),
    );
    let player_text = format!(
        "Player\nHP {}/{}\nBlock {}\nEnergy {}/{}\nGold {}\nDeck {}  Draw {}  Discard {}",
        game.player.hp,
        game.player.max_hp,
        combat.player_block,
        combat.energy,
        game::STARTING_ENERGY,
        game.player.gold,
        game.deck.len(),
        combat.draw_pile.len(),
        combat.discard_pile.len()
    );
    spawn_label(
        commands,
        &player_text,
        Vec3::new(-485.0, 245.0, 5.0),
        22.0,
        Color::srgb(0.9, 0.94, 0.96),
        Anchor::TopLeft,
    );

    let enemy_color = match combat.monster_rank {
        MonsterRank::Normal => Color::srgb(0.54, 0.34, 0.32),
        MonsterRank::MiniBoss => Color::srgb(0.62, 0.38, 0.18),
        MonsterRank::Boss => Color::srgb(0.56, 0.16, 0.18),
    };
    spawn_rect(
        commands,
        Vec3::new(230.0, 150.0, 1.0),
        Vec2::new(280.0, 190.0),
        enemy_color,
        1.0,
    );
    let enemy_text = format!(
        "{}\nHP {}/{}\nBlock {}\nIntent: {}",
        combat.monster_name,
        combat.enemy_hp.max(0),
        combat.enemy_max_hp,
        combat.enemy_block,
        combat.enemy_intent()
    );
    spawn_label(
        commands,
        &enemy_text,
        Vec3::new(230.0, 205.0, 5.0),
        24.0,
        Color::srgb(1.0, 0.97, 0.9),
        Anchor::Center,
    );

    spawn_label(
        commands,
        "Press 1-5 to play a hand card. Space / Enter ends turn.",
        Vec3::new(-565.0, -40.0, 5.0),
        21.0,
        Color::srgb(0.72, 0.78, 0.82),
        Anchor::CenterLeft,
    );

    for (index, card) in combat.hand.iter().take(HAND_SIZE).enumerate() {
        let x = -420.0 + index as f32 * 170.0;
        spawn_card(commands, Vec3::new(x, -215.0, 1.0), index + 1, *card);
    }

    let mut log_lines = combat.log.iter().rev().take(5).cloned().collect::<Vec<_>>();
    log_lines.reverse();
    spawn_label(
        commands,
        &log_lines.join("\n"),
        Vec3::new(-565.0, 35.0, 5.0),
        18.0,
        Color::srgb(0.82, 0.85, 0.86),
        Anchor::TopLeft,
    );
}

fn render_reward(commands: &mut Commands, game: &Game) {
    let reward = game
        .reward
        .as_ref()
        .expect("reward mode must have reward state");

    spawn_label(
        commands,
        &format!("{} defeated", reward.monster_name),
        Vec3::new(-350.0, 260.0, 5.0),
        34.0,
        Color::srgb(0.92, 0.88, 0.72),
        Anchor::CenterLeft,
    );

    let mut summary = format!("Gained {} gold.", reward.gold);
    if let Some(bonus) = reward.boss_bonus {
        summary.push('\n');
        summary.push_str(bonus);
    }

    spawn_label(
        commands,
        &summary,
        Vec3::new(-350.0, 210.0, 5.0),
        24.0,
        Color::srgb(0.9, 0.94, 0.96),
        Anchor::CenterLeft,
    );

    for (index, card) in reward.offers.iter().enumerate() {
        let x = -220.0 + index as f32 * 220.0;
        spawn_card(commands, Vec3::new(x, 20.0, 1.0), index + 1, *card);
    }

    spawn_label(
        commands,
        "Choose 1-3 to add a card. Space / Enter skips.",
        Vec3::new(-350.0, -170.0, 5.0),
        22.0,
        Color::srgb(0.72, 0.78, 0.82),
        Anchor::CenterLeft,
    );
}

fn render_victory(commands: &mut Commands, game: &Game) {
    spawn_label(
        commands,
        "Floor 1 Cleared",
        Vec3::new(-270.0, 140.0, 5.0),
        44.0,
        Color::srgb(0.92, 0.88, 0.72),
        Anchor::CenterLeft,
    );
    spawn_label(
        commands,
        &format!(
            "Demo complete.\nHP {}/{}  Gold {}  Deck {} cards\nPress R to restart.",
            game.player.hp,
            game.player.max_hp,
            game.player.gold,
            game.deck.len()
        ),
        Vec3::new(-270.0, 60.0, 5.0),
        26.0,
        Color::srgb(0.9, 0.94, 0.96),
        Anchor::TopLeft,
    );
}

fn render_game_over(commands: &mut Commands, game: &Game) {
    spawn_label(
        commands,
        "Game Over",
        Vec3::new(-180.0, 120.0, 5.0),
        48.0,
        Color::srgb(0.94, 0.42, 0.34),
        Anchor::CenterLeft,
    );
    spawn_label(
        commands,
        &format!(
            "Reached floor {} with {} gold.\nPress R to restart.",
            game.floor, game.player.gold
        ),
        Vec3::new(-180.0, 45.0, 5.0),
        26.0,
        Color::srgb(0.9, 0.94, 0.96),
        Anchor::TopLeft,
    );
}

fn spawn_card(commands: &mut Commands, center: Vec3, index: usize, card: CardId) {
    let def = card_def(card);
    spawn_rect(
        commands,
        center,
        Vec2::new(CARD_W, CARD_H),
        Color::srgb(0.18, 0.20, 0.24),
        1.0,
    );
    spawn_rect(
        commands,
        center + Vec3::new(0.0, CARD_H * 0.5 - 16.0, 1.0),
        Vec2::new(CARD_W, 28.0),
        Color::srgb(0.30, 0.36, 0.46),
        2.0,
    );

    let text = format!(
        "{}. {} [{}]\n{}",
        index,
        def.name,
        def.cost,
        wrap_card_text(def.text)
    );
    spawn_label(
        commands,
        &text,
        center + Vec3::new(-CARD_W * 0.44, CARD_H * 0.31, 5.0),
        17.0,
        Color::srgb(0.95, 0.95, 0.9),
        Anchor::TopLeft,
    );
}

fn wrap_card_text(text: &str) -> String {
    text.replace(". ", ".\n")
}

fn tile_center(x: usize, y: usize) -> Vec3 {
    Vec3::new(
        MAP_ORIGIN_X + x as f32 * TILE_SIZE,
        MAP_ORIGIN_Y - y as f32 * TILE_SIZE,
        1.0,
    )
}

fn tile_visual(game: &Game, tile: Tile) -> (Color, &'static str) {
    match tile {
        Tile::Floor => (Color::srgb(0.13, 0.15, 0.17), ""),
        Tile::Wall => (Color::srgb(0.03, 0.04, 0.05), "#"),
        Tile::Stairs => (Color::srgb(0.16, 0.40, 0.26), ">"),
        Tile::YellowDoor => (Color::srgb(0.76, 0.54, 0.16), "D"),
        Tile::YellowKey => (Color::srgb(0.80, 0.65, 0.20), "K"),
        Tile::BlueKey => (Color::srgb(0.12, 0.34, 0.72), "K"),
        Tile::SmallPotion => (Color::srgb(0.62, 0.12, 0.20), "+"),
        Tile::Chest => (Color::srgb(0.46, 0.27, 0.12), "C"),
        Tile::Monster(index) => {
            let monster = &game.monsters[index];
            let color = match monster.rank {
                MonsterRank::Normal => Color::srgb(0.44, 0.22, 0.20),
                MonsterRank::MiniBoss => Color::srgb(0.60, 0.32, 0.12),
                MonsterRank::Boss => Color::srgb(0.58, 0.12, 0.14),
            };
            (color, monster.marker)
        }
    }
}

fn spawn_panel(commands: &mut Commands, center: Vec3, size: Vec2) {
    spawn_rect(commands, center, size, Color::srgb(0.10, 0.12, 0.15), 0.0);
}

fn spawn_rect(commands: &mut Commands, center: Vec3, size: Vec2, color: Color, z: f32) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color,
                custom_size: Some(size),
                ..default()
            },
            transform: Transform::from_translation(center + Vec3::new(0.0, 0.0, z)),
            ..default()
        },
        SceneEntity,
    ));
}

fn spawn_label(
    commands: &mut Commands,
    value: &str,
    position: Vec3,
    font_size: f32,
    color: Color,
    anchor: Anchor,
) {
    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                value,
                TextStyle {
                    font_size,
                    color,
                    ..default()
                },
            ),
            text_anchor: anchor,
            transform: Transform::from_translation(position),
            ..default()
        },
        SceneEntity,
    ));
}
