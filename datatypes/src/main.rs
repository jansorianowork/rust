fn main() {
    //datatypes
    let x: i8 = 10;
    println!("{}",x);

    let y: u8= 10;

    let decimal = 02_55;
    
    let hex = 0xff;

    let octal = 0o377;

    let binary = 0b1111_1111;
    //test
    let byte = b'A';

    println!("{}",y);
    println!("{}",decimal);
    println!("{}",hex);
    println!("{}",octal);
    println!("{}",binary);
    println!("{}",byte);

    let f = 2.0;//f64 default
    println!("{}",f);
    let ff: f32 = 1.0;
    println!("{}",ff);

    //boolean
    let t = true;
    println!("{}",t);
    let fa: bool = false;
    println!("{}",fa);

    //char
    let ch: &str = "c";
    println!("{}",ch);

    // + - * / %
    let a = 10;
    let b = 4;

    let remi = a % b;

    println!("{}",remi)

}
