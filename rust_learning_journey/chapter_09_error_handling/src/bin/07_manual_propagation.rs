//07_manual_propagation
fn read_config()->Result<String, std::io::Error>
{
    let file=match std::fs::File::open("my_file.txt")
    {
        Ok(file)=>file,
        Err(e)=> return Err(e),
    };
}
fn main()
{
    let file=read_config();
    println!("File is={:?}",file);
}