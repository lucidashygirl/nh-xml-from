# nh-toml-to-xml
A tool that lets you convert TOML to XML for New Horizons

## Usage

Run the command in the directory you would like the xml file to generate at.
(You have to add the app to your path, unless you want to run it directly every time)

`nh-toml-to-xml <path_to_toml_file>`

## Formatting

You can find a guide to using the TOML format [here](https://toml.io/en/v1.0.0). 
There are some things that are specific to the tool though, such are the following:

- Names are lower_snake_case, excluding the header (NomaiObject, AstroObjectEntry, DialogueTree).
- Since TOML lacks some things in XML, some types changed, mainly self closing tags.
  - Location tags are arrays of strings: ["A", "B"]. Each string cooresponds to a location tag
  - Every other self closing tag uses an optional boolean.
- Some other things that were originally strings are now different types in order to catch errors before the XML is generated.
  - `log_condition.reveal_fact.condition` is now an array of integers, since originally it was a string with text block ids
    seperated by commas.
  - nothing else lmao.

## Current Pitfalls

This software is not complete, but currently works fine. Here's a list of issues with the app as of v0.1.0:

- no support for translating dialogue
- no support for translating xml to toml
- errors are likely to be hard to read (i used unwraps everywhere :3c)

## Installation

You can install with the following methods:

- Direct download from release page (x86_64 Linux and Windows)
- Clone the repository and compile with the release flag (Necessary on every platform not included in the release page)

Whichever way you get the program, it is recommended that you put it in your shell's path so that
you can run it as a command from anywhere. The tool may be available in the future from the AUR or cargo.
