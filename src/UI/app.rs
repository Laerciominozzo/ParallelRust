extern crate gtk;
extern crate gio;
extern crate gdk;


use gtk::{Builder,};
use gtk::prelude::{*};
use std::env::args;
use std::process;

use super::janelaPrincipal::janelaPrincipal;

pub struct  App{
    janela :janelaPrincipal,

}


impl App{

    pub fn novo()->App{
        if gtk::init().is_err() {
            eprintln!("failed to initialize GTK Application");
            process::exit(1);
        }

        let glade_src = include_str!("../window.glade");
        let builder = gtk::Builder::new_from_string(glade_src);

        App{ janela: janelaPrincipal::novo(builder)}
    }


}


