use super::objeto::Objeto;


pub struct Espaco<'a>{
     objetos: Vec<Objeto<'a>>
}

impl<'a> Espaco<'a>{
    pub fn new_with_objects(positions:&'a mut Vec<(f64,f64)>, inercias:&'a mut Vec<(f32,f32)>) ->  Espaco<'a>{
        let mut espaco = Espaco{objetos: Vec::with_capacity(positions.capacity())};

        let mut pos = positions.into_iter();
        let mut ine = inercias.into_iter();
        loop {
            match (pos.next(), ine.next()) {
                (Some(a),Some(b)) => espaco.objetos.push(Objeto::new( a,b)),
                _ => break
            }
        }
        espaco
    }


    pub fn processa(& mut self){
        for objeto in &mut self.objetos{
            let coordinates = objeto.calc_new_position();
             objeto.set_coordinates(coordinates);
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

    pub fn get_values(&self) -> Option <Vec<(f64, f64) > > {
        let mut retorno : Vec<(f64,f64)> = Vec::with_capacity(self.objetos.len());

        for obj in &self.objetos{
            retorno.push(obj.get_coordinates());
        }

        match retorno.len() {
            0=> None,
            _=> Some(retorno)
        }
    }

    pub fn print(&self, pixbuf : & mut gdk_pixbuf::Pixbuf){
        for obj in &self.objetos{
            obj.print(pixbuf);
        }
    }

}