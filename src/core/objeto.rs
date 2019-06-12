
pub struct Objeto<'a>{
    posicao:&'a mut (f64,f64),

}

impl<'a> Objeto<'a>{
    pub fn new(position: &mut (f64,f64)) -> Objeto{
        Objeto{posicao: position}
    }

    pub fn getCoordinates(&self) -> (f64,f64){
        *self.posicao
    }

    pub fn setCoordinates(& mut self, cordinates: (f64,f64)){
        *self.posicao = cordinates;
    }

    pub fn calcNewPosition(&self) -> (f64, f64){
        (self.posicao.0 + 2 as f64,
         self.posicao.1 + 2 as f64)
    }
}