
pub struct Objeto<'a>{
    posicao: &'a mut (f64, f64),
    inercia: &'a mut (f32, f32)
}

impl<'a> Objeto<'a>{
    pub fn new(position: &'a mut (f64,f64), inercia: &'a mut (f32,f32)) -> Objeto<'a>{
        Objeto{posicao: position, inercia: inercia}
    }

    pub fn get_coordinates(&self) -> (f64,f64){
        *self.posicao
    }

    pub fn set_coordinates(& mut self, cordinates: (f64,f64)){
        *self.posicao = cordinates;
    }

    pub fn calc_new_position(&self) -> (f64, f64){
        (self.posicao.0 + self.inercia.0 as f64,
         self.posicao.1 + self.inercia.1 as f64)
    }

    pub fn show(&self) -> String{
        format!("({:.2}, {:.2})",self.posicao.0, self.posicao.1 )
    }

    pub fn print(&self, pixbuf : & mut gdk_pixbuf::Pixbuf) {
        unsafe {
            let pixels = pixbuf.get_pixels();
            let coord = self.get_coordinates();
            for i in 0..10 {
                pixels[((coord.0 + coord.1) as usize+ i) * 10 ] = 255;
            }
        }
    }
}