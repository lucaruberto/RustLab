use clap::Parser;
//use es1::capitalize;
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about=None)]
struct Args{
    #[clap(short, long)]
    stringa: String,
}

fn main() {
    let args = Args::parse();

    //for _ in 0..args.count {
    //println!("Input -> {}", args.stringa);

    //println!("Hello, world!");
}

//cargo run
//cargo build

//Convertire in maiuscolo il primo carattere di ogni parola che compone
//il testo -s, ignorando eventuali altri caratteri maiuscoli al suo interno
fn capitalize(s:&str)->String{

    let chars=s.chars();
    let mut stringa=String::new();

    for c in s.chars(){
        stringa.push(c.to_ascii_uppercase());
    }
    stringa

}
