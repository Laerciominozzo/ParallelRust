use crate::ui::{MyWindow};
use crate::core::Espaco;
use std::cell::{RefMut, RefCell};
use std::rc::Rc;


#[derive(Clone)]
pub struct MyApp<'a>{
    my_window : MyWindow,
    space     : Espaco<'a>
}

impl<'a> MyApp<'a>{
    pub fn new(space :Espaco) -> MyApp{

        let mut my_window = MyWindow::new();

        MyApp{my_window:my_window, space:space}
    }

    pub fn show_in_imagearea(&self) {

        self.my_window.update_image( |image_area|{
            self.space.print_objects(|obj|{
                let coordenadas = obj.get_coordinates();
                    image_area.add_point(coordenadas.0, coordenadas.1);
            })
        })
    }

    pub fn connect_events(&self) -> Rc<RefCell<MyApp>>{
        let connected_self = Rc::new(RefCell::new(self.clone()));
        let borrowed_self = connected_self.clone();

        connected_self.borrow().my_window.connect_step(move||{
            let mut borrowed_self = borrowed_self.clone().get_mut();

            borrowed_self.space.processa();
            borrowed_self.show_in_imagearea();
        });

        connected_self
    }
}

