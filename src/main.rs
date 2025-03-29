use std::fs::File;
use std::env;
use std::io::Read;


fn open_file(path: &String) {
    println!("El path es: {path}");
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(why) => panic!("No se puede abrir el archivo {}", why)
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("No se puede abrir el archivo {}", why),
        Ok(_) => println!("{}", s)
    }

}
fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    open_file(path);
}