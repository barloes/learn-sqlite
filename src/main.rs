mod eval;
mod db;

use std::io::stdin;

fn repl() {
    let mut eval = eval::Eval::new();
    loop {
        println!(">> ");
        // create buffer to store input
        // clear buffer after each iteration
        let mut buffer = String::with_capacity(2048);
        stdin().read_line(&mut buffer).unwrap();

        //TODO change eval mod and naming
        eval.eval(&buffer.trim_end());
        buffer.clear();
    }
}

fn main() {
    repl();
}
