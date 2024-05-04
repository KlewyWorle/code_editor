#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui::{self, Key, TextBuffer};
use std::{borrow::{Borrow, BorrowMut}, fs::{File, OpenOptions}, future::IntoFuture, io::{Read, Write}};
use crate::core::gui::actions;
use crate::core::gui::textedit;
use crate::core::input::keyboard;
pub struct MyApp
{
    pub text : String,
    //path : String, 
    pub confirm_window : bool, // Should show confirm exit window
    pub can_exit : bool,
    pub cursor_index : usize, // Cursor index loc
    pub show_actions : bool, // Show action window
    pub current_dir : String,
    pub font_size : f32,
    pub language : String
}
impl Default for MyApp
{
    fn default() -> Self {
		println!("hello kcode");
        MyApp { text: String::from(""), confirm_window : false, can_exit : false, cursor_index : 0, show_actions : false, current_dir : String::new(), font_size : 15.0, language: "None".to_string()}
    }
}


pub fn exit_win(ui : &egui::Ui, ctx: &egui::Context, app : &mut MyApp)
{
    ctx.send_viewport_cmd(egui::ViewportCommand::CancelClose);
    egui::Window::new("Wanna exit noob?").resizable(true).movable(false).collapsible(false).current_pos([ui.available_width() / 2.2, ui.available_height() / 3.0])
    .show(ctx, |ui|
    {
        ui.horizontal(|ui|
        {
            if ui.button("Confirm").clicked()
            {
                app.can_exit = true;
                app.confirm_window = false;
                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            }
            if ui.button("Cancel").clicked()
            {
                app.confirm_window = false;
                
                //Do nothing and window will be closed
            }
        });
    });
}




impl eframe::App for MyApp
{
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        
        egui::CentralPanel::default().show(ctx, |ui| //Erm
        {
            ui.horizontal(|ui|
            {
                
                let mut value = 10;
                
                
                ui.label("      men code editor")
            });
            //Exit=========================
            
            keyboard::process_input(ctx, ui, self);
            
            textedit::text_conv(ui, self); //Handle {} and ()
            textedit::text_edit_setup(ui, self); //Displaying text edit
            
            
            actions::actions_menu(ctx, ui, self); //Action menu
            
        });
    }
}

