# docker-image

A simple CLI tool to parse Docker image names into its components.

## Building and running it

You can build and run the CLI tool using `cargo run`.

## Usage

```bash
docker-image 0.1.0
Yong Wen Chua <me@yongwen.xyz>
Parse Docker Image names into their components

USAGE:
    docker-image [FLAGS] <IMAGE>...

FLAGS:
        --always-array    Always output the results in an array, even if there is only one image name specified
    -h, --help            Prints help information
    -V, --version         Prints version information

ARGS:
    <IMAGE>...    Image names to parse
```

## TODOs

- Fuzz
- YAML output
- Validation