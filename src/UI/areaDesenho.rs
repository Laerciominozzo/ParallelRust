use gtk::{Builder, Image};
use gtk::prelude::{*};

pub struct Desenho{
    desenhoArea: Image,

}

impl Desenho{
    pub fn new(builder:Builder) -> Desenho{
        let area :Image = builder.get_object("desenho").unwrap();

        area.connect_draw(|widget, context|{

            context.arc(50.0,50.0,20.0,0.0,180.0);
            context.stroke();
            Inhibit(false)
        });


        Desenho {desenhoArea:area}
    }
}
