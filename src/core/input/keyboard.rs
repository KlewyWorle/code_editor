use eframe::egui::{self, TextBuffer};


use crate::core::app;


pub fn process_input(ctx: &egui::Context, ui : &mut egui::Ui, app : &mut app::MyApp)
{
    let sc  = egui::KeyboardShortcut::new(egui::Modifiers::CTRL, egui::Key::R);
    if ui.input_mut(|i| i.consume_shortcut(&sc)) //Action window shortcut
    {
        //println!("wtf");
        app.show_actions = true;
    }
    if ui.input(|i| i.viewport().close_requested()) //Confirm exit window logic
    {
        if app.can_exit
        {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }
        else
        {
            app.confirm_window = true;    
        }
    }

    if app.confirm_window
    {
        
        app::exit_win(ui, ctx, app);
    }
}