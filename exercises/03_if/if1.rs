fn bigger(a: i32, b: i32) -> i32 {
    if a>b{
        a
    }else{
        b
    }
}

fn bigger_ref(a: &i32, b: &i32) -> i32 {
    if a>b{
        *a
    }else{
        *b
    }
}

fn main() {
   let x:u8 = 11;
   let y:u8 = 23;
   println!("{}",bigger(x.into(),y.into()));
   let x:i32 =111;
   let y:i32 =222;
   println!("{}",bigger_ref(&x,&y));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ten_is_bigger_than_eight() {
        assert_eq!(10, bigger(10, 8));
    }

    #[test]
    fn fortytwo_is_bigger_than_thirtytwo() {
        assert_eq!(42, bigger(32, 42));
    }

    #[test]
    fn equal_numbers() {
        assert_eq!(42, bigger(42, 42));
    }
}
