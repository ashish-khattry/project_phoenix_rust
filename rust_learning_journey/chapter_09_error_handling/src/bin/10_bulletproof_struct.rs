//10_bulletproof_struct
public struct Age{ value:i32 }
impl Age
{
    fn new(&self)->i32
    {
        if self.value>0 && self.value<150
        {
            self.age
        }
        else
        {
            panic!("Invalid age!");
        }
    }
}
fn main()
{
    let my_age=Age::new(25);
    println!("My age is ={}",my_age);
}