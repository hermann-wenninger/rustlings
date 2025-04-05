fn call_me(num: u8, mut numx: u32) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
        numx += i;
    }
    numx
}

fn main() {
    // TODO: Fix the function call.
    let x = call_me(7,3);
    println!("{x}");
}
