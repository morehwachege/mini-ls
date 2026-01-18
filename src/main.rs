use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?},",args);
    for (_, argument) in args.iter().enumerate().skip(1) {
        println!("{}\n", argument)
    }
}
