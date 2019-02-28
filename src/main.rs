mod ui;
mod core;
use std::process;



fn main() {

    if gtk::init().is_err() {
        eprintln!("failed to initialize GTK Application");
        process::exit(1);
    }

    let espaco = core::Espaco::new();
    let app = ui::App::novo(espaco);
    gtk::main();
}
