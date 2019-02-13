use gtk::{Builder, Image};
use gtk::prelude::{*};

pub struct Desenho{
    desenho_area: Image,

}

impl Desenho{
    pub fn new(builder:Builder) -> Desenho{
        let area :Image = builder.get_object("desenho").unwrap();

        area.connect_draw(|widget, context|{

            context.stroke();
            Inhibit(false)
        });


        Desenho { desenho_area:area}
    }

    fn draw(position_x:i64, position_y: i64){

    }
}
