use std::fs;

#[derive(Debug, Clone)]
pub struct Emitter {
        pub full_path: String,
        pub header: String,
        pub code: String
}

impl Emitter {
        pub fn init(full_path: String) -> Self {
                Emitter { full_path: full_path, header: "".to_owned(), code: "".to_owned() }
        }

        pub fn set_path(&mut self, path: String) {
                self.full_path = path;
        }

        pub fn emit(&mut self, code: &str) {
                self.code = self.code.clone() + code;
        }

        pub fn emit_line(&mut self, code: &str) {
                self.code = self.code.clone() + code  + "\n";
        }

        pub fn header_line(&mut self, code: &str) {
                self.header = self.header.clone() + code  + "\n";
        }

        pub fn writeFile(&self) {
                fs::write(self.full_path.clone(), self.header.clone() + &self.code).unwrap();
        }
}