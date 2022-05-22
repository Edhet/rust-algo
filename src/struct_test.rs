struct Rectangle {
    x: f64,
    y: f64,
    z: f64,
}

impl Rectangle {
    fn volume (&self) -> f64 {
        return self.x * self.y * self.z
    }
    fn area (&self) -> f64 {
        return (self.x * self.y + self.z * self.y + self.x * self.z) * 2.0
    }
}

fn fodese() {
    let cubin = Rectangle {
        x: 12.0,
        y: 12.0,
        z: 12.0,
    };

    println!("Volume: {}", cubin.volume());
    println!("Area: {}", cubin.area());
}