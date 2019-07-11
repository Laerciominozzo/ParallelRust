extern crate gtk;

use gtk::{Window};
use gtk::prelude::{*};


use super::imageArea::ImageArea;
use std::rc::Rc;
use std::cell::RefCell;
use self::gtk::Button;


#[derive(Clone)]
pub struct MyWindow {
    janela: Window,
    image_area: ImageArea,
    builder : gtk::Builder,

}


impl MyWindow{
    pub fn new() -> MyWindow{

        let glade_src = include_str!("../window.glade");
        let builder = gtk::Builder::new_from_string(glade_src);
        let image_area = ImageArea::new(&builder);
        let window :Window = builder.get_object("janela").unwrap();

        window.show_all();
        window.connect_delete_event(|_, _| {
            gtk::main_quit();
            Inhibit(false)
        });

        MyWindow {
            janela:window,
            image_area: image_area,
            builder: builder,
            };
    }



    pub fn connect_play<F:Fn() + 'static >(&self, f:F){
        let button : Button = self.builder.get_object("play").unwrap();

        button.connect_clicked(move | button|{
           f();
        });
    }

    pub fn connect_step<F:Fn() + 'static >(&self, f:F){
        let button : Button = self.builder.get_object("step").unwrap();

        button.connect_clicked(move | button|{
            f();
        });
    }


    pub fn update_image<F:Fn(&ImageArea)>(&self, f:F){
        f(&self.image_area);
    }


}
/*https://stackoverflow.com/questions/31595115/how-can-i-get-my-own-data-to-a-gtk-callback-when-using-rust-gnome*/