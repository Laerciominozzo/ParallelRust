#[derive(Clone)]
pub struct Objeto{
    posicaoX:f64,
    posicaoY:f64,
    inerciaX:f32,
    inerciaY:f32,
}

impl Objeto{
    pub fn new(x:f64, y:f64) -> Objeto{
        Objeto{posicaoX: x, posicaoY: y, inerciaX: 2.0, inerciaY: 1.0}
    }

    pub fn getCoordinates(&self) -> (f64,f64){
        (self.posicaoX, self.posicaoY)
    }

    pub fn setCoordinates(&mut self, cordinates: (f64,f64)){
        self.posicaoX = cordinates.0;
        self.posicaoY = cordinates.1;
    }

    pub fn calcNewPosition(&self) -> (f64, f64){
        (self.posicaoX + self.inerciaX as f64,
         self.posicaoY + self.inerciaY as f64)
    }
}