extern crate rand;

use rand::Rng;

mod ui;
mod core;

use crate::core::Espaco;

const VEC_SIZE :i32 = 20;

fn populate_vector<T>(vec : & mut Vec<(T,T)>)
    where rand::distributions::Standard: rand::distributions::Distribution<T>{
    let mut rng= rand::thread_rng();
    for a in 0..VEC_SIZE{
        vec.push ((rng.gen(), rng.gen()));
    }
}

fn main() {

    let mut posicoes:Vec<(f64,f64)> = Vec::with_capacity(VEC_SIZE as usize);
    populate_vector(& mut posicoes);

    let mut inercias:Vec<(f32,f32)> = Vec::with_capacity(VEC_SIZE as usize);
    populate_vector(& mut inercias);

    if gtk::init().is_err() {
        eprintln!("failed to initialize GTK Application");
     //   process::exit(1);
    }

    let  espaco = Espaco::new_with_objects(& mut posicoes, & mut inercias);
    let mut app = ui::App::novo(espaco) ;
    app.conect_events();
    app.desenha();
    gtk::main();


/*


    let mut espaco = Espaco::new_with_objects(&mut posicoes, & mut inercias);

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
