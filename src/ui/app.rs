use gtk::{Window, Builder};
use gtk::prelude::{*};

use super::janela::Janela;
use super::area_desenho::Desenho;

use super::Espaco;

pub struct App {
    janela:Janela,
}


impl App {
    pub fn novo(espaco: Espaco) -> App {

        let glade_src = include_str!("../window.glade");
        let builder = gtk::Builder::new_from_string(glade_src);

        let desenho:Desenho = Desenho::new(builder.clone(), espaco);
        let janela = Janela::new(builder.clone(),desenho);
        let app = App{janela};


        app
    }

}