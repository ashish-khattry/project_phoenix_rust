//06_cto_expect_message
//05_the_unwrap_shortcut
fn main()
{
    let dirty_num_string="xyz";
    let num:i32=dirty_num_string.trim().parse::<i32>().expect("CRITICAL: Failed to parse user ID");
    println!("Number is ={num}");
}