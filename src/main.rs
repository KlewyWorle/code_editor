#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui::{self, TextBuffer};
use std::{borrow::{Borrow, BorrowMut}, fs::{File, OpenOptions}, future::IntoFuture, io::{Read, Write}};
mod core;
use core::app;


fn main() -> Result<(), eframe::Error>
{
    
    let options = eframe::NativeOptions
    {
        viewport : egui::ViewportBuilder::default().with_fullscreen(false).with_inner_size([1600.0, 800.0]),
        ..Default::default()
    };
    eframe::run_native("KCode Editor", options, Box::new(|ss|
    {
        Box::<app::MyApp>::new(app::MyApp::default())
    }))
}
