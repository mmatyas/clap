// Copyright ⓒ 2015-2016 Kevin B. Knapp and [`clap-rs` contributors](https://github.com/kbknapp/clap-rs/blob/master/CONTRIBUTORS.md).
// Licensed under the MIT license
// (see LICENSE or <http://opensource.org/licenses/MIT>) All files in the project carrying such
// notice may not be copied, modified, or distributed except according to those terms.

//! `clap` is a simple-to-use, efficient, and full-featured library for parsing command line
//! arguments and subcommands when writing console/terminal applications.
//!
//! ## About
//!
//! `clap` is used to parse *and validate* the string of command line arguments provided by the user
//! at runtime. You provide the list of valid possibilities, and `clap` handles the rest. This means
//! you focus on your *applications* functionality, and less on the parsing and validating of
//! arguments.
//!
//! `clap` also provides the traditional version and help switches (or flags) 'for free' meaning
//! automatically with no configuration. It does this by checking the list of valid possibilities you
//! supplied and adding only the ones you haven't already defined. If you are using subcommands,
//! `clap` will also auto-generate a `help` subcommand for you in addition to the traditional flags.
//!
//! Once `clap` parses the user provided string of arguments, it returns the matches along with any
//! applicable values. If the user made an error or typo, `clap` informs them of the mistake and
//! exits gracefully (or returns a `Result` type and allows you to perform any clean up prior to
//! exit). Because of this, you can make reasonable assumptions in your code about the validity of
//! the arguments.
//!
//!
//! ## Quick Example
//!
//! The following examples show a quick example of some of the very basic functionality of `clap`.
//! For more advanced usage, such as requirements, conflicts, groups, multiple values and
//! occurrences see the [documentation](https://docs.rs/clap/), [examples/] directory of
//! this repository or the [video tutorials].
//!
//! **NOTE:** All of these examples are functionally the same, but show different styles in which to
//! use `clap`
//!
//! The first example shows a method that allows more advanced configuration options (not shown in
//! this small example), or even dynamically generating arguments when desired. The downside is it's
//! more verbose.
//!
//! ```no_run
//! // (Full example with detailed comments in examples/01b_quick_example.rs)
//! //
//! // This example demonstrates clap's full 'builder pattern' style of creating arguments which is
//! // more verbose, but allows easier editing, and at times more advanced options, or the possibility
//! // to generate arguments dynamically.
//! use clap::{Arg, App, };
//!
//! fn main() {
//!     let matches = App::new("My Super Program")
//!                           .version("1.0")
//!                           .author("Kevin K. <kbknapp@gmail.com>")
//!                           .about("Does awesome things")
//!                           .arg(Arg::with_name("config")
//!                                .short('c')
//!                                .long("config")
//!                                .value_name("FILE")
//!                                .help("Sets a custom config file")
//!                                .takes_value(true))
//!                           .arg(Arg::with_name("INPUT")
//!                                .help("Sets the input file to use")
//!                                .required(true)
//!                                .index(1))
//!                           .arg(Arg::with_name("v")
//!                                .short('v')
//!                                .multiple(true)
//!                                .help("Sets the level of verbosity"))
//!                           .subcommand(App::new("test")
//!                                       .about("controls testing features")
//!                                       .version("1.3")
//!                                       .author("Someone E. <someone_else@other.com>")
//!                                       .arg(Arg::with_name("debug")
//!                                           .short('d')
//!                                           .help("print debug information verbosely")))
//!                           .get_matches();
//!
//!     // Gets a value for config if supplied by user, or defaults to "default.conf"
//!     let config = matches.value_of("config").unwrap_or("default.conf");
//!     println!("Value for config: {}", config);
//!
//!     // Calling .unwrap() is safe here because "INPUT" is required (if "INPUT" wasn't
//!     // required we could have used an 'if let' to conditionally get the value)
//!     println!("Using input file: {}", matches.value_of("INPUT").unwrap());
//!
//!     // Vary the output based on how many times the user used the "verbose" flag
//!     // (i.e. 'myprog -v -v -v' or 'myprog -vvv' vs 'myprog -v'
//!     match matches.occurrences_of("v") {
//!         0 => println!("No verbose info"),
//!         1 => println!("Some verbose info"),
//!         2 => println!("Tons of verbose info"),
//!         3 | _ => println!("Don't be crazy"),
//!     }
//!
//!     // You can handle information about subcommands by requesting their matches by name
//!     // (as below), requesting just the name used, or both at the same time
//!     if let Some(matches) = matches.subcommand_matches("test") {
//!         if matches.is_present("debug") {
//!             println!("Printing debug info...");
//!         } else {
//!             println!("Printing normally...");
//!         }
//!     }
//!
//!     // more program logic goes here...
//! }
//! ```
//!
//! The next example shows a far less verbose method, but sacrifices some of the advanced
//! configuration options (not shown in this small example). This method also takes a *very* minor
//! runtime penalty.
//!
//! ```no_run
//! // (Full example with detailed comments in examples/01a_quick_example.rs)
//! //
//! // This example demonstrates clap's "usage strings" method of creating arguments
//! // which is less verbose
//! use clap::{Arg, App, };
//!
//! fn main() {
//!     let matches = App::new("myapp")
//!                           .version("1.0")
//!                           .author("Kevin K. <kbknapp@gmail.com>")
//!                           .about("Does awesome things")
//!                           .arg("-c, --config=[FILE] 'Sets a custom config file'")
//!                           .arg("<INPUT>              'Sets the input file to use'")
//!                           .arg("-v...                'Sets the level of verbosity'")
//!                           .subcommand(App::new("test")
//!                                       .about("controls testing features")
//!                                       .version("1.3")
//!                                       .author("Someone E. <someone_else@other.com>")
//!                                       .arg("-d, --debug 'Print debug information'"))
//!                           .get_matches();
//!
//!     // Same as previous example...
//! }
//! ```
//!
//! This third method shows how you can use a YAML file to build your CLI and keep your Rust source
//! tidy or support multiple localized translations by having different YAML files for each
//! localization.
//!
//! First, create the `cli.yml` file to hold your CLI options, but it could be called anything we
//! like:
//!
//! ```yaml
//! name: myapp
//! version: "1.0"
//! author: Kevin K. <kbknapp@gmail.com>
//! about: Does awesome things
//! args:
//!     - config:
//!         short: c
//!         long: config
//!         value_name: FILE
//!         help: Sets a custom config file
//!         takes_value: true
//!     - INPUT:
//!         help: Sets the input file to use
//!         required: true
//!         index: 1
//!     - verbose:
//!         short: v
//!         multiple: true
//!         help: Sets the level of verbosity
//! subcommands:
//!     - test:
//!         about: controls testing features
//!         version: "1.3"
//!         author: Someone E. <someone_else@other.com>
//!         args:
//!             - debug:
//!                 short: d
//!                 help: print debug information
//! ```
//!
//! Since this feature requires additional dependencies that not everyone may want, it is *not*
//! compiled in by default and we need to enable a feature flag in Cargo.toml:
//!
//! Simply change your `clap = "~2.27.0"` to `clap = {version = "~2.27.0", features = ["yaml"]}`.
//!
//! At last we create our `main.rs` file just like we would have with the previous two examples:
//!
//! ```ignore
//! // (Full example with detailed comments in examples/17_yaml.rs)
//! //
//! // This example demonstrates clap's building from YAML style of creating arguments which is far
//! // more clean, but takes a very small performance hit compared to the other two methods.
//! use clap::App;
//!
//! fn main() {
//!     // The YAML file is found relative to the current file, similar to how modules are found
//!     let yaml = load_yaml!("cli.yml");
//!     let matches = App::from_yaml(yaml).get_matches();
//!
//!     // Same as previous examples...
//! }
//! ```
//!
//! Finally there is a macro version, which is like a hybrid approach offering the speed of the
//! builder pattern (the first example), but without all the verbosity.
//!
//! ```no_run
//! use clap::clap_app;
//! fn main() {
//!     let matches = clap_app!(myapp =>
//!         (version: "1.0")
//!         (author: "Kevin K. <kbknapp@gmail.com>")
//!         (about: "Does awesome things")
//!         (@arg CONFIG: -c --config +takes_value "Sets a custom config file")
//!         (@arg INPUT: +required "Sets the input file to use")
//!         (@arg debug: -d ... "Sets the level of debugging information")
//!         (@subcommand test =>
//!             (about: "controls testing features")
//!             (version: "1.3")
//!             (author: "Someone E. <someone_else@other.com>")
//!             (@arg verbose: -v --verbose "Print test information verbosely")
//!         )
//!     ).get_matches();
//!
//!     // Same as before...
//! }
//! ```
//!
//! If you were to compile any of the above programs and run them with the flag `--help` or `-h` (or
//! `help` subcommand, since we defined `test` as a subcommand) the following would be output
//!
//! ```text
//! $ myprog --help
//! My Super Program 1.0
//! Kevin K. <kbknapp@gmail.com>
//! Does awesome things
//!
//! USAGE:
//!     MyApp [FLAGS] [OPTIONS] <INPUT> [SUBCOMMAND]
//!
//! FLAGS:
//!     -h, --help       Prints this message
//!     -v               Sets the level of verbosity
//!     -V, --version    Prints version information
//!
//! OPTIONS:
//!     -c, --config <FILE>    Sets a custom config file
//!
//! ARGS:
//!     INPUT    The input file to use
//!
//! SUBCOMMANDS:
//!     help    Prints this message
//!     test    Controls testing features
//! ```
//!
//! **NOTE:** You could also run `myapp test --help` to see similar output and options for the
//! `test` subcommand.
//!
//! ## Try it!
//!
//! ### Pre-Built Test
//!
//! To try out the pre-built example, use the following steps:
//!
//! * Clone the repository `$ git clone https://github.com/kbknapp/clap-rs && cd clap-rs/tests`
//! * Compile the example `$ cargo build --release`
//! * Run the help info `$ ./target/release/claptests --help`
//! * Play with the arguments!
//!
//! ### BYOB (Build Your Own Binary)
//!
//! To test out `clap`'s default auto-generated help/version follow these steps:
//!
//! * Create a new cargo project `$ cargo new fake --bin && cd fake`
//! * Add `clap` to your `Cargo.toml`
//!
//! ```toml
//! [dependencies]
//! clap = "2"
//! ```
//!
//! * Add the following to your `src/main.rs`
//!
//! ```no_run
//! use clap::App;
//!
//! fn main() {
//!   App::new("fake").version("v1.0-beta").get_matches();
//! }
//! ```
//!
//! * Build your program `$ cargo build --release`
//! * Run with help or version `$ ./target/release/fake --help` or `$ ./target/release/fake
//! --version`
//!
//! ## Usage
//!
//! For full usage, add `clap` as a dependency in your `Cargo.toml` (it is **highly** recommended to
//! use the `~major.minor.patch` style versions in your `Cargo.toml`, for more information see
//! [Compatibility Policy](#compatibility-policy)) to use from crates.io:
//!
//! ```toml
//! [dependencies]
//! clap = "~2.27.0"
//! ```
//!
//! Or get the latest changes from the master branch at github:
//!
//! ```toml
//! [dependencies.clap]
//! git = "https://github.com/kbknapp/clap-rs.git"
//! ```
//!
//! Define a list of valid arguments for your program (see the
//! [documentation](https://docs.rs/clap/) or [examples/] directory of this repo)
//!
//! Then run `cargo build` or `cargo update && cargo build` for your project.
//!
//! ### Optional Dependencies / Features
//!
//! #### Features enabled by default
//!
//! * `suggestions`: Turns on the `Did you mean '--myoption'?` feature for when users make typos. (builds dependency `strsim`)
//! * `color`: Turns on colored error messages. This feature only works on non-Windows OSs. (builds dependency `ansi-term` and `atty`)
//! * `wrap_help`: Wraps the help at the actual terminal width when
//!  available, instead of 120 chracters. (builds dependency `textwrap`
//! with feature `term_size`)
//!
//! To disable these, add this to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies.clap]
//! version = "~2.27.0"
//! default-features = false
//! ```
//!
//! You can also selectively enable only the features you'd like to include, by adding:
//!
//! ```toml
//! [dependencies.clap]
//! version = "~2.27.0"
//! default-features = false
//!
//! # Cherry-pick the features you'd like to use
//! features = [ "suggestions", "color" ]
//! ```
//!
//! #### Opt-in features
//!
//! * **"yaml"**: Enables building CLIs from YAML documents. (builds dependency `yaml-rust`)
//! * **"unstable"**: Enables unstable `clap` features that may change from release to release
//!
//! ### Dependencies Tree
//!
//! The following graphic depicts `clap`s dependency graph (generated using
//! [cargo-graph](https://github.com/kbknapp/cargo-graph)).
//!
//!  * **Dashed** Line: Optional dependency
//!  * **Red** Color: **NOT** included by default (must use cargo `features` to enable)
//!  * **Blue** Color: Dev dependency, only used while developing.
//!
//! ![clap dependencies](https://raw.githubusercontent.com/kbknapp/clap-rs/master/clap_dep_graph.png)
//!
//! ### More Information
//!
//! You can find complete documentation on the [docs.rs](https://docs.rs/clap/) for this project.
//!
//! You can also find usage examples in the [examples/] directory of this repo.
//!
//! #### Video Tutorials
//!
//! There's also the video tutorial series [Argument Parsing with Rust v2][video tutorials].
//!
//! These videos slowly trickle out as I finish them and currently a work in progress.
//!
//! ## How to Contribute
//!
//! Please read [CONTRIBUTING.md](https://raw.githubusercontent.com/clap-rs/clap/master/.github/CONTRIBUTING.md) before you start contributing.
//!
//! ### Goals
//!
//! There are a few goals of `clap` that I'd like to maintain throughout contributions. If your
//! proposed changes break, or go against any of these goals we'll discuss the changes further
//! before merging (but will *not* be ignored, all contributes are welcome!). These are by no means
//! hard-and-fast rules, as I'm no expert and break them myself from time to time (even if by
//! mistake or ignorance).
//!
//! * Remain backwards compatible when possible
//!   - If backwards compatibility *must* be broken, use deprecation warnings if at all possible before
//!   removing legacy code - This does not apply for security concerns
//! * Parse arguments quickly
//!   - Parsing of arguments shouldn't slow down usage of the main program - This is also true of
//!   generating help and usage information (although *slightly* less stringent, as the program is about
//!   to exit)
//! * Try to be cognizant of memory usage
//!   - Once parsing is complete, the memory footprint of `clap` should be low since the  main program
//!   is the star of the show
//! * `panic!` on *developer* error, exit gracefully on *end-user* error
//!
//! ### Compatibility Policy
//!
//! Because `clap` takes `SemVer` and compatibility seriously, this is the official policy regarding
//! breaking changes and previous versions of Rust.
//!
//! `clap` will pin the minimum required version of Rust to the CI builds. Bumping the minimum
//! version of Rust is considered a minor breaking change, meaning *at a minimum* the minor version
//! of `clap` will be bumped.
//!
//! In order to keep from being suprised of breaking changes, it is **highly** recommended to use
//! the `~major.minor.patch` style in your `Cargo.toml`:
//!
//! ```toml
//! [dependencies] clap = "~2.27.0"
//! ```
//!
//! This will cause *only* the patch version to be updated upon a `cargo update` call, and therefore
//! cannot break due to new features, or bumped minimum versions of Rust.
//!
//! #### Minimum Version of Rust
//!
//! `clap` will officially support current stable Rust, minus two releases, but may work with prior
//! releases as well. For example, current stable Rust at the time of this writing is 1.21.0,
//! meaning `clap` is guaranteed to compile with 1.19.0 and beyond. At the 1.22.0 release, `clap`
//! will be guaranteed to compile with 1.20.0 and beyond, etc.
//!
//! Upon bumping the minimum version of Rust (assuming it's within the stable-2 range), it *must* be
//! clearly annotated in the `CHANGELOG.md`
//!
//! ## License
//!
//! `clap` is licensed under the MIT license. Please read the [LICENSE-MIT][license] file in
//! this repository for more information.
//!
//! [examples/]: https://github.com/kbknapp/clap-rs/tree/master/examples
//! [video tutorials]: https://www.youtube.com/playlist?list=PLza5oFLQGTl2Z5T8g1pRkIynR3E0_pc7U
//! [license]: https://raw.githubusercontent.com/kbknapp/clap-rs/master/LICENSE-MIT

