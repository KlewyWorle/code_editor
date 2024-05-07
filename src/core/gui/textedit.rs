use eframe::egui::{self, TextBuffer};
use std::{borrow::{Borrow, BorrowMut}, fs::{File, OpenOptions}, future::IntoFuture, io::{Read, Write}};
use egui_code_editor::{CodeEditor, ColorTheme, Syntax};
use std::collections::BTreeSet;
use crate::app;


pub fn getSyntax(lang : &str) -> Syntax
{
    match lang
    {
        "rs" =>
        {
            Syntax {
                language: "Rust",
                case_sensitive: true,
                comment: "//",
                comment_multiline: ["/*", "*/"],
                hyperlinks: BTreeSet::from(["http"]),
                keywords: BTreeSet::from([
                    "as", "break", "const", "continue", "crate", "else", "enum", "extern", "fn", "for",
                    "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref",
                    "return", "self", "struct", "super", "trait", "type", "use", "where", "while",
                    "async", "await", "abstract", "become", "box", "do", "final", "macro", "override",
                    "priv", "typeof", "unsized", "virtual", "yield", "try", "unsafe", "dyn",
                ]),
                types: BTreeSet::from([
                    "Option",
                    "Result",
                    "Error",
                    "Box",
                    "Cow",
                    // Primitives
                    "bool",
                    "i8",
                    "u8",
                    "i16",
                    "u16",
                    "i32",
                    "u32",
                    "i64",
                    "u64",
                    "i128",
                    "u128",
                    "isize",
                    "usize",
                    "f32",
                    "f64",
                    "char",
                    "str",
                    "String",
                    // STD Collections
                    "Vec",
                    "BTreeMap",
                    "BTreeSet",
                    "BTreeMap",
                    "BTreeSet",
                    "VecDeque",
                    "BinaryHeap",
                    "LinkedList",
                    // RC
                    "Rc",
                    "Weak",
                    "LazyCell",
                    "SyncUnsafeCell",
                    "BorrowErrorl",
                    "BorrowMutErrorl",
                    "Celll",
                    "OnceCelll",
                    "Refl",
                    "RefCelll",
                    "RefMutl",
                    "UnsafeCell",
                    "Exclusive",
                    "LazyLock",
                    // ARC
                    "Arc",
                    "Barrier",
                    "BarrierWaitResult",
                    "Condvar",
                    "Mutex",
                    "MutexGuard",
                    "Once",
                    "OnceLock",
                    "OnceState",
                    "PoisonError",
                    "RwLock",
                    "RwLockReadGuard",
                    "RwLockWriteGuard",
                    "WaitTimeoutResult",
                    "Weak",
                ]),
                special: BTreeSet::from(["Self", "static", "true", "false"]),
            }
        }
        "py" =>
        {
            //println!("fuck");
            Syntax::python()
        }
        _ => 
        {
            Syntax::default()
        }
    }
}

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
            .with_syntax(getSyntax(&app.language))
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