use gtk::{Window, Builder};
use gtk::prelude::{*};

use super::area_desenho::Desenho;

pub struct Janelaprincipal {
    janela: Window,
    desenho: Desenho,
}


impl Janelaprincipal {
    pub fn novo(builder: gtk::Builder) -> Janelaprincipal {
        let janela:Window = builder.get_object("janela").unwrap();

        janela.show_all();

        janela.connect_delete_event(|_, _| {
            gtk::main_quit();
            Inhibit(false)
        });
        let desenho = Desenho::new(builder);

        Janelaprincipal {janela,desenho}
    }
}