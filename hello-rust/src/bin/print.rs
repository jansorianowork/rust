//allows unused warnings
#![allow(unused)]

#[derive(Debug)]

//custom data type
struct Lang {
    language: String,
    version: String,
}

fn main() {
    let lang = "rust";
    // using format macro
    println!("Hello, {}!", lang);
    println!("Hello, {} {}!", lang, lang);
    println!("This is a simple {lang} program.");

    //positional arguments
    let x = 2;
    println!("{0} x {0} = {1}", x, x * x);

    //named arguments from struct
    let lang = Lang {
        language: "rust".to_string(),
        version: "1.83.0".to_string(),
    };

    println!("{:?}", lang);// works because of `Debug` derive
    println!("{:#?}", lang);
}
