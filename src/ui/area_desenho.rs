use gtk::{Builder, Image};
use gtk::prelude::{*};
use cairo::Context;

use super::Espaco;


pub struct Desenho {
    desenho_area: Image,
}

impl Desenho{
    pub fn new(builder:Builder, espaco: Espaco) -> Desenho
      {
          let area :Image = builder.get_object("desenho").unwrap();

          let object = Desenho { desenho_area:area};

          object.desenho_area.connect_draw(move |image, context|{

              for e in espaco.getCoordinates(){
                  context.arc(e.0, e.1, 1.0, 0.0, 360.0);
              }
              context.stroke();
              Inhibit(true)
          });

          object
    }


    pub fn atualiza(&self){
        self.desenho_area.queue_draw();
    }

}
//https://stackoverflow.com/questions/31595115/how-can-i-get-my-own-data-to-a-gtk-callback-when-using-rust-gnome
