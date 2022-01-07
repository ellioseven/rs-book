struct Number<T> {
    value: T,
}

// Implements `add` on generic `Number`.
impl<T> Number<T> {
    fn add(&self) {}
}

// Implements `i32_add` on `i32` `Number`.
impl Number<i32> {
    fn add_i32(&self) -> i32 {
        self.value + 1
    }
}


fn main() {
    let number = Number {
        value: 10 as i32
    };

    let value = number.add_i32();
    println!("{}", value);
}
