use gtk::{Image, Builder};


use gtk::prelude::{*};
use gdk_pixbuf;
use std::any::Any;
use gdk::enums::key::union;

const HEIGT :i32 = 200;
const WIDTH :i32 = 200;

const BORDER_UP: (f64, f64)= (0.0,0.0);
const BORDER_BOTTON: (f64,f64) = (WIDTH as f64, HEIGT as f64);
pub struct ImageArea{
    image : Image,
    heigt : i32,
    width : i32,
    upper_border: (f64, f64),
    botton_border: (f64, f64),
}

impl ImageArea{

    pub fn new(builder: &Builder) -> ImageArea{
        let area :Image = builder.get_object("desenho").unwrap();

        let image_area = ImageArea{image:area,
            heigt:HEIGT,
            width:WIDTH,
            upper_border: BORDER_UP,
            botton_border: BORDER_BOTTON
        };

        let mut pixbuf =gdk_pixbuf::Pixbuf::new(gdk_pixbuf::Colorspace::Rgb,
                                                false, 8,image_area.width,
                                                image_area.heigt);
        image_area.image.set_from_pixbuf(Some(&pixbuf));

        image_area
    }

    pub fn add_point(&self,x:f64, y:f64){
        match self.image.get_pixbuf() {
            Some(T) => unsafe{
                let mut pixels= T.get_pixels();
                pixels[((x*self.width as f64 + y)*3.0) as usize] = 255;
                pixels[((x*self.width as f64 + y)*3.0+1.0) as usize] = 255;
                pixels[((x*self.width as f64 + y)*3.0+2.0) as usize] = 255;
            },
            None => println!("Erro ao obter o pixbuf!"),

        } ;

    }

    fn calc_vewport(&self,coordnates : &(f64,f64) ) -> Option<usize>{


        let pos_x = (self.botton_border.0 - self.upper_border.0) / (self.width) as f64 * (coordnates.0 - self.upper_border.0);
        let pos_y = (self.botton_border.1 - self.upper_border.1) / (self.heigt) as f64 * (coordnates.1 - self.upper_border.1);

        Some((pos_x * self.width as f64 + pos_y) as usize)
    }


    pub fn get_size(&self) -> (i32, i32){
        (self.heigt, self.width)
    }


}