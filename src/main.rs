extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate clap;
#[macro_use]
extern crate serde_derive;

use std::io::{self, Write};
use clap::{Arg, App};

// TODO: Validation
// TODO: Fuzz

/// A Parsed Docker Image name
///
/// Refer to the Docker image
/// [specification](https://github.com/moby/moby/blob/master/image/spec/v1.2.md)
/// for more details.
#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct ImageName<'a> {
    /// A collection of tags grouped under a common prefix (the name component before `:`).
    /// For example, in an image tagged with the name `my-app:3.1.4`, `my-app`
    /// is the Repository component of the name.
    /// A repository name is made up of slash-separated name components,
    /// optionally prefixed by a DNS hostname. The hostname must comply with standard DNS rules,
    /// but may not contain `_` characters.
    /// If a hostname is present, it may optionally be followed by a port number in the format
    /// `:8080`.
    /// Name components may contain lowercase characters, digits, and separators.
    /// A separator is defined as a period, one or two underscores, or one or more dashes.
    /// A name component may not start or end with a separator.
    repository: &'a str,
    /// A tag serves to map a descriptive, user-given name to any single image ID.
    /// Tag values are limited to the set of characters `[a-zA-Z0-9_.-]`,
    /// except they may not start with a . or - character.
    /// Tags are limited to 128 characters.
    tag: Option<&'a str>,
}

impl<'a> ImageName<'a> {
    fn parse(image: &'a str) -> ImageName<'a> {
        // Separate the image name by the `:` seperator
        let colon_seperated: Vec<&'a str> = image.rsplit(':').collect();
        let tag = if colon_seperated.len() > 1 {
            Some(colon_seperated[0].clone())
        } else {
            None
        };

        let repository = match tag {
            None => image,
            Some(tag) => image.trim_right_matches(tag).trim_right_matches(':'),
        };

        ImageName { repository, tag }
    }
}

fn main() {
    let args = make_parser().get_matches();
    let images: Vec<ImageName> = args.values_of("image_names")
        .expect("Missing values should have been caught by validator!")
        .map(ImageName::parse)
        .collect();
    let always_array = args.occurrences_of("always_array") > 0;
    match print(&images, always_array, io::stdout()) {
        Ok(()) => {}
        Err(e) => panic!("{}", e),
    }
}

fn make_parser<'a, 'b>() -> App<'a, 'b>
    where 'a: 'b
{
    App::new("docker-image")
        .version(crate_version!())
        .author(crate_authors!())
        .about("Parse Docker Image names into their components")
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

fn print<'a, W: Write>(images: &[ImageName<'a>],
                       always_array: bool,
                       mut writer: W)
                       -> Result<(), String> {
    let json = if !always_array && images.len() == 1 {
            serde_json::to_string_pretty(&images[0])
        } else {
            serde_json::to_string_pretty(images)
        }
        .map_err(|e| e.to_string())?;

    writer
        .write_all(json.as_bytes())
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;
    use super::*;

    #[test]
    fn full_image_name_parsing() {
        let image = "registry.example.com/hello/world:v1.0.0";
        let parsed = ImageName::parse(image);

        let expected = ImageName {
            repository: "registry.example.com/hello/world",
            tag: Some("v1.0.0"),
        };

        assert_eq!(expected, parsed);
    }

    #[test]
    fn parse_image_without_hostname_and_tag() {
        let image = "hello-world";
        let parsed = ImageName::parse(image);

        let expected = ImageName {
            repository: "hello-world",
            tag: None,
        };

        assert_eq!(expected, parsed);
    }

    #[test]
    fn parse_image_without_hostname() {
        let image = "hello-world:v1.0.0";
        let parsed = ImageName::parse(image);

        let expected = ImageName {
            repository: "hello-world",
            tag: Some("v1.0.0"),
        };

        assert_eq!(expected, parsed);
    }

    #[test]
    fn prints_out_multiple_images() {
        let images = vec!["registry.example.com/hello/world:v1.0.0", "hello-world"];
        let parsed: Vec<ImageName> = images.iter().map(|s| ImageName::parse(*s)).collect();
        let buffer: Vec<u8> = vec![];
        let mut cursor = Cursor::new(buffer);

        print(&parsed, false, &mut cursor).unwrap();

        let expected_json = r#"[
  {
    "repository": "registry.example.com/hello/world",
    "tag": "v1.0.0"
  },
  {
    "repository": "hello-world",
    "tag": null
  }
]"#;
        let actual_json = String::from_utf8(cursor.into_inner()).unwrap();
        assert_eq!(expected_json, actual_json);
    }

    #[test]
    fn prints_out_single_image_without_array() {
        let images = vec!["hello-world"];
        let parsed: Vec<ImageName> = images.iter().map(|s| ImageName::parse(*s)).collect();
        let buffer: Vec<u8> = vec![];
        let mut cursor = Cursor::new(buffer);

        print(&parsed, false, &mut cursor).unwrap();

        let expected_json = r#"{
  "repository": "hello-world",
  "tag": null
}"#;
        let actual_json = String::from_utf8(cursor.into_inner()).unwrap();
        assert_eq!(expected_json, actual_json);
    }

    #[test]
    fn prints_out_single_image_with_array() {
        let images = vec!["hello-world"];
        let parsed: Vec<ImageName> = images.iter().map(|s| ImageName::parse(*s)).collect();
        let buffer: Vec<u8> = vec![];
        let mut cursor = Cursor::new(buffer);

        print(&parsed, true, &mut cursor).unwrap();

        let expected_json = r#"[
  {
    "repository": "hello-world",
    "tag": null
  }
]"#;
        let actual_json = String::from_utf8(cursor.into_inner()).unwrap();
        assert_eq!(expected_json, actual_json);
    }
}
