mod ui;
mod core;
use std::process;
use crate::core::Espaco;
use crate::core::Objeto;


fn main() {

    let mut posicoes:Vec<(f64,f64)> = vec![(2.3,2.0),(3.4,5.0)];

    if gtk::init().is_err() {
        eprintln!("failed to initialize GTK Application");
        process::exit(1);
    }

    let mut espaco = Espaco::new_with_objects(& mut posicoes);
    let app = ui::App::novo(espaco);
    gtk::main();


/*


    let mut espaco = Espaco::new_with_objects(&mut posicoes);

    match espaco.show() {
        Some(t) => println!("{}", t),
        None           => println!("Não há objetos no espaço. ")
    }
        espaco.processa();

    match espaco.show() {
        Some(t) => println!("{}", t),
        None           => println!("Não há objetos no espaço. ")
    }*/
}
