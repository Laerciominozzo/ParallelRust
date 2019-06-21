use crate::ui::MyWindow;
use crate::core::Espaco;

pub struct MyApp<'a>{
    my_window : MyWindow,
    space     : Espaco<'a>
}

impl<'a> MyApp<'a>{
    pub fn new(space :Espaco) -> MyApp{

        let mut my_window = MyWindow::new();
        my_window.conect_events();

        MyApp{my_window:my_window, space:space}
    }

    pub fn Teste(&self) {

    }
}