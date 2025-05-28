use std::process::Command;

/// This function returns the nth character of a string.
pub fn nth_char(input: &str, n: i32) -> char {
        input.chars().nth(n as usize).unwrap_or('\0')
}


/// this function runs the out.c program.
pub fn run_program() {
        let compile_status = Command::new("gcc").arg("out.c").arg("-o").arg("out").status().expect("Failed to compile the program.");

        if !compile_status.success() {
                eprintln!("Compilation failed with status: {:?}", compile_status);
                return;
        }

        let run_status = Command::new("./out").status().expect("Failed to run the compiled binary.");

        if !run_status.success() {
                eprintln!("Program couldn't run.");
                return;
        }
}