use bevy::prelude::*;
use super::app;
use super::gif_file;

#[derive(Component)]
pub struct SpriteInfo{
    pub frame: usize,
    pub delay: f32,
    pub unique_id: usize,
}
#[derive(Component)]
pub struct ParentInfo{
    pub unique_id: usize,
}

#[derive(Component)]
pub struct BackgroundInfo{
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
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
){
    if app.gui.state != app::State::InitSpawn{return;}
    let all_num = app.gui.gif_info.len();
    for u in 0..all_num{
        let mut tmp_id = 0;
        let image_infos = gif_file::get_gif(&app.json.gif_jsons[u].url, &mut tmp_id);
        let mut gif_info = gif_file::GifInfo::default();
        gif_info.unique_id = app.gui.gif_info[u].unique_id;
        let mut tmp_images = Vec::new();
        let mut frames = Vec::new();
        for (gi, g) in image_infos.iter().enumerate(){
            if gi == image_infos.len(){continue;}
            tmp_images.push(g.clone());
            frames.push(gi);
            if gi == 0{
                gif_info.width = g.image.size().x as f32;
                gif_info.height = g.image.size().y as f32;
            }
        }
        gif_info.image_infos = image_infos;
        app.gui.gif_info[u] = gif_info;
        gif_file::spawn_gif(
            &mut commands, &mut images, &mut meshes, &mut materials, app.gui.gif_info[u].unique_id, 
            app.gui.gif_info[u].width, app.gui.gif_info[u].height, app.gui.gif_info[u].image_infos.clone()
        );
    }
    app.gui.current_usize = 0;
    app.gui.state = app::State::Idle;

}

pub fn spawn_asset(
    mut commands: Commands,
    mut app: ResMut<app::MyApp>,
    mut images: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut parents: Query<(Entity, &ParentInfo)>,
    mut sprites: Query<(Entity, &SpriteInfo)>,
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
        &mut commands, &mut images, &mut meshes, &mut materials, app.gui.current_unique_id, app.gui.gif_info[app.gui.current_usize].width, 
        app.gui.gif_info[app.gui.current_usize].height, app.gui.gif_info[app.gui.current_usize].image_infos.clone()
    );
    app.gui.state = app::State::Idle;
}

pub fn update_gif(//Gifアニメーション処理
    mut app: ResMut<app::MyApp>,
    time: Res<Time>,
    mut sprites: Query<(&mut SpriteInfo, &mut Visibility) ,Without<ParentInfo>>,
){
    if app.gui.state != app::State::Idle || app.gui.gif_info.is_empty() || app.json.gif_jsons.is_empty(){return;}
    let dt = time.delta_secs();
    let gif_jsons = app.json.gif_jsons.clone();
    for (u, gi) in app.gui.gif_info.iter_mut().enumerate(){
        gi.gif_delay += dt * gif_jsons[u].delay;
    }
    for (si,mut _v) in sprites.iter_mut(){
        if let Some(u) = app.gui.gif_info.iter().position(|g|g.unique_id == si.unique_id){
            if si.delay <= app.gui.gif_info[u].gif_delay{
                app.gui.gif_info[u].gif_delay = app.gui.gif_info[u].gif_delay - si.delay;
                app.gui.gif_info[u].gif_frame += 1;
                if app.gui.gif_info[u].gif_frame == app.gui.gif_info[u].image_infos.len(){
                    app.gui.gif_info[u].gif_frame = 0;
                }
            }
        }
    } 
    for (si,mut v) in sprites.iter_mut(){
        if let Some(u) = app.gui.gif_info.iter().position(|g|g.unique_id == si.unique_id){
            match app.gui.gif_info[u].gif_frame == si.frame{
                true => *v = Visibility::Visible,
                _ =>    *v = Visibility::Hidden,
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

pub fn update_background(
    app: Res<app::MyApp>,
    mut bgs: Query<(&mut BackgroundInfo, &mut Visibility)>,
){
    if app.gui.state != app::State::Idle || app.gui.gif_info.is_empty() || app.json.gif_jsons.is_empty(){return;}
    if app.gui.hover_unique_id.is_none() || !app.gui.is_show_window{
        for (_, mut v) in bgs.iter_mut(){
            *v = Visibility::Hidden;
        }
    }else{
        for (b, mut v) in bgs.iter_mut(){
            match b.unique_id == app.gui.hover_unique_id.unwrap(){
                true => *v = Visibility::Visible,
                _ =>    *v = Visibility::Hidden,
            };
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