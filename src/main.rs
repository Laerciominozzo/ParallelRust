mod ui;
mod my_app;

use my_app::app;

fn main() {

    let App = app::novo();

    gtk::main();
}
