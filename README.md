# nh-toml-to-xml
A tool that lets you convert TOML to XML for New Horizons

## Usage

Run the command in the directory you would like the XML file to generate at.
(You have to add the app to your path, unless you want to run it directly every time)

`nh-toml-to-xml <path_to_toml_file>`

## Formatting

You can find a guide to using the TOML format [here](https://toml.io/en/v1.0.0). 
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

- Support for translating XML to TOML
- A configuration file for more customization
- Listen mode to update your XML as soon as your TOML changes

## Installation

You can install with the following methods:

- Direct download from release page (x86_64 Linux and Windows)
- Clone the repository and compile with the release flag (Necessary on every platform not included in the release page)

Whichever way you get the program, it is recommended that you put it in your shell's path so that
you can run it as a command from anywhere. The tool may be available in the future from the AUR or Cargo.
