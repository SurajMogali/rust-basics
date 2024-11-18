pub trait Summary {
    fn summarize(&self) -> String;
}



pub struct User{
    pub name:String,
     pub age:u32,

}

impl Summary for User{
    fn summarize(&self) -> String {
        return format!("User: {} - Age: {}", self.name, self.age);
    }
}


