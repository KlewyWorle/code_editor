use eframe::egui::{self, TextBuffer};
use std::{borrow::{Borrow, BorrowMut}, fs::{File, OpenOptions}, future::IntoFuture, io::{Read, Write}};
use crate::app;


fn save_file(app : &mut app::MyApp)
{
    
    if let Some(path) = rfd::FileDialog::new().set_file_name("fuck.txt").set_directory(if app.current_dir.is_empty() {"/home/"} else {app.current_dir.as_str()}).save_file() 
    {
        let fil = path.file_name().unwrap();
        println!("{}", fil.to_str().unwrap());

        let mut file = match OpenOptions::new().write(true).truncate(true).open(path.to_str().unwrap().to_owned() + path.file_name().unwrap().to_str().unwrap())
        {
            Ok(file) => file,
            Err(why) => File::create(path.to_str().unwrap().to_owned()).unwrap()
        };
        file.write(app.text.as_bytes()).expect("err");
        file.flush().expect("err");

        app.show_actions = false; // Close action window
        
        app.current_dir = path.to_str().unwrap().to_owned();

        app.language = path.to_str().unwrap().split(".").collect::<Vec<_>>()[1].to_owned(); //Language name for synax highlighting
        
    }
}
fn open_file(app : &mut app::MyApp)
{
    
    if let Some(path) = rfd::FileDialog::new().set_directory(if app.current_dir.is_empty() {"/home/"} else {app.current_dir.as_str()}).pick_file()
    {
        println!("fuck open file");
        let mut file = match OpenOptions::new().read(true).open(path.to_str().unwrap())
        {
            Ok(file) => file,
            Err(why) => panic!("404")
        };
        file.read_to_string(&mut app.text).expect("Err");
        file.flush().expect("Err");
        app.show_actions = false;
        app.current_dir = path.to_str().unwrap().to_owned();
        //println!("{}", path.to_str().unwrap());
        app.language = path.to_str().unwrap().split(".").collect::<Vec<_>>()[1].to_owned(); //Language name for synax highlighting
        
    }
}

pub fn actions_menu(ctx: &egui::Context, ui : &mut egui::Ui, app : &mut app::MyApp)
{
    

    if app.show_actions //Show action window
    {
        egui::Window::new("Actions").resizable(true).movable(false).collapsible(false)
        .current_pos([ui.available_width() / 2.4, ui.available_height() / 3.0]) //Set window position
        .show(ctx, |ui|
        {
            ui.horizontal(|ui|
            {
                if ui.button("Save").clicked()
                {
                    save_file(app);
                }
                if ui.button("Open").clicked()
                {
                    open_file(app);
                }
                if ui.button("Copy").clicked()
                {
                    ctx.copy_text(app.text.clone());
                    app.show_actions = false;
                }
                if ui.button("Close").clicked()
                {
                    app.show_actions = false;
                }
                
                
                
            });
            ui.horizontal(|ui|
            {
                let label_font = ui.label("Font size");
                let slider = ui.add(egui::Slider::new(&mut app.font_size, 5.0..=35.0)).labelled_by(label_font.id);
            });
        });
        
    }
}
