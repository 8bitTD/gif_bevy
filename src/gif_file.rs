use bevy::prelude::*;
use std::io::Write;
use super::asset::*;
use super::define::*;

#[derive(Debug, Clone)]
pub struct GifInfo{
    pub unique_id: usize,
    pub gif_atlas: Option<GifAtlas>,
}
impl Default for GifInfo{
    fn default() -> Self{
        GifInfo {
            unique_id: 0,
            gif_atlas: None,
        }
    }
}
#[derive(Debug, Clone)]
pub struct GifAtlas{
    pub image: Image,
    pub delays: Vec<f32>,
    pub width: u32,
    pub height: u32,
    pub column: u32,
    pub row: u32,
}

#[derive(Debug, Clone)]
pub struct ImageInfo{
    pub lines: Vec<Vec<u8>>,
    pub width: u32,
    pub atlas_width: u32,
    pub height: u32,
    pub column: u32,
    pub row: u32,
}
impl ImageInfo{
    pub fn new(lines: Vec<Vec<u8>>, width: u32, height: u32, column :u32, row: u32) -> ImageInfo{
        ImageInfo { 
            lines: lines, 
            width: width,
            atlas_width: width,
            height: height, 
            column: column, 
            row: row, 
        }
    }
    pub fn set_atlas_width(&mut self, atlas_width: u32){
        self.atlas_width = atlas_width;
    }
    pub fn set_datas(&self, datas: &mut Vec<u8>){
    let column = self.column as usize;
    let row = self.row as usize;
    let width = self.width as usize;
    let height = self.height as usize;
    let atlas_width = self.atlas_width as usize;
    for (uh, line) in self.lines.iter().enumerate(){
        for (wu, &v) in line.iter().enumerate(){
            let n = ((column * width * 4) + wu) + (uh * atlas_width * 4) + (atlas_width * 4 * height * row);
            datas[n] = v;
        }
    }
}
}

pub fn save_gif(url: &str) -> String{
    if !std::path::Path::new("gif").is_dir(){Some(std::fs::create_dir_all("gif"));}
    let rename_url = format!("{}/{}","gif", url.replace("/","_").replace(":","_"));
    if !std::path::Path::new(&rename_url).is_file(){
        let Ok(response) = reqwest::blocking::get(url) else {return String::new()};
        let Ok(bytes) = response.bytes() else {return String::new()};
        let mut file = std::fs::File::create(&rename_url).unwrap();
        let _ = file.write_all(&bytes);
    }
    return rename_url;
}

pub fn get_gif(url: &str, unique_sprite_id: &mut usize) -> Option<GifAtlas>{
    let rename_url = save_gif(url);
    if rename_url.is_empty(){return None;}
    let Ok(input) = std::fs::File::open(&rename_url) else {return None};    
    let mut options = gif::DecodeOptions::new();
    options.set_color_output(gif::ColorOutput::Indexed);
    let Ok(mut decoder) = options.read_info(input) else {return None};
    let mut screen = gif_dispose::Screen::new_decoder(&decoder);
    let mut width = 0;
    let mut height = 0;
    let mut atlas_width = 0;
    let mut count_limit_width = 0;
    let mut count_column = 0;
    let mut column = 0;
    let mut row = 0;
    let mut delays = Vec::new();
    let mut image_infos = Vec::new();
    while let Ok(Some(f)) = decoder.read_next_frame() {
        screen.blit_frame(f).unwrap();
        width = screen.pixels_rgba().width() as u32;
        height = screen.pixels_rgba().height() as u32;
        if atlas_width + width <= common::LIMITWIDTH{
            atlas_width += width;
            column += 1;
        }
        let delay = f.delay as f32 * 0.01;
        delays.push(delay);
        let mut lines = Vec::new();
        let mut line = Vec::new();
        let mut count_width = 0;
        for c in screen.pixels_rgba(){
            line.push(c.r);
            line.push(c.g);
            line.push(c.b);
            line.push(c.a);
            count_width += 1;
            if count_width == width{
                count_width = 0;
                lines.push(line);
                line = Vec::new();
            }
        }
        let image_info = ImageInfo::new(lines, width, height, count_column, row);
        image_infos.push(image_info);
        if count_limit_width + width <= common::LIMITWIDTH{
            count_limit_width += width;
            count_column += 1;
        }else {
            count_limit_width = 0;
            count_column = 0;
            row += 1;
        }
    }
    let atlas_height = height * (row+1);
    let all_num = atlas_width as usize * atlas_height as usize * 4;
    let mut datas = vec![0; all_num];
    for i in image_infos.iter_mut(){
        i.set_atlas_width(atlas_width);
        i.set_datas(&mut datas);
    }
    let image = Image {
        data: Some(datas),
        texture_descriptor: bevy::render::render_resource::TextureDescriptor {
            label: None,
            size:bevy::render::render_resource::Extent3d {
                width: atlas_width,
                height: atlas_height,
                depth_or_array_layers: 1,
            },
            dimension: bevy::render::render_resource::TextureDimension::D2,
            format: bevy::render::render_resource::TextureFormat::Rgba8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: bevy::render::render_resource::TextureUsages::TEXTURE_BINDING
                | bevy::render::render_resource::TextureUsages::COPY_DST
                | bevy::render::render_resource::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[], 
        },
        sampler: bevy::image::ImageSampler::nearest(),
        ..default()
    };
    let gif_atlas = GifAtlas{
        image: image,
        delays: delays,
        width: width,
        height: height,
        column: column,
        row: row+1
    };
    *unique_sprite_id += 1;
    return Some(gif_atlas);
}

pub fn spawn_gif(
    commands: &mut Commands,
    images: &mut ResMut<Assets<Image>>,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    gif_info: GifInfo,
){
    if gif_info.gif_atlas.is_none(){return;}
    let gif_atlas = gif_info.gif_atlas.unwrap();
    let all_num = gif_atlas.delays.len();
    let layout = 
        TextureAtlasLayout::from_grid(
            UVec2::new(gif_atlas.width, gif_atlas.height), 
            gif_atlas.column, gif_atlas.row, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let (first, last) = match gif_atlas.row == 1 {
        true => {(0 as usize, all_num-1)},
        _ =>    {(1 as usize, all_num-2)},
    };
    commands.spawn((
        Text2d::new(""),
        Transform::from_xyz(0.0, 0.0, 0.0),
        ParentInfo{
            unique_id: gif_info.unique_id,
        },
    )).with_children(|parent| {
        parent.spawn((
            Sprite::from_atlas_image(
            images.add(gif_atlas.image),
            TextureAtlas {
                    layout: texture_atlas_layout,
                    index: 0,
                },
            ),
            Transform::from_scale(Vec3::new(1.0,1.0,1.0)),            
            SpriteInfo{
                first: first,
                last: last,
                current_index: 0,
                timer: 0.0,
                delays: gif_atlas.delays,
                unique_id: gif_info.unique_id,
            }
        ));
    });
}