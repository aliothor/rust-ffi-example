include!("../bindings/sample_bindings.rs");

fn main() {
    let r = unsafe { add(2, 18) };
    println!("{:?}", r);
}
