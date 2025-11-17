use bevy::prelude::*;
use bevy_egui::{EguiContexts, egui};

use super::app;
use super::gif_file;

pub fn ui_system (
    mut contexts: EguiContexts, 
    mut app: ResMut<app::MyApp>, 
    mut windows: Query<&mut Window>
) -> Result{
    if !app.gui.is_show_window{return Ok(())}
    if app.gui.state != app::State::Idle {return Ok(());}
    if app.gui.is_init_ui{
        if let Ok(mut window) = windows.single_mut(){
            window.ime_enabled = true;
        };
        let mut txt_font = egui::FontDefinitions::default();
        txt_font.families.get_mut(&egui::FontFamily::Proportional).unwrap().insert(0, "Meiryo".to_owned());
        let fd = egui::FontData::from_static(include_bytes!("C:/Windows/Fonts/Meiryo.ttc"));
        txt_font.font_data.insert("Meiryo".to_owned(), std::sync::Arc::new(fd));
        if let Ok(context) = contexts.ctx_mut(){
            context.set_fonts(txt_font);
        }
        app.gui.is_init_ui = false;
    }
    let mut is_open_modal = app.gui.is_open_modal;
    let mut current_unique_id = app.gui.current_unique_id;
    let mut current_usize = app.gui.current_usize;
    let mut gj = None;
    let mut rm = None;
    let mut hover_unique_id = None;
    let mut is_show_menu = app.gui.is_show_menu;
    egui::Window::new("設定").open(&mut is_show_menu).constrain(false).max_width(200.0).default_pos(egui::Pos2::new(0.0,0.0)).show(contexts.ctx_mut()?, |ui| {
        for (u, g) in app.json.gif_jsons.iter_mut().enumerate(){
            ui.horizontal(|ui|{
                let res = ui.button("URL").on_hover_text(&g.url);
                if res.clicked(){ 
                    is_open_modal = true; 
                    current_unique_id = g.unique_id;
                    current_usize = u;
                }
                if res.hovered(){
                    hover_unique_id = Some(g.unique_id);
                }
                if ui.add(egui::DragValue::new(&mut g.pos_x).range(-1000.0..=1000.00).speed(5.00)).on_hover_text("pos_x").hovered(){
                    hover_unique_id = Some(g.unique_id);
                }
                if ui.add(egui::DragValue::new(&mut g.pos_y).range(-1000.0..=1000.00).speed(5.00)).on_hover_text("pos_y").hovered(){
                    hover_unique_id = Some(g.unique_id);
                }
                if ui.add(egui::DragValue::new(&mut g.pos_z).range(0.0..=1000.00).speed(5.00)).on_hover_text("pos_z").hovered(){
                    hover_unique_id = Some(g.unique_id);
                }
                if ui.add(egui::DragValue::new(&mut g.scale).range(0.001..=20.00).speed(0.025)).on_hover_text("scale").hovered(){
                    hover_unique_id = Some(g.unique_id);
                }
                if ui.add(egui::DragValue::new(&mut g.speed).range(0.10..=5.00).speed(0.025)).on_hover_text("speed").hovered(){
                    hover_unique_id = Some(g.unique_id);
                }
                if ui.checkbox(&mut g.flip_x,"").on_hover_text("x_flip").hovered(){
                    hover_unique_id = Some(g.unique_id);
                }
                let res = ui.button("-").on_hover_text("削除");
                if res.clicked(){
                    rm = Some(g.unique_id);
                }
                if res.hovered(){
                    hover_unique_id = Some(g.unique_id);
                }
            });
        }
        ui.vertical_centered(|ui|{
            if ui.button("+").on_hover_text("追加").clicked(){ 
                gj = Some(app::GifJson::new(app.gui.unique_gif_id));                
            }
        });
    });
    if gj.is_some(){ 
        let tmp_gj = gj.unwrap();
        let mut gi = gif_file::GifInfo::default();
        gi.unique_id = tmp_gj.unique_id;
        app.json.gif_jsons.push(tmp_gj); 
        app.gui.gif_info.push(gi);
        app.gui.unique_gif_id += 1;
    }
    if rm.is_some(){ 
        app.gui.remove_unique_id = rm;
        app.gui.state = app::State::Delete;
    }
    if app.gui.is_open_modal {
        let modal = egui::Modal::new(egui::Id::new("Modal URL")).show(contexts.ctx_mut()?, |ui| {
            let cu = app.json.gif_jsons.iter().position(|gj|gj.unique_id == current_unique_id).unwrap();
            ui.add_sized(
                egui::Vec2::new(600.0, 20.0), 
                egui::TextEdit::singleline(&mut app.json.gif_jsons[cu].url)
            );
            ui.separator();
            egui::Sides::new().show(
                ui,
                |_ui| {},
                |ui| {
                    if ui.button("Cancel").clicked() {
                        ui.close();
                    }
                    if ui.button("読み込み").clicked(){
                        app.load_gif(current_usize, current_unique_id);
                        ui.close();
                    }
                });
        });
        if modal.should_close() {
            is_open_modal = false;
        }
    }
    app.gui.current_usize = current_usize;
    app.gui.is_open_modal = is_open_modal;
    app.gui.current_unique_id = current_unique_id;
    app.gui.hover_unique_id = hover_unique_id;
    app.gui.is_show_menu = is_show_menu;
    Ok(())
}