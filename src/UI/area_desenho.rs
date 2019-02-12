use gtk::{Builder, Image};
use gtk::prelude::{*};

pub struct Desenho{
    desenhoArea: Image,

}

impl Desenho{
    pub fn new(builder:Builder) -> Desenho{
        let area :Image = builder.get_object("desenho").unwrap();

        area.connect_draw(|widget, context|{

            context.stroke();
            Inhibit(false)
        });


        Desenho {desenhoArea:area}
    }

    fn draw(positionX:i64, positionY: i64){

    }
}
