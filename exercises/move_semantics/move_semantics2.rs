// move_semantics2.rs
// Make me compile without changing line 13!
// Execute `rustlings hint move_semantics2` for hints :)


fn main() {
    let vec0 = Vec::<u32>::new();

    let mut vec1 = fill_vec();

    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec() -> Vec<i32> {
    let mut _vec = Vec::new();

    _vec.push(22);
    _vec.push(44);
    _vec.push(66);

    _vec
}
