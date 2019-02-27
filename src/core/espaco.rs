use super::objeto::Objeto;

pub struct Espaco {
    objeto:Objeto,
}

impl Espaco{
    pub fn new() -> Espaco{
        Espaco{objeto:Objeto::new()}
    }


}