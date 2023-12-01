use std::io;
use std::fs::File;
use std::fs;
use std::io::Read;
use std::str::Lines;
fn main () -> io::Result<()>
{
    //Open the text file containing the artistically-enhanced dataset
    let mut data_file: File = File::open("data.txt")?;
    
    //Rust makes us pre-allocate the memory the file will be read into
    let mut string: String = String::new();

    //Pull the file's contents into buffer
    data_file.read_to_string(&mut string)?;
    
    let lines: Lines<'_> = string.lines();

    let mut total: u32 = 0;

    for line in lines
    {
        let mut line_total: u32 = 0;
        for c in line.chars()
        {
            let mut first_num : u32 = 99;
            let result = c.to_digit(10);
            match result
            {
                Some(x) => first_num = x,
                None => ()
            }
            if(first_num > 0 && first_num < 10)
            {
                line_total += 10 * first_num;
                break;
            }
        }
        for c in line.chars().rev()
        {
            let mut second_num : u32 = 99;
            let result = c.to_digit(10);
            match result
            {
                Some(x) => second_num = x,
                None => ()
            }
            if(second_num > 0 && second_num < 10)
            {
                line_total += second_num;
                break;
            }
        }
        
        total += line_total;
        
    }
    println!("total: {}", total);
    Ok(())
} 