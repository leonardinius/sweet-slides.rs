fn main() {
    let mut v = vec![];
    v.push("Hello");
    let w = &v[0];
    v.push("world");
    println!("{} world", w);
}
