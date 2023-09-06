pub trait summable<T>{
    fn sum(&self)->T;
}

pub struct buffer<T>
where T: std::ops::Add<Output = T> + Copy{
    data:Vec<T>
}

pub impl <T> summable<T> for buffer<T>
where T: std::ops::Add<Output = T> + Copy{
    pub fn new(members:Vec<T>)->Self{
        Buffer::<T>{
            members
        }
    }
    pub fn sum(&self)->Option<T>{
        let mut buf = self.members[0];
        for i in 1..self.members.len(){
            buf = buf + self.members[i];
        }
        buf
    }
}