use atris::algebra;

fn main() {
    let v = algebra::Vector(5, 9);
    println!("{:?}", v.rotated(0));
    println!("{:?}", v.rotated(1));
    println!("{:?}", v.rotated(2));
    println!("{:?}", v.rotated(3));
    println!("{:?}", v.rotated(4));
    println!("{:?}", v.rotated(5));
}
