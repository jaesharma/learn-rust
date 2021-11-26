fn primitives() {
    let logical: bool = true;

    let a_float: f64 = 9.23;
    let a_integer: i64 = 5i32; //suffix annotation

    //defaults
    let d_float = 9.234; //f64
    let d_integer = 234; //i32

    //type can also be inferred from context
    let mut inferred_type = 12; //type i64 is inferred from another line
    inferred_type = 82936432;
    //also, inferred_type is mutatable so its value can be changed.

    inferred_type = true;
    ///ERROR: type of variable can't be changed.
    //variables can be overwritten with shadowing.
    let inferred_type = true;
}
