use std::{env::args, fs::File, io::{self, copy, BufReader}, path::Path};

use flate2::{write::GzEncoder, Compression};
fn get_output_file_name(path:&str)->io::Result<String>{
    let path=Path::new(path);
    let mut file_name= if let Some(stem) =path.file_stem()  {
         if let Some(stem_str)=stem.to_str(){
             println!("FIle name is {}",stem_str);
             stem_str.to_string()
         }else{
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "File name couldn't be converted to str"));
         }
     }else {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Incorrect file name"));
    };
     
     let suffix = "_compress.pdf"; // Define what you want to append
     file_name.push_str(suffix);
     Ok(file_name)
}

fn main()->io::Result<()> {
    let arguments:Vec<String>=args().collect();
    println!("{arguments:?}");
    if arguments.len()!=3 {
        eprintln!("Enter correct arguments");
    }
    let input_file=&arguments[1];
    // let output_file=&arguments[2];
    let output_file=get_output_file_name(input_file).unwrap();
    let open=File::open(input_file)?;
    let mut input=BufReader::new(open);


    let create_new_file=File::create(&output_file)?;
    
    let mut encode=GzEncoder::new(create_new_file,Compression::default());


    copy(&mut input, &mut encode)?;
    println!(
        "Source len: {:?}",
        input.get_ref().metadata().unwrap().len()
    );
    println!("Target len: {:?}", encode.finish().unwrap().metadata().unwrap().len());
    Ok(())
}
