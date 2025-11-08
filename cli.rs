// ...existing code...
// From Copilot: This file sets up command-line argument parsing for the small curl-like program.
// From Copilot: It uses the `clap` crate, which is the de-facto standard for CLI parsing in Rust.
// From Copilot: The function `get_arguments` builds and returns the parsed arguments (an `ArgMatches` value).
use clap::{Arg, ArgMathces, Command};
// From Copilot: NOTE: There are a few typos / missing imports in the line above.
// From Copilot: - `ArgMathces` should be `ArgMatches` (spelling).
// From Copilot: - `ArgAction` is used below but not imported; you can import it with `ArgAction` or refer to it as `clap::ArgAction`.
// From Copilot: A corrected import would look like:
// From Copilot: `use clap::{Arg, ArgMatches, Command, ArgAction};`

// From Copilot: `get_arguments` constructs a `Command` (the CLI app definition) and returns the parsed matches.
// From Copilot: `ArgMatches` is the type returned by clap after parsing arguments; it gives accessors like `get_one` / `contains_id`.
pub fn get_arguments()-> ArgMatches{
    Command::new("Ccurl - custom curl")
        // From Copilot: `.about` describes the program; shown in `--help`.
        .about("It helps to make HTTP methods")
        .version("1.0")
        .author("me")
        // From Copilot: The next block defines a positional argument named "url".
        // From Copilot: `index(1)` makes it the first positional argument (no flag, just the first value).
        // From Copilot: `required(true)` means clap will error if it's not provided.
        .arg(
            Arg::new("url")
            .index(1)
            .required(true)  //default action is to "get one" value
        )
        // From Copilot: This defines an optional named argument `--x-method` (short `-X`) for the HTTP method.
        // From Copilot: `.help` is the help text shown in `--help`.
        .arg(
            Arg::new("x-method")
                .help("Http method which you want to use")
                .long("x-method")
                .short('X'),
        )
        // From Copilot: This defines an optional `--data` (short `-d`) argument for a request body.
        .arg(
            Arg::new("data")
                .help("Payload you want to send with the request")
                .long("data")
                .short('d'),
        )
        // From Copilot: This defines the `--header` / `-H` argument which can be provided multiple times.
        // From Copilot: The code intends to use `ArgAction::Append` so each `-H` occurrence is collected.
        // From Copilot: Note: `ArgAction` must be in scope (see import note above). Without it this will not compile.
        .arg(
            Arg::new("headers")
                .help("Request header")
                .long("header")
                .short('H')
                .action(ArgAction::Append),
        )
        // From Copilot: This defines a `--verbose` / `-v` flag. `SetTrue` stores `true` when the flag is present.
        // From Copilot: A flag like this does not accept a value; presence means enabled.
        .arg(
            Arg::new("verbose")
                .help("Verbose mode")
                .long("verbose")
                .short('v')
                .action(clap::ArgAction::SetTrue),  // Used because there is no corresponding value supplied with -v
        )
        // From Copilot: The final call triggers parsing and returns the matches.
        // From Copilot: NOTE: The method name here is `getMatches()` which is incorrect for clap's API.
        // From Copilot: The correct method is `get_matches()` (snake_case) in common clap versions, or `get_matches` depending on version.
        // From Copilot: So this line will not compile as written; change it to `.get_matches()` or `.get_matches()` according to your clap version.
        .get_matches()
}
// ...existing code...