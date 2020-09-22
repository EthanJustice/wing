# wing

static site generator

+ [Roadmap](#roadmap)
+ [Install](#install)
+ [Usage](#usage)

## Roadmap

+ Hooks (pre- & post-install)
+ Custom data (`custom-data` branch)
  + Custom templates (`yaml!()`?)
+ File metadata (that will be available in templates)
+ Current entry metadata
+ Built-in helpers
+ Complete [rhai](https://lib.rs/crates/rhai) support
  + `helpers` field in config
+ Syntax highlighting with [syntect](https://lib.rs/crates/syntect)

## Install

For now, as Wing is not on [`crates.io`](https://crates.io/) and CI hasn't been set up, installation has to be done through Cargo.

`cargo install --git https://github.com/EthanJustice/wing.git`

## Usage

[Auto-generated by clap]

```text
USAGE:
    wing_generate [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    build    Builds your site.
    help     Prints this message or the help of the given subcommand(s)
    new      Create a new wing project.
```

For now, `wing` doesn't support custom templates.  You can only use `index.hbs` for generation.

### Custom Metadata

For now, `wing` doesn't support custom metadata, although it provides a small amount that can be used in templates.  `custom` contains the MarkDown of the current file in HTML; `items` contains the index of files.

### Wing Config
