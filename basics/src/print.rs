pub fn run(){
    //print to console
    println!("Hello from the print.rs file");

    //basic formatting
    println!("Test of {}-{}", "basic", "formatting");

    //Positional arguments
    println!("{0} will be typed thrice in this {1}, once here {0}, then here {0}, & at the end here {0}.", "lorem", "statement");

    //named arguments
    println!("{name} likes to play {activity}", name="user", activity="football");

    //placeholder traits
    println!("binary {:b}, hex {:x} & oct {:o}.", 10, 10, 10);

    //placeholder for debug traits
    println!("{:?}", (12, true, "string value"));
}