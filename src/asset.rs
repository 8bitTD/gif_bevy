use bevy::prelude::*;
use super::app;
use super::gif_file;

#[derive(Component)]
pub struct SpriteInfo{
    pub first: usize,
    pub last: usize,
    pub current_index: usize,
    pub timer: f32,
    pub delays: Vec<f32>,
    pub unique_id: usize,
}
#[derive(Component)]
pub struct ParentInfo{
    pub unique_id: usize,
}

pub fn setup_asset(
    mut commands: Commands,
) {
    commands.spawn((
        Camera2d::default(),
    ));
}

pub fn delete_asset(
    mut commands: Commands,
    mut app: ResMut<app::MyApp>,
    mut parents: Query<(Entity, &ParentInfo)>,
    mut sprites: Query<(Entity, &SpriteInfo)>,
){
    if app.gui.state != app::State::Delete{return;}
    if let Some(id) = app.gui.remove_unique_id{
        for (entity, pi) in parents.iter_mut(){
            if pi.unique_id != id{continue;}
            commands.entity(entity).despawn();
        }
        for (entity, si) in sprites.iter_mut(){
            if si.unique_id != id{continue;}
            commands.entity(entity).despawn();
        }
        if let Some(u) = app.gui.gif_info.iter().position(|g|g.unique_id == id){
            app.gui.gif_info.remove(u);
            app.json.gif_jsons.remove(u); 
        }
        app.gui.remove_unique_id = None;
    }
    app.gui.state = app::State::Idle;
}

pub fn init_spawn(
    mut app: ResMut<app::MyApp>,
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
){
    if app.gui.state != app::State::InitSpawn{return;}
    let all_num = app.gui.gif_info.len();
    for u in 0..all_num{
        let mut tmp_id = 0;
        let gif_atlas = gif_file::get_gif(&app.json.gif_jsons[u].url, &mut tmp_id);
        let mut gif_info = gif_file::GifInfo::default();
        gif_info.unique_id = app.gui.gif_info[u].unique_id;
  
        gif_info.gif_atlas = gif_atlas;
        app.gui.gif_info[u] = gif_info;
        gif_file::spawn_gif(
            &mut commands, &mut images, &mut texture_atlas_layouts, app.gui.gif_info[u].clone()
        );
    }
    app.gui.current_usize = 0;
    app.gui.state = app::State::Idle;
}

pub fn spawn_asset(
    mut commands: Commands,
    mut app: ResMut<app::MyApp>,
    mut images: ResMut<Assets<Image>>,
    mut parents: Query<(Entity, &ParentInfo)>,
    mut sprites: Query<(Entity, &SpriteInfo)>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    if app.gui.state != app::State::Spawn {return;}
    for (entity, pi) in parents.iter_mut(){
        if pi.unique_id != app.gui.current_unique_id{continue;}
        commands.entity(entity).despawn();
    }
    for (entity, si) in sprites.iter_mut(){
        if si.unique_id != app.gui.current_unique_id{continue;}
        commands.entity(entity).despawn();
    }
    gif_file::spawn_gif(
        &mut commands, &mut images, &mut texture_atlas_layouts, app.gui.gif_info[app.gui.current_usize].clone()
    );
    app.gui.state = app::State::Idle;
}

pub fn update_gif(//Gifアニメーション処理
    app: Res<app::MyApp>,
    time: Res<Time>,
    mut sprites: Query<(&mut SpriteInfo, &mut Sprite)>,
){
    if app.gui.state != app::State::Idle || app.gui.gif_info.is_empty() || app.json.gif_jsons.is_empty(){return;}
    for (mut si, mut s) in &mut sprites{
        if app.json.gif_jsons.iter().position(|g|g.unique_id == si.unique_id).is_none(){continue;}
        if let Some(atlas) = &mut s.texture_atlas{
            let u = app.json.gif_jsons.iter().position(|g|g.unique_id == si.unique_id).unwrap();
            si.timer += time.delta_secs() * app.json.gif_jsons[u].speed;
            let target_time = si.delays[si.current_index];
            if si.timer >= target_time{
                si.timer = si.timer - target_time;
                atlas.index = match atlas.index == si.last{
                    true => si.first,
                    _ =>    atlas.index + 1
                };
            }
        }
        if app.gui.hover_unique_id.is_none() || !app.gui.is_show_menu || !app.gui.is_show_window{
            s.color = Color::WHITE;
        }else{
            let id = app.gui.hover_unique_id.unwrap();
            match id == si.unique_id{
                true => { s.color = Color::srgba(0.25,0.75,1.0, 1.0); },
                _ =>    { s.color = Color::WHITE; },
            };
        }
    }
}

pub fn update_transform(
    app: Res<app::MyApp>,
    mut parents: Query<(&mut Transform, &ParentInfo)>,
){
    if app.gui.state != app::State::Idle || app.gui.gif_info.is_empty() || app.json.gif_jsons.is_empty(){return;}
    for (mut t, p) in parents.iter_mut(){
        let uid = p.unique_id;
        if let Some(u) = app.json.gif_jsons.iter().position(|gf|gf.unique_id == uid){
            t.translation = Vec3::new(app.json.gif_jsons[u].pos_x, app.json.gif_jsons[u].pos_y, app.json.gif_jsons[u].pos_z);
            t.scale = Vec3::new(app.json.gif_jsons[u].scale, app.json.gif_jsons[u].scale, app.json.gif_jsons[u].scale);
        }
    }
}

pub fn update_flip_x(
    app: Res<app::MyApp>,
    mut gifs: Query<(&mut SpriteInfo, &mut Sprite)>,
){
    if app.gui.state != app::State::Idle || app.gui.gif_info.is_empty(){return;}
    for ( si, mut s) in gifs.iter_mut(){
        let uid = si.unique_id;
        if let Some(u) = app.json.gif_jsons.iter().position(|gf|gf.unique_id == uid){
            s.flip_x = app.json.gif_jsons[u].flip_x;
        }
    }
}

pub fn update_menu(
    mut app: ResMut<app::MyApp>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
){
    if keyboard_input.just_released(KeyCode::Escape) {
        app.gui.is_show_menu = !app.gui.is_show_menu;
    }
}

pub fn update_window(
    mut app: ResMut<app::MyApp>,
    mut windows: Query<&mut Window>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut clear_color: ResMut<ClearColor>
){
    if keyboard_input.just_released(KeyCode::F12) {//フルスクリーン
        app.gui.is_show_window = !app.gui.is_show_window;
    }
    let Ok(mut win) = windows.single_mut() else {return};
    win.decorations = app.gui.is_show_window;
    match app.gui.is_show_window{
        true => clear_color.0 = Color::srgba(0.2, 0.2, 0.2, 1.0),
        _ =>    clear_color.0 = Color::srgba(0.0, 0.0, 0.0, 0.0),
    };
}