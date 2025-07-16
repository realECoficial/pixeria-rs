use std::io;
use std::collections::HashMap;
use std::thread;
use std::time::Duration;
#[warn(non_camel_case_types)]

enum Pixas {
    pixa1,    
    pixa2,    
    pixa3,    
    pixa4,    
} 
struct Repartidor { 
    nombre: String,
    edad: i32,
    tiempo: i32,
} 

impl Repartidor { 
    fn cuanto_le_falta(&self) -> i32{
        self.tiempo 
    }

    fn eleccion_pixa(pixasmap: &HashMap<i32, String>, numero: i32) -> String{
       let psa = String::from(""); 
        pixasmap.get(&numero).unwrap_or(&psa).to_owned()
    }
}


fn main() {

    let mut pixasmap = HashMap::new();     
    pixasmap.insert(1, String::from("pixa con piña"));
    pixasmap.insert(2, String::from("pixa con peperoni"));
    pixasmap.insert(3, String::from("pixa con mierda"));
    pixasmap.insert(4, String::from("pixa con ccoco"));
    

    let nombre = String::from("emelioo");
    let edad= 21;
    let tiempo= 21;
    let repartidor1 = Repartidor {
        nombre: nombre,
        edad: edad,
        tiempo: tiempo,
    
    };
    println!("Elija que pixa quiere:");
   

    let mut eleccion_pixa= String::new();
    io::stdin()
    .read_line(&mut eleccion_pixa)
    .expect("Failed to read line");    
     
    let eleccion_pixa: i32 = eleccion_pixa.trim().parse().expect("");
    let pixa_elejida = Repartidor::eleccion_pixa(&pixasmap, eleccion_pixa); 
    
    println!("Nombre: {}, edad: {}, tiempo estimado: {}, pixa: {}", repartidor1.nombre,repartidor1.edad,repartidor1.tiempo,pixa_elejida);
    
    //TODO hacer que se pueda consultar en tiempo real 
    //cuanto le falta a la pixa
    //PROBAANDO 
/*
   thread::spawn(|| {

    println!("Consulte cuanto le queda:");
   

    let mut cuanto_queda= String::new();
    io::stdin()
    .read_line(&mut cuanto_queda)
    .expect("Failed to read line");    
     
    let cuanto_queda: i32 = cuanto_queda.trim().parse().expect("");

       for i in 1..21 {
            if cuanto_queda == 1{
                println!("le falta: {i} para que se haga la pixa. ");
                cuanto_queda = 0; 
            } 
             
            thread::sleep(Duration::from_millis(600));
        }
    });
*/ 

    /*  
   for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1300));
    }
  */ 
   //TODO
   //PROBAANDO 
    //TODO let tiempo_estimado =Repartidor::cuanto_le_falta(&repartidor1); 
    

    
}

//TODO https://doc.rust-lang.org/book/ch16-01-threads.html Add multithreading for counting down for
/*
struct Repartidor { 
    nombre: String,
    edad: String,
    tiempo: String, ---> This one
}*/ 
