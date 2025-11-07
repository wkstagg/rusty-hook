use clap::{Arg, ArgMathces, Command};

pub fn get_arguments()-> ArgMatches{
    Command::new("Ccurl - custom curl")
        .about("It helps to make HTTP methods")
        .version("1.0")
        .author("me")
        .arg(
            Arg::new("url")
            .index(1)
            .required(true)  //default action is to "get one" value
        )
        .arg(
            Arg::new("x-method")
                .help("Http method which you want to use")
                .long("x-method")
                .short('X'),
        )
        .arg(
            Arg::new("data")
                .help("Payload you want to send with the request")
                .long("data")
                .short('d'),
        )
        .arg(
            Arg::new("headers")
                .help("Request header")
                .long("header")
                .short('H')
                .action(ArgAction::Append),
        )
        .arg(
            Arg::new("verbose")
                .help("Verbose mode")
                .long("verbose")
                .short('v')
                .action(clap::ArgAction::SetTrue),  // Used because there is no corresponding value supplied with -v
        )
        .getMatches()
}