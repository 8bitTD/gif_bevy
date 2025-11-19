//#![windows_subsystem = "windows"]
use bevy::{
    prelude::*,
    window::*,
};
use bevy_egui::EguiPlugin;

mod app;
mod asset;
mod define;
mod gif_file;
mod ui;

fn main() {
    let json = app::Json::load_json();

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: define::common::TOOLNAME.into(),
                position: WindowPosition::new(IVec2::new(json.window_info.left, json.window_info.top)),
                resolution: WindowResolution::new(json.window_info.width, json.window_info.height),
                transparent: true,
                window_level: WindowLevel::Normal,
                present_mode: PresentMode::AutoNoVsync,
                prevent_default_event_handling: false,
                 ..default()
            }),
            exit_condition: bevy::window::ExitCondition::OnAllClosed,
            close_when_requested: true,
            ..default()
        }))
        .add_plugins(EguiPlugin::default())
        .insert_resource(app::MyApp{gui: app::Gui::new(&json.gif_jsons), json: json}) 
        .add_systems(Startup, asset::setup_asset)
        .add_systems(bevy_egui::EguiPrimaryContextPass, ui::ui_system)
        .add_systems(PreUpdate,asset::init_spawn)
        .add_systems(Update,
            (
                asset::set_window_icon,
                asset::spawn_asset,
                asset::update_gif,
                asset::update_transform,
                asset::update_flip_x,
                asset::update_menu,
                asset::update_window,
            )
        )
        .add_systems(PostUpdate, 
            (
                asset::delete_asset, 
            )
        )
        .add_systems(Last,exec_exit)
        .run();
}

fn exec_exit(//ウィンドウ終了時の処理
    mut reader: MessageReader<WindowCloseRequested>,
    mut app: ResMut<app::MyApp>, 
    windows: Query<&mut Window>, 
){
    if reader.read().next().is_some() {
        app.json.save_json(windows);
    }
}
