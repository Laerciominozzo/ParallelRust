use gtk::{Builder, Image};
use gtk::prelude::{*};

pub struct Desenho{
    desenho_area: Image,

}

impl Desenho{
    pub fn new(builder:Builder) -> Desenho{
        let area :Image = builder.get_object("desenho").unwrap();

        area.connect_draw(|widget, context| unsafe {

            static mut teste:f64 =50.0;
            context.arc(teste,50.0,20.0,0.0,180.0);
            teste = teste +1.0;
            context.stroke();
            widget.queue_draw();
            Inhibit(true)

        });


        Desenho { desenho_area:area}
    }





}
