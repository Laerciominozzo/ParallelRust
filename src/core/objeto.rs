pub struct Objeto{
    x:f64,
    y:f64,
}

impl Objeto{
    pub fn new(x:f64, y:f64) -> Objeto{
        Objeto{x: x, y: y}
    }

    pub fn getCoordinates(&self) -> (f64,f64){
        (self.x, self.y)
    }
}