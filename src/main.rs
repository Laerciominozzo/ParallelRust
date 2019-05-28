mod ui;
mod core;
use std::process;
use crate::core::Espaco;
use crate::core::Objeto;


fn main() {

    if gtk::init().is_err() {
        eprintln!("failed to initialize GTK Application");
        process::exit(1);
    }


    let app = ui::App::novo();
    gtk::main();
}
