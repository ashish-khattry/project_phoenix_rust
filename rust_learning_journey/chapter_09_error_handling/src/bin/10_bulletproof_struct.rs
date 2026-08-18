//10_bulletproof_struct
pub struct Age{ value:i32 }
impl Age
{
    fn new(value:i32)->i32
    {
        if value>0 && value<150
        {
            value
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