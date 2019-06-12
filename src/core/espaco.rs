use super::objeto::Objeto;
use core::borrow::Borrow;


pub struct Espaco<'a>{
     objetos: Vec<Objeto<'a> >
}

impl<'a> Espaco<'a>{
    pub fn new_with_objects(positions:& mut Vec<(f64,f64)>) ->  Espaco{
        let mut espaco = Espaco{objetos: Vec::with_capacity(positions.capacity())};

        for i in positions{
            espaco.objetos.push(Objeto::new(i ));
        }
        espaco
    }


    pub fn processa(& mut self){
        for objeto in &mut self.objetos{
            let coordinates = objeto.calcNewPosition();
             objeto.setCoordinates(coordinates);
        }

    }

}