mod eval;

use std::io::stdin;

fn repl() {
    loop {
        println!(">> ");
        // create buffer to store input
        // clear buffer after each iteration
        let mut buffer = String::with_capacity(2048);
        stdin().read_line(&mut buffer).unwrap();

        //TODO change eval mod and naming
        let eval = eval::Eval::new();
        eval.eval(&buffer.trim_end());
        buffer.clear();
    }
}

fn main() {
    repl();
}
