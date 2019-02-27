mod ui;
mod core;




fn main() {

    let espaco = core::Espaco::new();
    let app = ui::App::novo(espaco);
    gtk::main();
}
