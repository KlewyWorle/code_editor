use eframe::egui::{self, TextBuffer};
use std::{borrow::{Borrow, BorrowMut}, fs::{File, OpenOptions}, future::IntoFuture, io::{Read, Write}};
use egui_code_editor::{CodeEditor, ColorTheme, Syntax};

use crate::app;


pub fn text_edit_setup(ui : &mut egui::Ui, app : &mut app::MyApp)
{
    egui::ScrollArea::new(true).show(ui, |ui|
        {
            let mut layouter = |ui: &egui::Ui, string: &str, wrap_width: f32| {
                let mut layout_job =
                    egui_extras::syntax_highlighting::highlight(ui.ctx(), &egui_extras::syntax_highlighting::CodeTheme::dark(), string, &app.language);
                
                for i in &mut layout_job.sections
                {
                    i.format.font_id.size = app.font_size;
                }
                layout_job.wrap.max_width = wrap_width;
                ui.fonts(|f| f.layout_job(layout_job))
            };

            let t_edit = CodeEditor::default()
            .id_source("code editor")
            .with_rows((ui.available_height() / 15.0) as usize)
            .with_fontsize(app.font_size)
            //.with_theme(ColorTheme::AYU_DARK)
            .with_syntax(Syntax::rust())
            .with_numlines(true)
            .show(ui, &mut app.text);

            // let t_edit = egui::TextEdit::multiline(&mut app.text).hint_text("Type...")
            // .min_size(egui::vec2(ui.available_width(), ui.available_height()))
            // .desired_width(ui.available_width())
            // .code_editor()
            // .layouter(&mut layouter)
            // .show(ui); //Use it so it returns TextEditOutput instead of response


            // Finding where the cursor is for {} () stuff
            if let Some(text) = t_edit.cursor_range
            {
                app.cursor_index = text.primary.ccursor.index;
            }
            
        
            
            
        });
}

pub fn text_conv(ui : &egui::Ui, app : &mut app::MyApp)
{
    if ui.input(|i|  i.events.contains(&egui::Event::Text("{".to_owned()))) //There is no { in egui::Key
        {   
            if app.cursor_index == app.text.len() //If u press shift and { at the end of the string it adds it to the end
            {
                app.text += "}";
                //println!("wtf");
                
            }
            else
            {
                app.text.replace_range(app.cursor_index..(app.cursor_index + 1), "}");
            }
            
        }
        if ui.input(|i| i.events.contains(&egui::Event::Text("(".to_owned()))) // { kinda logic
        {
            if app.cursor_index == app.text.len()
            {
                app.text += ")";
                //println!("wtf");
                
            }
            else
            {
                println!("{}", app.cursor_index);
                app.text.replace_range(app.cursor_index..(app.cursor_index), ")");
            }
        }
        if ui.input(|i| i.events.contains(&egui::Event::Text('"'.to_string())))
        {
            if app.cursor_index == app.text.len()
            {
                app.text += '"'.to_string().as_str();
                //println!("wtf");
                
            }
            else
            {
                app.text.replace_range(app.cursor_index..(app.cursor_index), '"'.to_string().as_str());
            }
        }

}