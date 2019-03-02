use super::objeto::Objeto;

pub struct Espaco {
    objetos:Vec<Objeto>,
}

impl Espaco{
    pub fn new() -> Espaco{
        let espaco = Espaco{ objetos:Vec::new()};

        espaco
    }

    pub fn InsertObject(&mut self, x:f64, y:f64) {
        self.objetos.push(Objeto::new(x,y));

    }

    pub fn getCoordinates(&self)-> Vec<(f64,f64)>{
        let mut vec = Vec::new();

        for element  in self.objetos.iter(){
            vec.push(element.getCoordinates());
        }

        vec
    }

}