#[deriving(Clone)]
enum List {
    Cons(u32, ~List),
    Nil
}

fn cdr(list: &List) -> u32 {
    match list {
        &Cons(a, _) => a,
        _ => 0
    }
}

fn car(list: List) -> List {
    match list {
        Cons(_, ~b) => b,
        _ => Nil
    }
}

fn eq(a: &List, b: &List) -> bool {
    match (a, b) {
        (&Nil, &Nil) => true,
        (&Cons(x, ~ref aa), &Cons(y, ~ref bb)) if x == y => eq(aa, bb),
        _ => false
    }
}

fn main() {
    let cons = Cons(3, ~Cons(5, ~Nil));
    println!("{}", cdr(&cons));
    println!("{}", cdr(&car(cons)));
    if eq(&Cons(2, ~Nil), &Cons(2, ~Nil)) {
        println!("Equal!")
    } else {
        println!("Not equal :-(")
    }
}
