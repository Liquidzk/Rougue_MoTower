mod game;
mod localization;
mod save;

use std::{env, fs};

use bevy::app::AppExit;
use bevy::ecs::message::MessageWriter;
use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy::text::Font;
use bevy::window::{PrimaryWindow, WindowResolution};
use game::{
    card_cost, monster_marker, CardId, Game, Mode, MonsterRank, Tile, HAND_SIZE, MAP_HEIGHT,
    MAP_WIDTH,
};
use localization::{language_name, text, Language, TextKey};

const TILE_SIZE: f32 = 48.0;
const MAP_ORIGIN_X: f32 = -455.0;
const MAP_ORIGIN_Y: f32 = 205.0;
const CARD_W: f32 = 150.0;
const CARD_H: f32 = 104.0;
const MENU_ITEM_COUNT: usize = 4;
const SETTINGS_ITEM_COUNT: usize = 2;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum AppScreen {
    StartMenu,
    Settings,
    Playing,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum StartMenuItem {
    NewGame,
    LoadGame,
    Settings,
    Exit,
}

impl StartMenuItem {
    fn from_index(index: usize) -> Self {
        match index {
            0 => Self::NewGame,
            1 => Self::LoadGame,
            2 => Self::Settings,
            _ => Self::Exit,
        }
    }

    fn index(self) -> usize {
        match self {
            Self::NewGame => 0,
            Self::LoadGame => 1,
            Self::Settings => 2,
            Self::Exit => 3,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum SettingsItem {
    Language,
    Back,
}

impl SettingsItem {
    fn from_index(index: usize) -> Self {
        match index {
            0 => Self::Language,
            _ => Self::Back,
        }
    }

    fn index(self) -> usize {
        match self {
            Self::Language => 0,
            Self::Back => 1,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ClickAction {
    StartMenu(StartMenuItem),
    Settings(SettingsItem),
    Tile(usize, usize),
    PlayCard(usize),
    EndTurn,
    ChooseReward(usize),
    SkipReward,
    Restart,
    BackToMenu,
}

#[derive(Component)]
struct SceneEntity;

#[derive(Component)]
struct ClickArea {
    size: Vec2,
    action: ClickAction,
}

struct CardView {
    center: Vec3,
    index: usize,
    card: CardId,
    action: ClickAction,
    highlighted: bool,
}

#[derive(Resource)]
struct UiFont(Handle<Font>);

#[derive(Resource)]
struct AppState {
    game: Game,
    screen: AppScreen,
    dirty: bool,
    language: Language,
    menu_index: usize,
    settings_index: usize,
    hovered_card: Option<usize>,
    menu_message: String,
}

fn main() {
    let language = Language::default();

    App::new()
        .insert_resource(ClearColor(Color::srgb(0.05, 0.06, 0.08)))
        .insert_resource(AppState {
            game: Game::new_with_language(language),
            screen: AppScreen::StartMenu,
            dirty: true,
            language,
            menu_index: 0,
            settings_index: 0,
            hovered_card: None,
            menu_message: String::new(),
        })
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Rougue MoTower - Bevy Demo".to_string(),
                resolution: WindowResolution::new(1280, 720),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (handle_keyboard, handle_mouse, render_scene).chain(),
        )
        .run();
}

fn setup(mut commands: Commands, mut fonts: ResMut<Assets<Font>>) {
    commands.spawn(Camera2d);
    commands.insert_resource(UiFont(load_ui_font(&mut fonts)));
}

fn load_ui_font(fonts: &mut Assets<Font>) -> Handle<Font> {
    if let Ok(path) = env::var("ROUGUE_MOTOWER_FONT") {
        if let Some(font) = load_font_from_path(fonts, &path) {
            return font;
        }
    }

    let candidates = [
        "/usr/share/fonts/opentype/noto/NotoSansCJK-Regular.ttc",
        "/usr/share/fonts/truetype/droid/DroidSansFallbackFull.ttf",
        "/usr/share/fonts/opentype/noto/NotoSerifCJK-Regular.ttc",
        "/usr/share/fonts/truetype/wqy/wqy-microhei.ttc",
        "/usr/share/fonts/truetype/wqy/wqy-zenhei.ttc",
        "/usr/share/fonts/truetype/arphic/uming.ttc",
        "/usr/share/fonts/truetype/arphic/ukai.ttc",
        "/System/Library/Fonts/PingFang.ttc",
        "/System/Library/Fonts/STHeiti Light.ttc",
        "/System/Library/Fonts/STHeiti Medium.ttc",
        "C:\\Windows\\Fonts\\NotoSansSC-VF.ttf",
        "C:\\Windows\\Fonts\\msyh.ttc",
        "C:\\Windows\\Fonts\\msyh.ttf",
        "C:\\Windows\\Fonts\\simhei.ttf",
        "C:\\Windows\\Fonts\\simsun.ttc",
    ];

    for path in candidates {
        if let Some(font) = load_font_from_path(fonts, path) {
            return font;
        }
    }

    Handle::default()
}

fn load_font_from_path(fonts: &mut Assets<Font>, path: &str) -> Option<Handle<Font>> {
    let bytes = fs::read(path).ok()?;
    let font = Font::try_from_bytes(bytes).ok()?;
    Some(fonts.add(font))
}

fn handle_keyboard(
    keys: Res<ButtonInput<KeyCode>>,
    mut state: ResMut<AppState>,
    mut app_exit: MessageWriter<AppExit>,
) {
    let mut changed = false;
    let mut save_after_change = false;

    match state.screen {
        AppScreen::StartMenu => {
            changed = handle_start_menu_input(&keys, &mut state, &mut app_exit);
        }
        AppScreen::Settings => {
            changed = handle_settings_input(&keys, &mut state);
        }
        AppScreen::Playing => {
            let language = state.language;

            if keys.just_pressed(KeyCode::Escape) {
                autosave_current_game(&mut state);
                state.screen = AppScreen::StartMenu;
                state.menu_message = text(language, TextKey::MenuSaved).to_string();
                changed = true;
            } else {
                if keys.just_pressed(KeyCode::KeyR) {
                    state.game.restart(language);
                    changed = true;
                    save_after_change = true;
                }

                match state.game.mode {
                    Mode::Explore => {
                        let movement = if keys.just_pressed(KeyCode::ArrowUp)
                            || keys.just_pressed(KeyCode::KeyW)
                        {
                            Some((0, -1))
                        } else if keys.just_pressed(KeyCode::ArrowDown)
                            || keys.just_pressed(KeyCode::KeyS)
                        {
                            Some((0, 1))
                        } else if keys.just_pressed(KeyCode::ArrowLeft)
                            || keys.just_pressed(KeyCode::KeyA)
                        {
                            Some((-1, 0))
                        } else if keys.just_pressed(KeyCode::ArrowRight)
                            || keys.just_pressed(KeyCode::KeyD)
                        {
                            Some((1, 0))
                        } else {
                            None
                        };

                        if let Some((dx, dy)) = movement {
                            state.game.try_move(dx, dy, language);
                            changed = true;
                            save_after_change = true;
                        }
                    }
                    Mode::Combat => {
                        if keys.just_pressed(KeyCode::Digit1) {
                            state.game.play_card(0, language);
                            changed = true;
                            save_after_change = true;
                        } else if keys.just_pressed(KeyCode::Digit2) {
                            state.game.play_card(1, language);
                            changed = true;
                            save_after_change = true;
                        } else if keys.just_pressed(KeyCode::Digit3) {
                            state.game.play_card(2, language);
                            changed = true;
                            save_after_change = true;
                        } else if keys.just_pressed(KeyCode::Digit4) {
                            state.game.play_card(3, language);
                            changed = true;
                            save_after_change = true;
                        } else if keys.just_pressed(KeyCode::Digit5) {
                            state.game.play_card(4, language);
                            changed = true;
                            save_after_change = true;
                        } else if keys.just_pressed(KeyCode::Space)
                            || keys.just_pressed(KeyCode::Enter)
                        {
                            state.game.end_turn(language);
                            changed = true;
                            save_after_change = true;
                        }
                    }
                    Mode::Reward => {
                        if keys.just_pressed(KeyCode::Digit1) {
                            state.game.choose_reward(Some(0), language);
                            changed = true;
                            save_after_change = true;
                        } else if keys.just_pressed(KeyCode::Digit2) {
                            state.game.choose_reward(Some(1), language);
                            changed = true;
                            save_after_change = true;
                        } else if keys.just_pressed(KeyCode::Digit3) {
                            state.game.choose_reward(Some(2), language);
                            changed = true;
                            save_after_change = true;
                        } else if keys.just_pressed(KeyCode::Space)
                            || keys.just_pressed(KeyCode::Enter)
                        {
                            state.game.choose_reward(None, language);
                            changed = true;
                            save_after_change = true;
                        }
                    }
                    Mode::Victory | Mode::GameOver => {}
                }
            }
        }
    }

    if save_after_change {
        autosave_current_game(&mut state);
    }

    if changed {
        state.dirty = true;
    }
}

fn handle_mouse(
    mouse: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    click_areas: Query<(&ClickArea, &GlobalTransform)>,
    mut state: ResMut<AppState>,
    mut app_exit: MessageWriter<AppExit>,
) {
    let Some(cursor_world) = cursor_world_position(&windows, &cameras) else {
        if clear_hover_state(&mut state) {
            state.dirty = true;
        }
        return;
    };
    let action = click_action_at(cursor_world, &click_areas);

    let mut changed = update_hover_selection(action, &mut state);
    let mut save_after_change = false;

    if let Some(action) = action.filter(|_| mouse.just_pressed(MouseButton::Left)) {
        let (action_changed, should_save) =
            activate_click_action(action, &mut state, &mut app_exit);
        changed |= action_changed;
        save_after_change |= should_save;
    }

    if save_after_change {
        autosave_current_game(&mut state);
    }

    if changed {
        state.dirty = true;
    }
}

fn cursor_world_position(
    windows: &Query<&Window, With<PrimaryWindow>>,
    cameras: &Query<(&Camera, &GlobalTransform)>,
) -> Option<Vec2> {
    let window = windows.iter().next()?;
    let cursor_position = window.cursor_position()?;
    let (camera, camera_transform) = cameras.iter().next()?;
    camera
        .viewport_to_world_2d(camera_transform, cursor_position)
        .ok()
}

fn click_action_at(
    cursor_world: Vec2,
    click_areas: &Query<(&ClickArea, &GlobalTransform)>,
) -> Option<ClickAction> {
    let mut hit = None;

    for (area, transform) in click_areas {
        let center = transform.translation();
        let half = area.size * 0.5;
        let inside = cursor_world.x >= center.x - half.x
            && cursor_world.x <= center.x + half.x
            && cursor_world.y >= center.y - half.y
            && cursor_world.y <= center.y + half.y;

        if inside && hit.is_none_or(|(z, _)| center.z >= z) {
            hit = Some((center.z, area.action));
        }
    }

    hit.map(|(_, action)| action)
}

fn update_hover_selection(action: Option<ClickAction>, state: &mut AppState) -> bool {
    let mut changed = false;
    let hovered_card = match (state.screen, state.game.mode, action) {
        (AppScreen::Playing, Mode::Combat, Some(ClickAction::PlayCard(index))) => Some(index),
        _ => None,
    };

    if state.hovered_card != hovered_card {
        state.hovered_card = hovered_card;
        changed = true;
    }

    match (state.screen, action) {
        (AppScreen::StartMenu, Some(ClickAction::StartMenu(item))) => {
            let index = item.index();
            if state.menu_index != index {
                state.menu_index = index;
                changed = true;
            }
        }
        (AppScreen::Settings, Some(ClickAction::Settings(item))) => {
            let index = item.index();
            if state.settings_index != index {
                state.settings_index = index;
                changed = true;
            }
        }
        _ => {}
    }

    changed
}

fn clear_hover_state(state: &mut AppState) -> bool {
    if state.hovered_card.is_none() {
        return false;
    }

    state.hovered_card = None;
    true
}

fn activate_click_action(
    action: ClickAction,
    state: &mut AppState,
    app_exit: &mut MessageWriter<AppExit>,
) -> (bool, bool) {
    match action {
        ClickAction::StartMenu(item) if state.screen == AppScreen::StartMenu => {
            (activate_start_menu(item, state, app_exit), false)
        }
        ClickAction::Settings(item) if state.screen == AppScreen::Settings => {
            (activate_settings(item, state), false)
        }
        ClickAction::Tile(x, y)
            if state.screen == AppScreen::Playing && state.game.mode == Mode::Explore =>
        {
            let language = state.language;
            let changed = state.game.try_click_tile(game::Pos { x, y }, language);
            (changed, changed)
        }
        ClickAction::PlayCard(index)
            if state.screen == AppScreen::Playing && state.game.mode == Mode::Combat =>
        {
            let language = state.language;
            state.game.play_card(index, language);
            (true, true)
        }
        ClickAction::EndTurn
            if state.screen == AppScreen::Playing && state.game.mode == Mode::Combat =>
        {
            let language = state.language;
            state.game.end_turn(language);
            (true, true)
        }
        ClickAction::ChooseReward(index)
            if state.screen == AppScreen::Playing && state.game.mode == Mode::Reward =>
        {
            let language = state.language;
            state.game.choose_reward(Some(index), language);
            (true, true)
        }
        ClickAction::SkipReward
            if state.screen == AppScreen::Playing && state.game.mode == Mode::Reward =>
        {
            let language = state.language;
            state.game.choose_reward(None, language);
            (true, true)
        }
        ClickAction::Restart
            if state.screen == AppScreen::Playing
                && matches!(state.game.mode, Mode::Victory | Mode::GameOver) =>
        {
            let language = state.language;
            state.game.restart(language);
            (true, true)
        }
        ClickAction::BackToMenu if state.screen == AppScreen::Playing => {
            let language = state.language;
            autosave_current_game(state);
            state.screen = AppScreen::StartMenu;
            state.menu_message = text(language, TextKey::MenuSaved).to_string();
            (true, false)
        }
        _ => (false, false),
    }
}

fn handle_start_menu_input(
    keys: &ButtonInput<KeyCode>,
    state: &mut AppState,
    app_exit: &mut MessageWriter<AppExit>,
) -> bool {
    if keys.just_pressed(KeyCode::ArrowUp) || keys.just_pressed(KeyCode::KeyW) {
        state.menu_index = (state.menu_index + MENU_ITEM_COUNT - 1) % MENU_ITEM_COUNT;
        return true;
    }

    if keys.just_pressed(KeyCode::ArrowDown) || keys.just_pressed(KeyCode::KeyS) {
        state.menu_index = (state.menu_index + 1) % MENU_ITEM_COUNT;
        return true;
    }

    if keys.just_pressed(KeyCode::Digit1) {
        return activate_start_menu(StartMenuItem::NewGame, state, app_exit);
    }
    if keys.just_pressed(KeyCode::Digit2) {
        return activate_start_menu(StartMenuItem::LoadGame, state, app_exit);
    }
    if keys.just_pressed(KeyCode::Digit3) {
        return activate_start_menu(StartMenuItem::Settings, state, app_exit);
    }
    if keys.just_pressed(KeyCode::Digit4) {
        return activate_start_menu(StartMenuItem::Exit, state, app_exit);
    }

    if keys.just_pressed(KeyCode::Enter) || keys.just_pressed(KeyCode::Space) {
        return activate_start_menu(StartMenuItem::from_index(state.menu_index), state, app_exit);
    }

    false
}

fn activate_start_menu(
    item: StartMenuItem,
    state: &mut AppState,
    app_exit: &mut MessageWriter<AppExit>,
) -> bool {
    match item {
        StartMenuItem::NewGame => {
            state.game = Game::new_with_language(state.language);
            state.screen = AppScreen::Playing;
            state.menu_message.clear();
            autosave_current_game(state);
        }
        StartMenuItem::LoadGame => {
            if !save::save_exists() {
                state.menu_message = text(state.language, TextKey::MenuNoSave).to_string();
            } else {
                match save::load_game() {
                    Ok(mut game) => {
                        game.set_message(localization::message_loaded(state.language));
                        state.game = game;
                        state.screen = AppScreen::Playing;
                        state.menu_message.clear();
                    }
                    Err(error) => {
                        state.menu_message =
                            localization::message_load_failed(state.language, &error);
                    }
                }
            }
        }
        StartMenuItem::Settings => {
            state.screen = AppScreen::Settings;
            state.settings_index = 0;
            state.menu_message.clear();
        }
        StartMenuItem::Exit => {
            app_exit.write(AppExit::Success);
        }
    }

    true
}

fn handle_settings_input(keys: &ButtonInput<KeyCode>, state: &mut AppState) -> bool {
    if keys.just_pressed(KeyCode::Escape) {
        state.screen = AppScreen::StartMenu;
        return true;
    }

    if keys.just_pressed(KeyCode::ArrowUp) || keys.just_pressed(KeyCode::KeyW) {
        state.settings_index =
            (state.settings_index + SETTINGS_ITEM_COUNT - 1) % SETTINGS_ITEM_COUNT;
        return true;
    }

    if keys.just_pressed(KeyCode::ArrowDown) || keys.just_pressed(KeyCode::KeyS) {
        state.settings_index = (state.settings_index + 1) % SETTINGS_ITEM_COUNT;
        return true;
    }

    if keys.just_pressed(KeyCode::Digit1) {
        return activate_settings(SettingsItem::Language, state);
    }
    if keys.just_pressed(KeyCode::Digit2) {
        return activate_settings(SettingsItem::Back, state);
    }

    if keys.just_pressed(KeyCode::Enter) || keys.just_pressed(KeyCode::Space) {
        return activate_settings(SettingsItem::from_index(state.settings_index), state);
    }

    false
}

fn activate_settings(item: SettingsItem, state: &mut AppState) -> bool {
    match item {
        SettingsItem::Language => {
            state.language = state.language.toggled();
            state.menu_message = localization::message_language_changed(state.language);
        }
        SettingsItem::Back => {
            state.screen = AppScreen::StartMenu;
        }
    }

    true
}

fn autosave_current_game(state: &mut AppState) {
    if let Err(error) = save::save_game(&state.game) {
        state
            .game
            .set_message(localization::message_save_failed(state.language, &error));
    }
}

fn render_scene(
    mut commands: Commands,
    mut state: ResMut<AppState>,
    rendered: Query<Entity, With<SceneEntity>>,
    font: Res<UiFont>,
) {
    if !state.dirty {
        return;
    }

    for entity in &rendered {
        commands.entity(entity).despawn();
    }

    if !matches!(state.screen, AppScreen::Playing) || state.game.mode != Mode::Combat {
        state.hovered_card = None;
    }

    match state.screen {
        AppScreen::StartMenu => render_start_menu(&mut commands, &state, &font.0),
        AppScreen::Settings => render_settings(&mut commands, &state, &font.0),
        AppScreen::Playing => match state.game.mode {
            Mode::Explore => render_explore(&mut commands, &state.game, state.language, &font.0),
            Mode::Combat => render_combat(
                &mut commands,
                &state.game,
                state.language,
                &font.0,
                state.hovered_card,
            ),
            Mode::Reward => render_reward(&mut commands, &state.game, state.language, &font.0),
            Mode::Victory => render_victory(&mut commands, &state.game, state.language, &font.0),
            Mode::GameOver => render_game_over(&mut commands, &state.game, state.language, &font.0),
        },
    }

    state.dirty = false;
}

fn render_start_menu(commands: &mut Commands, state: &AppState, font: &Handle<Font>) {
    let language = state.language;
    spawn_label(
        commands,
        text(language, TextKey::AppTitle),
        Vec3::new(-320.0, 195.0, 5.0),
        52.0,
        Color::srgb(0.92, 0.88, 0.72),
        Anchor::CENTER_LEFT,
        font,
    );
    spawn_label(
        commands,
        text(language, TextKey::AppSubtitle),
        Vec3::new(-320.0, 135.0, 5.0),
        23.0,
        Color::srgb(0.72, 0.78, 0.82),
        Anchor::CENTER_LEFT,
        font,
    );

    spawn_panel(
        commands,
        Vec3::new(0.0, -35.0, 0.0),
        Vec2::new(520.0, 330.0),
    );

    let items = [
        (StartMenuItem::NewGame, TextKey::MenuNewGame),
        (StartMenuItem::LoadGame, TextKey::MenuLoadGame),
        (StartMenuItem::Settings, TextKey::MenuSettings),
        (StartMenuItem::Exit, TextKey::MenuExit),
    ];

    for (index, (_, key)) in items.iter().enumerate() {
        let y = 55.0 - index as f32 * 64.0;
        spawn_menu_option(
            commands,
            Vec3::new(0.0, y, 1.0),
            Vec2::new(420.0, 48.0),
            state.menu_index == index,
            &format!("{}. {}", index + 1, text(language, *key)),
            font,
            ClickAction::StartMenu(items[index].0),
        );
    }

    spawn_label(
        commands,
        text(language, TextKey::MenuHint),
        Vec3::new(0.0, -210.0, 5.0),
        20.0,
        Color::srgb(0.66, 0.72, 0.76),
        Anchor::CENTER,
        font,
    );

    if !state.menu_message.is_empty() {
        spawn_label(
            commands,
            &state.menu_message,
            Vec3::new(0.0, -260.0, 5.0),
            22.0,
            Color::srgb(0.95, 0.82, 0.52),
            Anchor::CENTER,
            font,
        );
    }
}

fn render_settings(commands: &mut Commands, state: &AppState, font: &Handle<Font>) {
    let language = state.language;
    spawn_label(
        commands,
        text(language, TextKey::SettingsTitle),
        Vec3::new(-260.0, 170.0, 5.0),
        44.0,
        Color::srgb(0.92, 0.88, 0.72),
        Anchor::CENTER_LEFT,
        font,
    );

    spawn_panel(commands, Vec3::new(0.0, 20.0, 0.0), Vec2::new(560.0, 270.0));

    let language_line = format!(
        "{}: {}",
        text(language, TextKey::SettingsLanguage),
        language_name(language)
    );
    let back_line = text(language, TextKey::SettingsBack).to_string();
    let options = [language_line, back_line];

    for (index, label) in options.iter().enumerate() {
        let y = 70.0 - index as f32 * 70.0;
        spawn_menu_option(
            commands,
            Vec3::new(0.0, y, 1.0),
            Vec2::new(460.0, 50.0),
            state.settings_index == index,
            &format!("{}. {}", index + 1, label),
            font,
            ClickAction::Settings(SettingsItem::from_index(index)),
        );
    }

    spawn_label(
        commands,
        text(language, TextKey::SettingsHint),
        Vec3::new(0.0, -150.0, 5.0),
        20.0,
        Color::srgb(0.66, 0.72, 0.76),
        Anchor::CENTER,
        font,
    );

    if !state.menu_message.is_empty() {
        spawn_label(
            commands,
            &state.menu_message,
            Vec3::new(0.0, -205.0, 5.0),
            22.0,
            Color::srgb(0.95, 0.82, 0.52),
            Anchor::CENTER,
            font,
        );
    }
}

fn spawn_menu_option(
    commands: &mut Commands,
    center: Vec3,
    size: Vec2,
    selected: bool,
    label: &str,
    font: &Handle<Font>,
    action: ClickAction,
) {
    let color = if selected {
        Color::srgb(0.27, 0.36, 0.46)
    } else {
        Color::srgb(0.14, 0.16, 0.20)
    };
    let text_color = if selected {
        Color::srgb(1.0, 0.95, 0.75)
    } else {
        Color::srgb(0.86, 0.88, 0.86)
    };

    spawn_click_rect(commands, center, size, color, 0.0, action);
    spawn_label(
        commands,
        label,
        center + Vec3::new(-size.x * 0.42, 4.0, 4.0),
        24.0,
        text_color,
        Anchor::CENTER_LEFT,
        font,
    );
}

fn render_explore(commands: &mut Commands, game: &Game, language: Language, font: &Handle<Font>) {
    spawn_label(
        commands,
        text(language, TextKey::ExploreTitle),
        Vec3::new(-470.0, 320.0, 5.0),
        34.0,
        Color::srgb(0.92, 0.88, 0.72),
        Anchor::CENTER_LEFT,
        font,
    );

    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            let pos = game::Pos { x, y };
            let tile = game.tile_at(pos);
            let (color, marker) = tile_visual(game, tile);
            let center = tile_center(x, y);
            spawn_click_rect(
                commands,
                center,
                Vec2::splat(TILE_SIZE - 3.0),
                color,
                0.0,
                ClickAction::Tile(x, y),
            );

            if !marker.is_empty() {
                spawn_label(
                    commands,
                    marker,
                    center + Vec3::new(0.0, -1.0, 3.0),
                    24.0,
                    Color::srgb(0.96, 0.95, 0.88),
                    Anchor::CENTER,
                    font,
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
        Anchor::CENTER,
        font,
    );

    let stats = match language {
        Language::Chinese => format!(
            "{} {}\n{} {}/{}\n{} {}\n{} {}\n{} {}\n{} {}\n{} {}\n{} {}\n{} {} 张",
            text(language, TextKey::StatsFloor),
            game.floor,
            text(language, TextKey::StatsHp),
            game.player.hp,
            game.player.max_hp,
            text(language, TextKey::StatsAttack),
            game.player.attack,
            text(language, TextKey::StatsDefense),
            game.player.defense,
            text(language, TextKey::StatsGold),
            game.player.gold,
            text(language, TextKey::StatsExperience),
            game.player.experience,
            text(language, TextKey::StatsYellowKeys),
            game.player.yellow_keys,
            text(language, TextKey::StatsBlueKeys),
            game.player.blue_keys,
            text(language, TextKey::StatsDeck),
            game.deck.len()
        ),
        Language::English => format!(
            "{} {}\n{} {}/{}\n{} {}\n{} {}\n{} {}\n{} {}\n{} {}\n{} {}\n{} {} cards",
            text(language, TextKey::StatsFloor),
            game.floor,
            text(language, TextKey::StatsHp),
            game.player.hp,
            game.player.max_hp,
            text(language, TextKey::StatsAttack),
            game.player.attack,
            text(language, TextKey::StatsDefense),
            game.player.defense,
            text(language, TextKey::StatsGold),
            game.player.gold,
            text(language, TextKey::StatsExperience),
            game.player.experience,
            text(language, TextKey::StatsYellowKeys),
            game.player.yellow_keys,
            text(language, TextKey::StatsBlueKeys),
            game.player.blue_keys,
            text(language, TextKey::StatsDeck),
            game.deck.len()
        ),
    };

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
        Anchor::TOP_LEFT,
        font,
    );

    let controls = format!(
        "{}\n{}\n{}\n{}\n{}\n{}",
        text(language, TextKey::ControlsTitle),
        text(language, TextKey::ControlsMove),
        text(language, TextKey::ControlsPlayCard),
        text(language, TextKey::ControlsEndTurn),
        text(language, TextKey::ControlsRestart),
        text(language, TextKey::ControlsBackToMenu)
    );
    spawn_label(
        commands,
        &controls,
        Vec3::new(190.0, -20.0, 5.0),
        20.0,
        Color::srgb(0.72, 0.78, 0.82),
        Anchor::TOP_LEFT,
        font,
    );
    spawn_label(
        commands,
        &game.message,
        Vec3::new(-470.0, -290.0, 5.0),
        22.0,
        Color::srgb(0.95, 0.88, 0.62),
        Anchor::CENTER_LEFT,
        font,
    );

    spawn_action_button(
        commands,
        Vec3::new(420.0, -285.0, 1.0),
        Vec2::new(170.0, 42.0),
        text(language, TextKey::ButtonMenu),
        font,
        ClickAction::BackToMenu,
    );
}

fn render_combat(
    commands: &mut Commands,
    game: &Game,
    language: Language,
    font: &Handle<Font>,
    hovered_card: Option<usize>,
) {
    let combat = game
        .combat
        .as_ref()
        .expect("combat mode must have combat state");

    spawn_label(
        commands,
        text(language, TextKey::CombatTitle),
        Vec3::new(-565.0, 315.0, 5.0),
        34.0,
        Color::srgb(0.92, 0.88, 0.72),
        Anchor::CENTER_LEFT,
        font,
    );

    spawn_action_button(
        commands,
        Vec3::new(490.0, 315.0, 1.0),
        Vec2::new(150.0, 40.0),
        text(language, TextKey::ButtonMenu),
        font,
        ClickAction::BackToMenu,
    );

    spawn_panel(
        commands,
        Vec3::new(-345.0, 100.0, 0.0),
        Vec2::new(330.0, 350.0),
    );
    let player_text = format!(
        "{}\n{} {}/{}\n{} {}  {} {}\n{} {}\n{} {}/{}\n{} {}\n{} {}\n{} {}  {} {}  {} {}",
        text(language, TextKey::PlayerLabel),
        text(language, TextKey::StatsHp),
        game.player.hp,
        game.player.max_hp,
        text(language, TextKey::StatsAttack),
        game.player.attack,
        text(language, TextKey::StatsDefense),
        game.player.defense,
        text(language, TextKey::BlockLabel),
        combat.player_block,
        text(language, TextKey::EnergyLabel),
        combat.energy,
        game::STARTING_ENERGY,
        text(language, TextKey::StatsGold),
        game.player.gold,
        text(language, TextKey::StatsExperience),
        game.player.experience,
        text(language, TextKey::StatsDeck),
        game.deck.len(),
        text(language, TextKey::DrawLabel),
        combat.draw_pile.len(),
        text(language, TextKey::DiscardLabel),
        combat.discard_pile.len()
    );
    spawn_label(
        commands,
        &player_text,
        Vec3::new(-485.0, 245.0, 5.0),
        22.0,
        Color::srgb(0.9, 0.94, 0.96),
        Anchor::TOP_LEFT,
        font,
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
        "{}\n{} {}/{}\n{} {}\n{}: {}",
        localization::monster_name(language, combat.monster_kind),
        text(language, TextKey::StatsHp),
        combat.enemy_hp.max(0),
        combat.enemy_max_hp,
        text(language, TextKey::BlockLabel),
        combat.enemy_block,
        text(language, TextKey::IntentLabel),
        combat.enemy_intent(language)
    );
    spawn_label(
        commands,
        &enemy_text,
        Vec3::new(230.0, 205.0, 5.0),
        24.0,
        Color::srgb(1.0, 0.97, 0.9),
        Anchor::CENTER,
        font,
    );

    spawn_label(
        commands,
        text(language, TextKey::CombatHint),
        Vec3::new(-565.0, -40.0, 5.0),
        21.0,
        Color::srgb(0.72, 0.78, 0.82),
        Anchor::CENTER_LEFT,
        font,
    );

    for (index, card) in combat.hand.iter().take(HAND_SIZE).enumerate() {
        let x = -420.0 + index as f32 * 170.0;
        spawn_card(
            commands,
            CardView {
                center: Vec3::new(x, -215.0, 1.0),
                index: index + 1,
                card: *card,
                action: ClickAction::PlayCard(index),
                highlighted: hovered_card == Some(index),
            },
            language,
            font,
        );
    }

    spawn_action_button(
        commands,
        Vec3::new(425.0, -40.0, 1.0),
        Vec2::new(190.0, 42.0),
        text(language, TextKey::ButtonEndTurn),
        font,
        ClickAction::EndTurn,
    );

    let mut log_lines = combat.log.iter().rev().take(5).cloned().collect::<Vec<_>>();
    log_lines.reverse();
    spawn_label(
        commands,
        &log_lines.join("\n"),
        Vec3::new(-565.0, 35.0, 5.0),
        18.0,
        Color::srgb(0.82, 0.85, 0.86),
        Anchor::TOP_LEFT,
        font,
    );
}

fn render_reward(commands: &mut Commands, game: &Game, language: Language, font: &Handle<Font>) {
    let reward = game
        .reward
        .as_ref()
        .expect("reward mode must have reward state");

    let defeated = match language {
        Language::Chinese => format!(
            "{}已被击败",
            localization::monster_name(language, reward.monster_kind)
        ),
        Language::English => format!(
            "{} defeated",
            localization::monster_name(language, reward.monster_kind)
        ),
    };
    spawn_label(
        commands,
        &defeated,
        Vec3::new(-350.0, 260.0, 5.0),
        34.0,
        Color::srgb(0.92, 0.88, 0.72),
        Anchor::CENTER_LEFT,
        font,
    );

    let mut summary = localization::reward_gold(language, reward.gold);
    summary.push('\n');
    summary.push_str(&localization::reward_experience(
        language,
        reward.experience,
    ));
    if let Some(bonus) = reward.boss_bonus {
        summary.push('\n');
        summary.push_str(localization::boss_bonus_text(language, bonus));
    }

    spawn_label(
        commands,
        &summary,
        Vec3::new(-350.0, 210.0, 5.0),
        24.0,
        Color::srgb(0.9, 0.94, 0.96),
        Anchor::CENTER_LEFT,
        font,
    );

    for (index, card) in reward.offers.iter().enumerate() {
        let x = -220.0 + index as f32 * 220.0;
        spawn_card(
            commands,
            CardView {
                center: Vec3::new(x, 20.0, 1.0),
                index: index + 1,
                card: *card,
                action: ClickAction::ChooseReward(index),
                highlighted: false,
            },
            language,
            font,
        );
    }

    spawn_label(
        commands,
        text(language, TextKey::RewardChooseHint),
        Vec3::new(-350.0, -170.0, 5.0),
        22.0,
        Color::srgb(0.72, 0.78, 0.82),
        Anchor::CENTER_LEFT,
        font,
    );

    spawn_action_button(
        commands,
        Vec3::new(250.0, -170.0, 1.0),
        Vec2::new(180.0, 42.0),
        text(language, TextKey::ButtonSkipReward),
        font,
        ClickAction::SkipReward,
    );
    spawn_action_button(
        commands,
        Vec3::new(465.0, -170.0, 1.0),
        Vec2::new(150.0, 42.0),
        text(language, TextKey::ButtonMenu),
        font,
        ClickAction::BackToMenu,
    );
}

fn render_victory(commands: &mut Commands, game: &Game, language: Language, font: &Handle<Font>) {
    spawn_label(
        commands,
        text(language, TextKey::VictoryTitle),
        Vec3::new(-270.0, 140.0, 5.0),
        44.0,
        Color::srgb(0.92, 0.88, 0.72),
        Anchor::CENTER_LEFT,
        font,
    );

    let summary = match language {
        Language::Chinese => format!(
            "{}\n生命 {}/{}  金币 {}  牌组 {} 张\n按 R 重新开始，Esc 返回菜单。",
            text(language, TextKey::VictoryDemoComplete),
            game.player.hp,
            game.player.max_hp,
            game.player.gold,
            game.deck.len()
        ),
        Language::English => format!(
            "{}\nHP {}/{}  Gold {}  Deck {} cards\nPress R to restart, Esc returns to menu.",
            text(language, TextKey::VictoryDemoComplete),
            game.player.hp,
            game.player.max_hp,
            game.player.gold,
            game.deck.len()
        ),
    };
    spawn_label(
        commands,
        &summary,
        Vec3::new(-270.0, 60.0, 5.0),
        26.0,
        Color::srgb(0.9, 0.94, 0.96),
        Anchor::TOP_LEFT,
        font,
    );

    spawn_action_button(
        commands,
        Vec3::new(-170.0, -150.0, 1.0),
        Vec2::new(170.0, 44.0),
        text(language, TextKey::ButtonRestart),
        font,
        ClickAction::Restart,
    );
    spawn_action_button(
        commands,
        Vec3::new(30.0, -150.0, 1.0),
        Vec2::new(170.0, 44.0),
        text(language, TextKey::ButtonMenu),
        font,
        ClickAction::BackToMenu,
    );
}

fn render_game_over(commands: &mut Commands, game: &Game, language: Language, font: &Handle<Font>) {
    spawn_label(
        commands,
        text(language, TextKey::GameOverTitle),
        Vec3::new(-180.0, 120.0, 5.0),
        48.0,
        Color::srgb(0.94, 0.42, 0.34),
        Anchor::CENTER_LEFT,
        font,
    );
    let summary = match language {
        Language::Chinese => format!(
            "到达第 {} 层，持有 {} 金币。\n按 R 重新开始，Esc 返回菜单。",
            game.floor, game.player.gold
        ),
        Language::English => format!(
            "Reached floor {} with {} gold.\nPress R to restart, Esc returns to menu.",
            game.floor, game.player.gold
        ),
    };
    spawn_label(
        commands,
        &summary,
        Vec3::new(-180.0, 45.0, 5.0),
        26.0,
        Color::srgb(0.9, 0.94, 0.96),
        Anchor::TOP_LEFT,
        font,
    );

    spawn_action_button(
        commands,
        Vec3::new(-100.0, -145.0, 1.0),
        Vec2::new(170.0, 44.0),
        text(language, TextKey::ButtonRestart),
        font,
        ClickAction::Restart,
    );
    spawn_action_button(
        commands,
        Vec3::new(100.0, -145.0, 1.0),
        Vec2::new(170.0, 44.0),
        text(language, TextKey::ButtonMenu),
        font,
        ClickAction::BackToMenu,
    );
}

fn spawn_card(commands: &mut Commands, view: CardView, language: Language, font: &Handle<Font>) {
    if view.highlighted {
        spawn_rect(
            commands,
            view.center,
            Vec2::new(CARD_W + 12.0, CARD_H + 12.0),
            Color::srgb(0.92, 0.72, 0.28),
            0.6,
        );
    }

    let card_color = if view.highlighted {
        Color::srgb(0.24, 0.27, 0.33)
    } else {
        Color::srgb(0.18, 0.20, 0.24)
    };
    let header_color = if view.highlighted {
        Color::srgb(0.40, 0.46, 0.58)
    } else {
        Color::srgb(0.30, 0.36, 0.46)
    };

    spawn_click_rect(
        commands,
        view.center,
        Vec2::new(CARD_W, CARD_H),
        card_color,
        1.0,
        view.action,
    );
    spawn_rect(
        commands,
        view.center + Vec3::new(0.0, CARD_H * 0.5 - 16.0, 1.0),
        Vec2::new(CARD_W, 28.0),
        header_color,
        2.0,
    );

    let card_text = format!(
        "{}. {} [{}]\n{}",
        view.index,
        localization::card_name(language, view.card),
        card_cost(view.card),
        wrap_card_text(localization::card_text(language, view.card))
    );
    spawn_label(
        commands,
        &card_text,
        view.center + Vec3::new(-CARD_W * 0.44, CARD_H * 0.31, 5.0),
        17.0,
        Color::srgb(0.95, 0.95, 0.9),
        Anchor::TOP_LEFT,
        font,
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
        Tile::BlueDoor => (Color::srgb(0.10, 0.28, 0.58), "D"),
        Tile::YellowKey => (Color::srgb(0.80, 0.65, 0.20), "K"),
        Tile::BlueKey => (Color::srgb(0.12, 0.34, 0.72), "K"),
        Tile::SmallPotion => (Color::srgb(0.62, 0.12, 0.20), "+"),
        Tile::Chest => (Color::srgb(0.46, 0.27, 0.12), "C"),
        Tile::Shop => (Color::srgb(0.18, 0.44, 0.48), "$"),
        Tile::Sage => (Color::srgb(0.34, 0.32, 0.58), "?"),
        Tile::Monster(index) => {
            let monster = &game.monsters[index];
            let color = match monster.rank {
                MonsterRank::Normal => Color::srgb(0.44, 0.22, 0.20),
                MonsterRank::MiniBoss => Color::srgb(0.60, 0.32, 0.12),
                MonsterRank::Boss => Color::srgb(0.58, 0.12, 0.14),
            };
            (color, monster_marker(monster.kind))
        }
    }
}

fn spawn_panel(commands: &mut Commands, center: Vec3, size: Vec2) {
    spawn_rect(commands, center, size, Color::srgb(0.10, 0.12, 0.15), 0.0);
}

fn spawn_action_button(
    commands: &mut Commands,
    center: Vec3,
    size: Vec2,
    label: &str,
    font: &Handle<Font>,
    action: ClickAction,
) {
    spawn_click_rect(
        commands,
        center,
        size,
        Color::srgb(0.22, 0.30, 0.38),
        0.0,
        action,
    );
    spawn_label(
        commands,
        label,
        center + Vec3::new(0.0, 3.0, 4.0),
        22.0,
        Color::srgb(0.96, 0.94, 0.84),
        Anchor::CENTER,
        font,
    );
}

fn spawn_rect(commands: &mut Commands, center: Vec3, size: Vec2, color: Color, z: f32) {
    commands.spawn((
        Sprite::from_color(color, size),
        Transform::from_translation(center + Vec3::new(0.0, 0.0, z)),
        SceneEntity,
    ));
}

fn spawn_click_rect(
    commands: &mut Commands,
    center: Vec3,
    size: Vec2,
    color: Color,
    z: f32,
    action: ClickAction,
) {
    commands.spawn((
        Sprite::from_color(color, size),
        Transform::from_translation(center + Vec3::new(0.0, 0.0, z)),
        ClickArea { size, action },
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
    font: &Handle<Font>,
) {
    commands.spawn((
        Text2d::new(value.to_string()),
        TextFont::from_font_size(font_size).with_font(font.clone()),
        TextColor(color),
        TextLayout::new_with_justify(justify_for_anchor(anchor)),
        anchor,
        Transform::from_translation(position),
        SceneEntity,
    ));
}

fn justify_for_anchor(anchor: Anchor) -> Justify {
    if anchor == Anchor::CENTER || anchor == Anchor::TOP_CENTER || anchor == Anchor::BOTTOM_CENTER {
        Justify::Center
    } else if anchor == Anchor::CENTER_RIGHT
        || anchor == Anchor::TOP_RIGHT
        || anchor == Anchor::BOTTOM_RIGHT
    {
        Justify::Right
    } else {
        Justify::Left
    }
}
