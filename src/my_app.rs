extern crate gtk;
extern crate gio;
extern crate gdk;


use gtk::{Builder};
use std::env::args;
use std::process;

use super::ui::Janelaprincipal;

pub struct app {
    janela : Janelaprincipal,

}


impl app {

    pub fn novo()-> app {
        if gtk::init().is_err() {
            eprintln!("failed to initialize GTK Application");
            process::exit(1);
        }

        let glade_src = include_str!("window.glade");
        let builder = gtk::Builder::new_from_string(glade_src);

        app { janela: Janelaprincipal::novo(builder)}
    }


}


