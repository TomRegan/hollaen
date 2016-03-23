use std::gc::Gc;
use std::rc::Rc;

fn main() {
    let sq = |x: &u32| -> u32 { *x * *x };
    let x: ~u32 = ~2;
    let y: Gc<u32> = Gc::new(4);  // formerly '@' (managed box)
    let z: &u32 = &8;
    let r: Rc<u32> = Rc::new(16);

    println!("x squared is {:u}", sq(x));
    println!("y squared is {:u}", sq(y.borrow()));
    println!("z squared is {:u}", sq(z));
    println!("r squared is {:u}", sq(r.borrow()));

    let mut n = 31;
    {
        let m = &mut n;  // n is frozen in this block
        *m += 1;
        println!("m squared is {}", sq(m));
    }
    println!("n squared is {}", sq(&n));
}
