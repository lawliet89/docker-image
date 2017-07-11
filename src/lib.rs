//! A small library to parse Docker image names into their respective components
//!
//! For the CLI tool, refer to the [repository](https://github.com/lawliet89/docker-image)
//!
//! # Usage
//! ```toml
//! [dependencies]
//! docker_image = { git = "https://github.com/lawliet89/docker-image", branch = "master" }
//! ```
//!
//! # Examples
//! ```rust
//! let image = "ubuntu:latest";
//! let parsed = docker_image::ImageName::parse(image);
//! ```
#![deny(missing_docs)]
#![doc(test(attr(allow(unused_variables), deny(warnings))))]

extern crate serde;

#[macro_use]
extern crate serde_derive;

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
    pub repository: &'a str,
    /// A tag serves to map a descriptive, user-given name to any single image ID.
    /// Tag values are limited to the set of characters `[a-zA-Z0-9_.-]`,
    /// except they may not start with a . or - character.
    /// Tags are limited to 128 characters.
    pub tag: Option<&'a str>,
}

impl<'a> ImageName<'a> {
    /// Parse an image string into its constituent components
    pub fn parse(image: &'a str) -> ImageName<'a> {
        // Separate the image name by the `:` seperator
        let colon_seperated: Vec<&'a str> = image.rsplit(':').collect();
        let tag = if colon_seperated.len() > 1 {
            Some(colon_seperated[0])
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

/// Parse an image string into its constituent components
pub fn parse<'a>(image: &'a str) -> ImageName<'a> {
    ImageName::parse(image)
}

#[cfg(test)]
mod tests {
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
}
