use toy_vec::ToyVec;

fn main() {
    let v = ToyVec::<String>::new();
    dbg!(&v);
    println!("Length: {}", v.len());
    println!("Capacity: {}", v.capacity());
}
