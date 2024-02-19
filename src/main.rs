use std::io::{stdin, stdout, Write};
use std::process::Command;

fn main(){
    loop {
        // use the `>` character as the prompt
        // need to explicitly flush this to ensure it prints before read_line
        print!("> ");
        //Ignoring the result of stdout().flush()
        let _ = stdout().flush();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let command = input.trim();

        let mut child = Command::new(command)
            .spawn()
            .unwrap();

        // don't accept another command until this one completes
        //Ignoring the result of child.wait()
        let _ = child.wait();
    }
}
