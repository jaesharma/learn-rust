struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

pub fn rectangle_impl() {
    let mut rect = Rectangle {
        width: 10,
        height: 10,
    };

    // println!("rect: {:#?}", rect);
    let Rectangle {
        width: rW,
        height: rH,
    } = rect;

    println!("rectangle area: {} - {}", rW, rH);
}
