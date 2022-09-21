fn main() {
    loop{
        println!("loop and break");
        break;
    }

    let mut n = 5;
    while(n != 0){
        println!("The number is {}",n);
        n -= 1;
    }
}
