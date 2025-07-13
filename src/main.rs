enum Pixas {
    pixa1,    
    pixa2,    
    pixa3,    
    pixa4,    
} 
struct Repartidor { 
    nombre: String,
    edad: i32,
    encargo: String,
    tiempo: i32,
} 

impl Repartidor {
    fn cuanto_le_falta(&self) -> String{
        self.tiempo.to_string() 
    }

    fn eleccion_pixa(pixa: Pixas){
        match  pixa{
           Pixas::pixa1 => println!("pixa con pinia"), 
           Pixas::pixa2 => println!("polla"), 
           Pixas::pixa3 => println!("polla"), 
           Pixas::pixa4 => println!("polla"), 
        } 

    }
}


fn main() {
     
    let nombre = String::from("emelioo");
    let edad= 21;
    let tiempo= 21;
    let repartidor1 = Repartidor {
        nombre: nombre,
        edad: edad,
        encargo: "".to_string(),
        tiempo: tiempo,
    
    };
    println!("Nombre: {}, edad: {}, tiempo: {}", repartidor1.nombre,repartidor1.edad,repartidor1.tiempo);

}

//TODO https://doc.rust-lang.org/book/ch16-01-threads.html
//Add multithreading for counting down for
/*
struct Repartidor { 
    nombre: String,
    edad: String,
    encargo: String,
    tiempo: String, ---> This one
}*/ 
