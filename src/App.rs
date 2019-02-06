extern crate gtk;
extern crate gio;
extern crate gdk;


use gtk::{Builder, Window, Image};
use gtk::prelude::{*};
use std::env::args;
use std::process;


pub struct  App{
    janela :janelaPrincipal,

}


impl App{

    pub fn novo()->App{
        if gtk::init().is_err() {
            eprintln!("failed to initialize GTK Application");
            process::exit(1);
        }

        let glade_src = include_str!("window.glade");
        let builder = gtk::Builder::new_from_string(glade_src);

        App{ janela: janelaPrincipal::novo(builder)}
    }


}

struct janelaPrincipal{
    janela: Window,
    desenho: Desenho,
}


impl janelaPrincipal{
    fn novo(builder: gtk::Builder) -> janelaPrincipal{
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

struct Desenho{
    desenhoArea: Image,

}

impl Desenho{
    fn new(builder:Builder) -> Desenho{
        let area :Image = builder.get_object("desenho").unwrap();

        area.connect_draw(|widget, context|{

            context.arc(50.0,50.0,20.0,0.0,180.0);
            context.stroke();
            Inhibit(false)
        });


        Desenho {desenhoArea:area}
    }
}
