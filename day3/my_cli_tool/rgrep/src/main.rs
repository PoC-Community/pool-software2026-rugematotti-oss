use clap::Parser;
mod highlight; 
mod search_in_file;
use crate::search_in_file::search_in_file;



#[derive(Parser)]
struct Args {
    pattern: String,
    file: String,
    #[arg(short = 'n', long = "line-numbers")]
    line_numbers: bool,

    #[arg(short = 'i', long = "case-insensitive")]
    ignore_case: bool,
      
      
}



fn main() {
    let args = Args::parse();
     if let Err(e) = run(args) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
    
}

fn run(args: Args) -> Result<(), Box<dyn std::error::Error>> {
    search_in_file(
        &args.pattern,
        &args.file,
        args.line_numbers,
        args.ignore_case,
    );

    Ok(())
}
