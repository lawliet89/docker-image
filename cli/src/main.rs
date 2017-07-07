extern crate docker_image;
extern crate serde_json;

#[macro_use]
extern crate clap;

use std::io::{self, Write};
use clap::{Arg, App};
use docker_image::ImageName;

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
where
    'a: 'b,
{
    App::new("docker-image")
        .version(crate_version!())
        .author(crate_authors!())
        .about("Parse Docker Image names into their components")
        .arg(Arg::with_name("always_array").long("always-array").help(
            "Always output the results in an array, \
                  even if there is only one image name specified",
        ))
        .arg(
            Arg::with_name("image_names")
                .help("Image names to parse")
                .value_name("IMAGE")
                .required(true)
                .multiple(true),
        )
}

fn print<'a, W: Write>(
    images: &[ImageName<'a>],
    always_array: bool,
    mut writer: W,
) -> Result<(), String> {
    let json = if !always_array && images.len() == 1 {
        serde_json::to_string_pretty(&images[0])
    } else {
        serde_json::to_string_pretty(images)
    }.map_err(|e| e.to_string())?;

    writer.write_all(json.as_bytes()).map_err(|e| e.to_string())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;
    use super::*;

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
