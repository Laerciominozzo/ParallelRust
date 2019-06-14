use gtk::{Window, Builder, Image};
use gtk::prelude::{*};

use super::Espaco;
use gdk::enums::key::Open;

pub struct App<'a>{
    desenho_area: Image,
    janela: Window,
    espaco:Espaco<'a>,
}


impl<'a> App<'a>{
    pub fn novo( mut espaco :  Espaco) -> App {

        let glade_src = include_str!("../window.glade");
        let builder = gtk::Builder::new_from_string(glade_src);
        let area :Image = builder.get_object("desenho").unwrap();
        let janela:Window = builder.get_object("janela").unwrap();

        janela.show_all();

        let e_espaco :&Espaco = &espaco;
        area.connect_draw(move|image, context| {


            e_espaco.getValues();
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
/*https://stackoverflow.com/questions/31595115/how-can-i-get-my-own-data-to-a-gtk-callback-when-using-rust-gnome*/