#![crate_type = "lib"]
#![doc(html_root_url = "https://docs.rs/clap/3.0.0-beta.1")]
#![deny(
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    unused_import_braces,
    unused_allocation,
    trivial_numeric_casts
)]

#[cfg(not(feature = "std"))]
compile_error!("`std` feature is currently required to build `clap`");

pub use crate::build::{App, AppSettings, Arg, ArgGroup, ArgSettings};
pub use crate::derive::{Clap, FromArgMatches, IntoApp, Subcommand};
pub use crate::parse::errors::{Error, ErrorKind, Result};
pub use crate::parse::{ArgMatches, OsValues, SubCommand, Values};

#[cfg(feature = "yaml")]
pub use yaml_rust::YamlLoader;

#[cfg(feature = "derive")]
#[cfg_attr(feature = "derive", doc(hidden))]
pub use clap_derive::{self, *};

#[cfg(feature = "derive")]
#[cfg_attr(feature = "derive", doc(hidden))]
pub use lazy_static;

#[macro_use]
#[allow(missing_docs)]
pub mod macros;

pub mod derive;

mod build;
mod mkeymap;
mod output;
mod parse;
mod util;

const INTERNAL_ERROR_MSG: &str = "Fatal internal error. Please consider filing a bug \
                                  report at https://github.com/clap-rs/clap/issues";
const INVALID_UTF8: &str = "unexpected invalid UTF-8 code point";
