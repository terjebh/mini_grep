use std::env;
use std::fs;
fn main() {
    let args: Vec<String> = env::args().collect(); 
    let query = &args[1]; 
    let filename = &args[2];
    print!("Søker etter: '{}' ", query);
    println!("i filen: '{}'", filename);

    let contents = fs::read_to_string(filename)
    .expect("Noe gikk galt ved lesing av filen");

    println!("Tekst i søkefil: \n{}",contents);


}
