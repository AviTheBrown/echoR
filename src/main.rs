use clap::{App, Arg};

fn main() {
	// the _ (underscore) lets the compiler to 
	// ignmore unsused variable
    let matches = App::new("echoR")
	.version("0.1.0")
	.author("Avi Brown <avinbrown.focus@icloud.com>")
	.about("Rust echo implementation")
	.arg(
	     Arg::with_name("text")
	          .value_name("TEXT")
		  .help("Input text")
		  .required(true)
		  .min_values(1),
    	)
	.arg(
	    Arg::with_name("omit_newline")
		.help("Does not print newline")
		.takes_value(false)
		.short("n"),
	)
	.get_matches();

    	let text = matches.values_of_lossy("text").unwrap();
	let omit_newline = matches.is_present("omit_newline");
	
	let mut ending = if omit_newline {""} else {"\n"};

	println!("{}{}", text.join(" "), ending);

	    
   println!("{:#?}", matches);
}
