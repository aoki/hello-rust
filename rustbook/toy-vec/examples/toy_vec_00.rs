use toy_vec::ToyVec;

fn main() {
    let mut v = ToyVec::<usize>::new_with_capacity(5);
    // dbg!(&v);
    // println!("Length: {}", v.len());
    // println!("Capacity: {}", v.capacity());
    v.push(10);
    let e = v.get(0);
    print!("{}", e.unwrap);
}
