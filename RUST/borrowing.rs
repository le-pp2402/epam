fn main() {

    let a: i32 = 11 ;
    
    print!("{}", num_of_steps(a));
}


fn num_of_steps(n: i32) -> i32 {
    if n == 1 {
        return 0;
    }

    if n % 2 == 0 {
        return 1 + num_of_steps(n / 2);
    } else {
        return 1 + num_of_steps(n * 3 + 1);
    }
}



