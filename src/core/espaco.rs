use super::objeto::Objeto;
use core::borrow::Borrow;

#[derive(Clone)]
pub struct Espaco{
     objetos:Vec<Objeto>,
}

impl Espaco{
    pub fn new() ->   Espaco{
        let mut espaco : Espaco =  Espaco{ objetos:Vec::new()};
        espaco.InsertObject(10.0,10.0);
        espaco.InsertObject(20.0, 20.0);
        espaco
    }

    pub fn InsertObject(&mut self, x:f64, y:f64) {
        self.objetos.push(Objeto::new(x,y));

    }



    pub fn processa(& mut self){
        for  objeto in & mut self.objetos{
            objeto.setCoordinates(objeto.calcNewPosition());
        }

    }

}