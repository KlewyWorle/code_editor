#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui::{self, TextBuffer};
use std::{borrow::{Borrow, BorrowMut}, fs::{File, OpenOptions}, future::IntoFuture, io::{Read, Write}};


struct MyApp
{
    text : String,
    //path : String, 
    allowed_close : bool, // Should show confirm exit window
    can_exit : bool,
    cursor_index : usize, // Cursor index loc
    show_actions : bool, // Show action window
    current_dir : String,
    font_size : f32
}


impl eframe::App for MyApp
{
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| //Erm
        {
            ui.horizontal(|ui|
            {
                
                let mut value = 10;
                let label_font = ui.label("Font size");
                let slider = ui.add(egui::Slider::new(&mut self.font_size, 5.0..=35.0)).labelled_by(label_font.id);
                ui.label("      men code editor")
            });
            //Exit=========================
            
            if ui.input(|i| i.viewport().close_requested()) //Confirm exit window logic
            {
                if self.can_exit
                {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }
                else
                {
                    self.allowed_close = true;    
                }
            }
            if self.allowed_close
            {
                ctx.send_viewport_cmd(egui::ViewportCommand::CancelClose);
                egui::Window::new("Wanna exit noob?").resizable(true).movable(false).collapsible(false).current_pos([ui.available_width() / 2.2, ui.available_height() / 3.0])
                .show(ctx, |ui|
                {
                    ui.horizontal(|ui|
                    {
                        if ui.button("Confirm").clicked()
                        {
                            self.can_exit = true;
                            self.allowed_close = false;
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                        if ui.button("Cancel").clicked()
                        {
                            self.allowed_close = false;
                            
                            //Do nothing and window will be closed
                        }
                    });
                });
            }
            


            //Exit===================================
            //println!("{}", self.cursor_index);
            
            if ui.input(|i|  i.events.contains(&egui::Event::Text("{".to_owned()))) //There is no { in egui::Key
            {   
                if self.cursor_index == self.text.len() //If u press shift and { at the end of the string it adds it to the end
                {
                    self.text += "}";
                    //println!("wtf");
                    
                }
                else
                {
                    self.text.replace_range(self.cursor_index..(self.cursor_index), "}");
                }
                
            }
            if ui.input(|i| i.events.contains(&egui::Event::Text("(".to_owned()))) // { kinda logic
            {
                if self.cursor_index == self.text.len()
                {
                    self.text += ")";
                    //println!("wtf");
                    
                }
                else
                {
                    self.text.replace_range(self.cursor_index..(self.cursor_index), ")");
                }
            }

            
            //println!("{:?}", ctx.input(|f| return f.curs ));
            //keys_down.iter().for_each(|f| println!("fff {}", f.name()));
            //println!("{:?}", ctx.input(|f| return f.events.contains(&egui::Event::Text("(".to_owned()))));
            let sc  = egui::KeyboardShortcut::new(egui::Modifiers::CTRL, egui::Key::R);
            if ui.input_mut(|i| i.consume_shortcut(&sc)) //Action window shortcut
            {
                //println!("wtf");
                self.show_actions = true;
            }

            if self.show_actions //Show action window
            {
                egui::Window::new("Actions").resizable(true).movable(false).collapsible(false)
                .current_pos([ui.available_width() / 2.2, ui.available_height() / 3.0]) //Set window position
                .show(ctx, |ui|
                {
                    ui.horizontal(|ui|
                        {
                            if ui.button("Save").clicked()
                            {
                                
                    
                                if let Some(path) = rfd::FileDialog::new().set_file_name("fuck.txt").set_directory(if self.current_dir.is_empty() {"/home/klewy/"} else {self.current_dir.as_str()}).save_file() 
                                {
                                    let fil = path.file_name().unwrap();
                                    println!("{}", fil.to_str().unwrap());
            
                                    let mut file = match OpenOptions::new().write(true).truncate(true).open(path.to_str().unwrap().to_owned() + path.file_name().unwrap().to_str().unwrap())
                                    {
                                        Ok(file) => file,
                                        Err(why) => File::create(path.to_str().unwrap().to_owned()).unwrap()
                                    };
                                    file.write(self.text.as_bytes()).expect("err");
                                    file.flush().expect("err");

                                    self.show_actions = false; // Close action window

                                    self.current_dir = path.to_str().unwrap().to_owned();
                                    
                                }
                            }
                            if ui.button("Open").clicked()
                            {
                                if let Some(path) = rfd::FileDialog::new().set_directory(if self.current_dir.is_empty() {"/home/klewy/"} else {self.current_dir.as_str()}).pick_file()
                                {
                                    let mut file = match OpenOptions::new().read(true).open(path.to_str().unwrap())
                                    {
                                        Ok(file) => file,
                                        Err(why) => panic!("404")
                                    };
                                    file.read_to_string(&mut self.text).expect("Err");
                                    file.flush().expect("Err");
                                    self.show_actions = false;
                                    self.current_dir = path.to_str().unwrap().to_owned();
                                }
                            }
                            if ui.button("Copy").clicked()
                            {
                                ctx.copy_text(self.text.clone());
                                self.show_actions = false;
                            }
                            if ui.button("Close").clicked()
                            {
                                self.show_actions = false;
                            }
                            
                        });
                });
                
            }
            
            egui::ScrollArea::new(true).show(ui, |ui|
            {
                // let t_ed = ui.add(egui::TextEdit::multiline(&mut self.text).hint_text("Type...")
                // .min_size(egui::vec2(ui.available_width(), ui.available_height()))
                // .font(egui::FontId::new(100.0, egui::FontFamily::Monospace))
                // .desired_width(ui.available_width())
                // .code_editor());

                let t_edit = egui::TextEdit::multiline(&mut self.text).hint_text("Type...")
                .min_size(egui::vec2(ui.available_width(), ui.available_height()))
                .font(egui::FontId::new(100.0, egui::FontFamily::Monospace))
                .desired_width(ui.available_width())
                .code_editor()
                .font(egui::FontId::proportional(self.font_size))
                .show(ui); //Use it so it returns TextEditOutput instead of response

                if let Some(text) = t_edit.cursor_range
                {
                    self.cursor_index = text.primary.ccursor.index;
                }
                
            
                
                
            });
            
        });
    }
}



fn main() -> Result<(), eframe::Error>
{
    let options = eframe::NativeOptions
    {
        viewport : egui::ViewportBuilder::default().with_fullscreen(false).with_inner_size([1600.0, 800.0]),
        ..Default::default()
    };
    eframe::run_native("KCode Editor", options, Box::new(|ss|
    {
        Box::<MyApp>::new(MyApp { text: String::from(""), allowed_close : false, can_exit : false, cursor_index : 0, show_actions : false, current_dir : String::new(), font_size : 15.0})
    }))
}
