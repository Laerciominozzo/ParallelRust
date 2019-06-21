extern crate gtk;

use gtk::{Window};
use gtk::prelude::{*};


use super::imageArea::ImageArea;



pub struct MyWindow {
    janela: Window,
    image_area: ImageArea,


}


impl MyWindow{
    pub fn new() -> MyWindow{

        let glade_src = include_str!("../window.glade");
        let builder = gtk::Builder::new_from_string(glade_src);
        let image_area = ImageArea::new(&builder);
        let window :Window = builder.get_object("janela").unwrap();

        window.show_all();

        MyWindow {
            janela:window,
            image_area: image_area}

    }

    pub fn conect_events(&self){

        self.janela.connect_delete_event(|_, _| {
            gtk::main_quit();
            Inhibit(false)
        });
    }

    pub fn update_image<F:Fn(&ImageArea)>(&self, f:F){
        f(&self.image_area);
    }


}
/*https://stackoverflow.com/questions/31595115/how-can-i-get-my-own-data-to-a-gtk-callback-when-using-rust-gnome*/