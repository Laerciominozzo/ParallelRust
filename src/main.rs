//mod ui;
mod core;
use std::process;
use crate::core::Espaco;
use crate::core::Objeto;


fn main() {

   /* if gtk::init().is_err() {
        eprintln!("failed to initialize GTK Application");
        process::exit(1);
    }


    let app = ui::App::novo();
    gtk::main();*/



    let mut posicoes:Vec<(f64,f64)> = vec![(2.3,2.0),(3.4,5.0)];

    for e in &posicoes{
        print!("({},{}), ", e.0, e.1);
    }
    println!("");

        Espaco::new_with_objects(&mut posicoes).processa();

    for e in &posicoes{
        print!("({},{}), ", e.0, e.1);
    }
}
