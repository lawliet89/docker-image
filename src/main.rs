extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate clap;
#[macro_use]
extern crate serde_derive;

use clap::{Arg, App};

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct ImageName<'a> {
    repository: Repository<'a>,
    tag: &'a str,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct Repository<'a> {
    registry: Option<&'a str>,
    path: &'a str,
}

fn main() {
    let args = make_parser().get_matches();
}

fn make_parser<'a, 'b>() -> App<'a, 'b>
    where 'a: 'b
{
    App::new("docker-image")
        .version(crate_version!())
        .author(crate_authors!())
        .about("Parse Docker Image names into their components")
        .arg(Arg::with_name("json")
                 .short("j")
                 .long("json")
                 .help("Output the parsed value as JSON (default)"))
        .arg(Arg::with_name("always_array")
                 .long("always-array")
                 .help("Always output the results in an array, \
                  even if there is only one image name specified"))
        .arg(Arg::with_name("image_names")
                 .help("Image names to parse")
                 .value_name("IMAGE")
                 .required(true)
                 .multiple(true))
}
