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

    pub fn show(&self) -> Option<String>{
        let mut retorno = String::new();

        for obj in &self.objetos{
            retorno.push_str(obj.show().as_str());
        }

        match retorno.len() {
            0 => None,
            _ => Some(retorno)
        }
    }

    pub fn getValues(&self) -> Option <Vec<(f64, f64) > > {
        let mut retorno : Vec<(f64,f64)> = Vec::with_capacity(self.objetos.len());

        for obj in &self.objetos{
            retorno.push(obj.getCoordinates());
        }

        match retorno.len() {
            0=> None,
            _=> Some(retorno)
        }
    }

}