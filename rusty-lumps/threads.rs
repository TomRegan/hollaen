//use std::task::try;

fn main() {
    let (port, chan): (Port<u32>, Chan<u32>) = Chan::new();
    do spawn {
        let mut n = 10;
        while n > 0 {
            chan.send(n);
            n -= 1;
        }
    }
    do spawn {
        loop {
            println!("{}", port.recv());
        }
    }
    // this and futures are a little wtf potato
    // let r: Result<<V9>, ~std::any::Any:Send> = do try {
    //     loop {
    //         println!("received {}", port.recv());
    //     }
    //};
}
