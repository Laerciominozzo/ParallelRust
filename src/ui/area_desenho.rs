use gtk::{Builder, Image};
use gtk::prelude::{*};
use cairo::Context;

use super::Espaco;


pub struct Desenho {
    desenho_area: Image,
}

impl Desenho{
    pub fn new<F: Fn(&Image, &cairo::Context) -> Inhibit + 'static>(builder: &Builder, f : F) -> Desenho

      {
          let area :Image = builder.get_object("desenho").unwrap();
          let object = Desenho { desenho_area:area};



          object
    }


    pub fn atualiza(&self){
        self.desenho_area.queue_draw();
    }

}
//https://stackoverflow.com/questions/31595115/how-can-i-get-my-own-data-to-a-gtk-callback-when-using-rust-gnome
