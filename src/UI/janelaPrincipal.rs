use gtk::{Builder, Window};
use gtk::prelude::{*};

use super::areaDesenho::Desenho;

pub struct janelaPrincipal{
    janela: Window,
    desenho: Desenho,
}


impl janelaPrincipal{
    pub fn novo(builder: gtk::Builder) -> janelaPrincipal{
        let janela:Window = builder.get_object("janela").unwrap();

        janela.show_all();

        janela.connect_delete_event(|_, _| {
            gtk::main_quit();
            Inhibit(false)
        });
        let desenho = Desenho::new(builder);

        janelaPrincipal{janela,desenho}
    }
}