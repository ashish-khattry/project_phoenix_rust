//08_question_mark_magic
use std::io::Error;
use std::fs::File;
fn read_config()->Result<File,Error>
{
    let file=File::open("my_file.txt")?;
    return Ok(file);
}
fn main()
{
    let file=read_config();
    println!("{:?}",file);
}