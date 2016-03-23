#[deriving(Clone)]
enum OpCode {
    NOP,
    ERR
}

pub struct MyClass {
    priv message: ~str
}

impl MyClass {
    pub fn new(mess: ~str) -> MyClass {
        return MyClass { message: mess };
    }
    pub fn message(&mut self) -> ~str {
        return self.message.clone();
    }
    pub fn some_op_code(&mut self, opCode: ~OpCode) {
        match opCode {
            ~NOP => println!("no-op"),
            _ => println!("some-shit")
        };
    }
}

fn main() {
    let mut class = MyClass::new(~"Hello, World");
    println!("{}", class.message());
    class.some_op_code(~NOP);
}
