extern crate clap;

use clap::{App, AppSettings};

fn main() {
    let pargs = App::new("Extract splitted and discordant reads from bam")
        .version("0.0.1")
        .author("liuxiaochuan@novogene.com")
        .about("Extract splitted and discordant reads from bam")
        .setting(AppSettings::ArgRequiredElseHelp)
        .args_from_usage(
            "   
            [verbose] -v, --verbose 'boolean input do not take value'
            <input> 'required input'
            ",
        )
        .get_matches();
    println!("{:?}", pargs.value_of("verbose"));
    println!("{:?}", pargs.value_of("input"));
}
