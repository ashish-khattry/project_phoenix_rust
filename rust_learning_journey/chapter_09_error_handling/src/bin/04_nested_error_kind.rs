//04_nested_error_kind
fn main()
{// this program has to fix later 
    let file=match std::fs::File::open("secret.txt")
    {
        Ok(file)=>file,
        Err(file_error)=> match file_error.kind()
        {
            Ok(NotFound)=>
            {
                std::fs::create("secret.txt")
                {
                    Ok(file)=>file,
                    Err(_)=>
                    {
                        println!("File creation failed!");
                        return;
                    }
                }
            }
            Err(_)=>
            {
                println!("Other error");
                return;
            }
        };
    }
}