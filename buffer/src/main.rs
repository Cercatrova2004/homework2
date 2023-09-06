struct Buffer<T> {
    data: Vec<T>,
}

impl<T> Buffer<T>
where
    T: std::ops::Add<Output = T> + Copy + Default,
{
    fn new() -> Self {
        Buffer { 
            data: Vec::new() 
        }
    }

    fn sum(&self) -> T {
        self.data.iter().fold(T::default(), |a, &b| a + b)
    }
}

fn main() {
    let mut buffer_int = Buffer::<i32>::new();
    buffer_int.data.push(1);
    buffer_int.data.push(2);
    buffer_int.data.push(3);
    buffer_int.data.push(4);
    println!("Integer sum: {}", buffer_int.sum());
    let mut buffer_float =Buffer::<f64>::new();
    buffer_float.data.push(1.0);
    buffer_float.data.push(2.0);
    buffer_float.data.push(3.0);
    buffer_float.data.push(4.0);
    println!("Float sum: {}", buffer_float.sum());
}
