use gtk::{Window, Builder};
use gtk::prelude::{*};

use super::area_desenho::Desenho;
use super::area_desenho::F;
use super::Espaco;

pub struct App {
    janela: Window,
    desenho: Desenho,
    espaco:Espaco,
}


impl App {
    pub fn novo(espaco: Espaco) -> App {

        let glade_src = include_str!("../window.glade");
        let builder = gtk::Builder::new_from_string(glade_src);

        let janela:Window = builder.get_object("janela").unwrap();
        let desenho:Desenho = Desenho::new(builder);

        janela.show_all();
        let app = App{janela,desenho,espaco};
        app.connect_events();

        app
    }

    fn connect_events(&self) {
        self.janela.connect_delete_event(|_, _| {
            gtk::main_quit();
            Inhibit(false)
        });
    }

    pub fn atualiza(&self){
        self.desenho.atualiza();
    }

}