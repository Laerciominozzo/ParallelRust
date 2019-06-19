extern crate gtk;

use gtk::{Window, Image};
use gtk::prelude::{*};
use gdk_pixbuf;


use super::Espaco;


pub struct App<'a>{
    desenho_area: Image,
    janela: Window,
    espaco:Espaco<'a>,
}


impl<'a> App<'a>{
    pub fn novo( espaco :  Espaco<'a>) -> App<'a>{

        let glade_src = include_str!("../window.glade");
        let builder = gtk::Builder::new_from_string(glade_src);
        let area :Image = builder.get_object("desenho").unwrap();
        let janela:Window = builder.get_object("janela").unwrap();

        let mut pixbuf =gdk_pixbuf::Pixbuf::new(gdk_pixbuf::Colorspace::Rgb, false, 8,600 ,600);
        area.set_from_pixbuf(Some(&pixbuf));

        janela.show_all();

        App{espaco:espaco,
            janela:janela,
            desenho_area: area}

    }

    pub fn conect_events(&self){

       // let app = *(self.clone());
//        self.desenho_area.connect_draw(move|_image, context| {
//
//         //   app.espaco.show();
//            context.stroke();
//            Inhibit(true)
//        });

        self.janela.connect_delete_event(|_, _| {
            gtk::main_quit();
            Inhibit(false)
        });
    }

    pub fn desenha(& mut self){

        let mut pixbuf =self.desenho_area.get_pixbuf();
        self.espaco.processa();
        self.espaco.print(& mut pixbuf.unwrap());




    }


}
/*https://stackoverflow.com/questions/31595115/how-can-i-get-my-own-data-to-a-gtk-callback-when-using-rust-gnome*/