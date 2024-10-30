# nh-toml-from
A tool that lets you convert TOML, JSON, YAML, and RON to XML for New Horizons

## Usage

Run the command in the directory you would like the XML file to generate at.
(You have to add the app to your path, unless you want to run it directly every time)

`nh-xml-from <path_to_input_file>`

## Formatting

Here are guides for each supported format:

- [toml](https://toml.io/en/v1.0.0) 
- [json](https://www.w3schools.com/js/js_json_intro.asp)
- [yaml](https://yaml.org/spec/1.2.2/)
- [ron](https://github.com/ron-rs/ron/blob/master/docs/grammar.md)

However, there are some things that are specific to the tool, such are the following:

- Names are lower_snake_case, excluding the header (NomaiObject, AstroObjectEntry, DialogueTree).
- Since TOML lacks some things in XML, some types changed, mainly self closing tags.
  - Location tags are arrays of strings: ["A", "B"]. Each string corresponds to a location tag
  - Every other self closing tag uses an optional boolean.
- Some other things that were originally strings are now different types in order to catch errors before the XML is generated.
  - `log_condition.reveal_fact.condition` is now an array of integers, since originally it was a string with text block ids
    separated by commas.
  - Any other fields that can be used multiple times are also wrapped in arrays

## Possible Updates

This software has reached it's original goal for completion. Here's a list of potential features to be added in newer versions:

- Support for translating from TOML to one of the supported formats
- A configuration file for more customization
- Listen mode to update your XML as soon as your configuration changes

## Installation

You can install with the following methods:

- Direct download from release page (x86_64 Linux and Windows)
- Clone the repository and compile with the release flag (Necessary on every platform not included in the release page)
- From crates.io using Cargo: `cargo install nh-xml-from`

Whichever way you get the program, it is recommended that you put it in your shell's path so that
you can run it as a command from anywhere. The tool may be available in the future from the AUR.
