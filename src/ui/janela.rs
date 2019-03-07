use gtk::{Window, Builder};
use gtk::prelude::{*};

use super::area_desenho::Desenho;

pub struct Janela{
    janela: Window,
    desenho: Desenho,
}

impl Janela{
    pub fn new(builder:Builder, desenho:Desenho) -> Janela{

        let janela:Window = builder.get_object("janela").unwrap();

        janela.show_all();

        let obj = Janela{janela:janela, desenho:desenho};
        obj.connect_events();

        obj
    }

    fn connect_events(&self) {
        self.janela.connect_delete_event(|_, _| {
            gtk::main_quit();
            Inhibit(false)
        });
    }
}