fn main() {
    let mut x = 5;
    { //Creating a new scope solves the problem
        let y = &mut x;
        *y += 1;
    }
    { //Creating a new scope solves the problem
        let z = &mut x;
        *z += 1;
    }
    println!("{}", x);
}
