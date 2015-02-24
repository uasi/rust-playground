#![feature(plugin)]
#![plugin(docopt_macros)] // Import `docopt!()`

extern crate docopt;
extern crate "rustc-serialize" as rustc_serialize; // Necessary for `docopt!()`

use docopt::Docopt;

docopt!(
Args derive Clone Debug, // The Args struct will be generated.
"
Usage:
    docopt [options] <arg1> <arg2> <argint>
    docopt --help

Options:
    -o, --opt1=<optarg1>    Optional argument 1.
    -O, --opt2=<optarg2>    Optional argument 2.
    -i, --int=<optint>      Optional integer value.
",
arg_argint: isize,  // You can optionally type a field (defaults to String).
flag_optint: isize, // Argument inconvertible to the type causes error.
);

fn main() {
    let args: Args =
        Args::docopt() // Get a Docopt,
             .decode() // decode argv into Result<Args, docopt::Error>,
             .unwrap_or_else(|e| e.exit()); // and then unwrap it or else
                                            // exit program printing usage.
    println!("{:?}", args);
    let _ensure_clonable = args.clone();
}
