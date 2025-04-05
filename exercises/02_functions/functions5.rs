
fn square(num: i32, x: &mut u8) -> i32 {
    *x +=21;
    num * num * num
}

fn main() {
    let mut x:u8 =1;
    
    let answer:i32 = square(3, &mut x) * square(5,&mut x) * square(29, &mut x);
    println!("The quart-square of 3 is {answer}");
    println!("the referenz ist updatet to    {x}");
    
}
