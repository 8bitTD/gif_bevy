use bevy::prelude::*;
use std::io::Write;
use super::asset::*;

#[derive(Debug)]
pub struct GifInfo{
    pub width: f32,
    pub height: f32,
    pub gif_frame: usize,
    pub gif_delay: f32,
    pub unique_id: usize,
    pub image_infos: Vec<ImageInfo>,
}
impl Default for GifInfo{
    fn default() -> Self{
        GifInfo {
            width: 0.0,
            height: 0.0,
            gif_frame: 0,
            gif_delay: 0.0,
            unique_id: 0,
            image_infos: Vec::new(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct ImageInfo{
    pub image: Image,
    pub frame: usize,
    pub delay: f32,
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

pub fn get_gif(url: &str, unique_sprite_id: &mut usize) -> Vec<ImageInfo>{
    let rename_url = save_gif(url);
    if rename_url.is_empty(){return Vec::new();}
    let input = std::fs::File::open(&rename_url).unwrap();    
    let mut options = gif::DecodeOptions::new();
    options.set_color_output(gif::ColorOutput::Indexed);
    let mut decoder = options.read_info(input).unwrap();
    let mut screen = gif_dispose::Screen::new_decoder(&decoder);
    let mut image_infos = Vec::new();
    let mut frame = 0;
    while let Some(f) = decoder.read_next_frame().unwrap() {
        screen.blit_frame(f).unwrap();
        let mut data = Vec::new();
        let delay = f.delay as f32 * 0.01;
        for c in screen.pixels_rgba(){
            data.push(c.b);
            data.push(c.g);
            data.push(c.r);
            data.push(c.a);
        }
        let width = screen.pixels_rgba().width();
        let height = screen.pixels_rgba().height();
        let image = Image {
            data: Some(data),
            texture_descriptor: bevy::render::render_resource::TextureDescriptor {
                label: None,
                size:bevy::render::render_resource::Extent3d {
                    width: width as u32,
                    height: height as u32,
                    depth_or_array_layers: 1,
                },
                dimension: bevy::render::render_resource::TextureDimension::D2,
                format: bevy::render::render_resource::TextureFormat::Bgra8UnormSrgb,
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
        let image_info = ImageInfo{
            image: image,
            frame: frame,
            delay: delay,
        };
        image_infos.push(image_info);
        *unique_sprite_id += 1;
        frame += 1;
    }
    return image_infos;
}

pub fn spawn_gif(
    commands: &mut Commands,
    images: &mut ResMut<Assets<Image>>,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    unique_id: usize,
    width: f32,
    height: f32,
    image_infos: Vec<ImageInfo>,
){
    commands.spawn((
        Text2d::new(""),
        Transform::from_xyz(0.0, 0.0, 0.0),
        ParentInfo{
            unique_id: unique_id,
        },
    )).with_children(|parent| {
        for img in image_infos.iter(){
            parent.spawn((
                Sprite{
                    image: images.add(img.image.clone()),
                    color: Color::srgba( 1.0, 1.0, 1.0, 1.0 ),
                    ..default()
                },
                Transform::from_scale(Vec3::new(1.0,1.0,1.0)),
                SpriteInfo{
                    frame: img.frame,
                    delay: img.delay,
                    unique_id: unique_id,
                }
            ));
        }
        parent.spawn((
            Mesh2d(meshes.add(Rectangle::new(width, height))),
            MeshMaterial2d(materials.add(Color::srgba( 0.0, 1.0, 0.5, 0.15 ))),
            Transform::from_scale(Vec3::new(1.0,1.0,1.0)).with_translation(Vec3::new(0.0,0.0,-1.0)),
            BackgroundInfo{
                unique_id: unique_id
            }
        ));
    });
}