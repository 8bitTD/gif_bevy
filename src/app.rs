use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::io::prelude::*;
use super::define::*;
use super::gif_file;

#[derive(Resource)] 
pub struct MyApp{
    pub gui: Gui,
    pub json: Json,
}
impl Default for MyApp{
    fn default() -> Self{
        MyApp{
            gui: Gui::default(),
            json: Json::default(),
        }
    }
}
impl MyApp{
    pub fn load_gif(&mut self, current_usize: usize, unique_id: usize){
        let gif_atlas = gif_file::get_gif(&self.json.gif_jsons[current_usize].url, &mut self.gui.unique_sprite_id);
        let mut gif_info = gif_file::GifInfo::default();
        gif_info.unique_id = unique_id;
        gif_info.gif_atlas = gif_atlas;
        self.gui.gif_info[current_usize] = gif_info;
        if self.gui.state == State::Idle{
            self.gui.state = State::Spawn;
        }
    }
}

#[derive(Debug)] 
pub struct Gui {
    pub is_init_ui: bool,
    pub gif_info: Vec<gif_file::GifInfo>,
    pub remove_unique_id: Option<usize>,
    pub current_usize: usize,
    pub current_unique_id: usize,
    pub unique_gif_id: usize,
    pub unique_sprite_id: usize,
    pub state: State,
    pub is_show_window: bool,
    pub is_show_menu: bool,
    pub is_open_modal: bool,
    pub hover_unique_id: Option<usize>,
}
impl Default for Gui{
    fn default() -> Self{
        Gui {
            is_init_ui: true,
            is_show_menu: true,
            gif_info: vec![],
            remove_unique_id: None,
            current_usize: 0,
            current_unique_id: 0,
            unique_gif_id: 0,
            unique_sprite_id: 0,
            state: State::InitSpawn,
            is_show_window: true,
            is_open_modal: false,
            hover_unique_id: None,
        }
    }
}
impl Gui{
    pub fn new(gif_json: &Vec<GifJson>) -> Gui{
        let mut gui = Gui::default();
        let num = gif_json.len();
        let mut unique_gif_id = 0;
        for u in 0..num{
            if unique_gif_id < gif_json[u].unique_id{
                unique_gif_id = gif_json[u].unique_id;
            }
            let mut gi = gif_file::GifInfo::default();
            gi.unique_id = gif_json.get(u).unwrap().unique_id;
            gui.gif_info.push(gi);
        }
        gui.unique_gif_id = unique_gif_id + 1;
        println!("{:?}", gui);
        return gui;
    }
}

#[derive(Debug, PartialEq)] 
pub enum State{
    Idle,
    Delete,
    InitSpawn,
    Spawn,
}

#[derive(Debug, Deserialize, Serialize, Clone)] 
pub struct GifJson {
    pub url: String,
    pub pos_x: f32,
    pub pos_y: f32,
    pub pos_z: f32,
    pub scale: f32,
    pub speed: f32,
    pub flip_x: bool,
    pub unique_id: usize,
}
impl Default for GifJson{
    fn default() -> Self{
        GifJson {
            url: String::new(),
            pos_x: 0.0,
            pos_y: 0.0,
            pos_z: 0.0,
            scale: 1.0,
            speed: 1.0,
            flip_x: false,
            unique_id: 0,
        }
    }
}
impl GifJson{
    pub fn new(unique_id: usize) -> GifJson{
        let mut gj = GifJson::default();
        gj.unique_id = unique_id;
        return gj;
    }
}

#[derive(Debug, Deserialize, Serialize)] 
pub struct Json {
    pub gif_jsons: Vec<GifJson>,
    pub window_info: WindowInfo,
}
impl Default for Json{
    fn default() -> Self{
        Json {
            gif_jsons: Vec::new(),
            window_info: WindowInfo::default(),
        }
    }
}
impl Json{
    pub fn load_json() -> Json{
        let mut jsn_path: String = dirs::home_dir().unwrap().as_os_str().to_str().unwrap().to_string();
        let rust_path = format!("{}{}",&jsn_path, common::DOCUMENT);
        if !std::path::Path::new(&rust_path).is_dir(){Some(std::fs::create_dir_all(&rust_path));}
        jsn_path.push_str(format!("{}{}{}", common::DOCUMENT, common::TOOLNAME,".json").as_str());
        let contents = match std::fs::read_to_string(&jsn_path) {                                                
            Ok(contents) => contents,                                                  
            Err(_error) => { String::from("") },                                                                 
        }; 
        let res: Result<Json,_> = serde_json::from_str(&contents);
        if res.is_ok(){ return res.unwrap(); }
        return Json::default();
    }
    pub fn save_json(&mut self, windows: Query<&mut Window>){
        let Ok(res) = windows.single() else {return};
        let wpt =  res.position;
        let wps = match wpt{
            WindowPosition::At(e) => {(e.x, e.y)},
            _ => {(100, 100)}
        };
        let wrs = &res.resolution;
        let left = wps.0 as f32 * wrs.scale_factor() as f32;
        let top = wps.1 as f32 * wrs.scale_factor() as f32;
        let width = wrs.width() * wrs.scale_factor() as f32;
        let height = wrs.height() * wrs.scale_factor() as f32;
        let wi = WindowInfo{
            left: left as i32,
            top: top as i32,
            width: width as u32,
            height: height as u32,
        };
        self.window_info = wi;
        let content = serde_json::to_string_pretty(&self).unwrap();        
        let mut jsn_path: String = dirs::home_dir().unwrap().as_os_str().to_str().unwrap().to_string();
        let path = std::path::Path::new(&jsn_path);
        if !path.is_dir(){Some(std::fs::create_dir_all(path));}
        jsn_path.push_str(format!("{}{}{}",common::DOCUMENT,common::TOOLNAME,".json").as_str());
        let mut file = std::fs::File::create(&jsn_path).expect("create failed");
        file.write_all(content.as_bytes()).unwrap();
    }
}

#[derive(Debug, Deserialize, Serialize)] 
pub struct WindowInfo {
    pub left: i32,
    pub top: i32,
    pub width: u32,
    pub height: u32,
}
impl Default for WindowInfo{
    fn default() -> WindowInfo{
        WindowInfo { 
            left: 600, 
            top: 200, 
            width: 900, 
            height: 600 
        }
    }
}