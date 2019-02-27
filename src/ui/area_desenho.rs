use gtk::{Builder, DrawingArea};
use gtk::prelude::{*};
use cairo::Context;

pub type F =  fn(&DrawingArea,&Context) -> Inhibit ;

pub struct Desenho {
    desenho_area: DrawingArea,
}

impl Desenho{
    pub fn new(builder:Builder) -> Desenho
      {
        let area :DrawingArea = builder.get_object("desenho").unwrap();

        let object = Desenho { desenho_area:area};

        object
    }



    pub fn atualiza(&self){
        self.desenho_area.queue_draw();
    }

}
//https://stackoverflow.com/questions/31595115/how-can-i-get-my-own-data-to-a-gtk-callback-when-using-rust-gnome
