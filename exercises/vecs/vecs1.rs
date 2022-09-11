// vecs1.rs
// Your task is to create a `Vec` which holds the exact same elements
// as in the array `a`.
// Make me compile and pass the test!
// Execute `rustlings hint vecs1` or use the `hint` watch subcommand for a hint.

fn array_and_vec() -> ([i32; 4], Vec<i32>) {
    // arrays are stored on the stack, they have fixed size
    let a = [10, 20, 30, 40]; // a plain array
    // let v = vec!(10, 20, 30, 40); // TODO: declare your vector here with the macro for vectors

    // Note: vectors are stored on the heap so they can grow/shrink
    let mut v = Vec::new(); // alternative solution
    for e in a {
        v.push(e);
    }
    // Q: how to build vector from the existing array?
    (a, v)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_and_vec_similarity() {
        let (a, v) = array_and_vec();
        assert_eq!(a, v[..]);
    }
}
