use gtk::{Window, Builder, Image};
use gtk::prelude::{*};

use super::Espaco;

pub struct App{
    desenho_area: Image,
    janela: Window,
    espaco:Espaco,
}


impl App{
    pub fn novo(  ) -> App {

        let glade_src = include_str!("../window.glade");
        let builder = gtk::Builder::new_from_string(glade_src);
        let area :Image = builder.get_object("desenho").unwrap();
        let janela:Window = builder.get_object("janela").unwrap();

        janela.show_all();
        let  espaco = Espaco::new();

        let e_espaco = espaco.clone();
        area.connect_draw(move|image, context| {

            let mut espaco = e_espaco.clone();
            espaco.processa();
            context.stroke();
            Inhibit(true)
        });

        janela.connect_delete_event(|_, _| {
            gtk::main_quit();
            Inhibit(false)
        });

        App{espaco:espaco,
            janela:janela,
            desenho_area: area}


    }

    pub fn rodadaProcessamento(&mut self, numeroRodadas: u32){
        for _i in 0 .. numeroRodadas{
            self.processa();
        }
    }

    fn processa(&mut self){
        self.desenho_area.queue_draw();
    }


}