use structopt::StructOpt;

// search for a pattern in a file and display the lines that contain it
#[derive(StructOpt)]
struct Cli{
    //the pattern to look for
    pattern: String,
    //the pattern to the file to read
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf, //path is like a string, but for file systems paths (cross-platform)

}

fn main() {
    println!("Parsing command line arguments");
    
    //let pattern = std::env::args().nth(1).expect("no pattern given");
    //let path = std::env::args().nth(2).expect("no path given");
    
    let args = Cli::from_args();
    let content = std::fs::read_to_string(&args.path).expect("Could not read file"); 
                                        //.expect() method is shortcut that makes the program quit
                                        //immediately when the vlaue could not be read.
    for line in content.lines(){
        if line.contains(&args.pattern){
            println!("{}", line);
        }
    }
}
