# rusty-exif
A simple exif tool for the command line written in Rust. 

## Usage

The base tool is designed to be used in two modes
```
Usage: rsexif <COMMAND>

Commands:
  file, -f, --file  Extract exif from a single file
  dir, -d, --dir    Extract exif from every files in a directory
  rm, -r, --remove  Remove exifs
  help              Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```
### Mode file
```
Usage: rsexif {file|--file|-f} [OPTIONS] <file>

Arguments:
  <file>  image to extract exif from

Options:
  -e, --export <export>  Json file to output exifs to
  -h, --help             Print help
```

### Mode Directory
```
Usage: rsexif {dir|--dir|-d} [OPTIONS] <folder>

Arguments:
  <folder>  directory containing images to extract exifs from

Options:
  -s, --split <split>    Wether you decide to store all exifs into one file or multiples [possible values: true, false]
  -e, --export <export>  The name of the Json file containing all the exifs
  -h, --help             Print help
```

### Mode Remove
```
Usage: rsexif {rm|--remove|-r} <path>

Arguments:
  <path>  file to remove exifs from

Options:
  -h, --help  Print help
```

## To-do
- [x] Read exif data
- [x] Write exif data in a json file (for one or multiple files)
- [ ] Write exif data in the image file (one or multiple files)
- [ ] Add a GUI
- [X] Add the argument to convert GPS coordinates into a google maps link
- [ ] Add the argument to convert GPS coordinates into an address and/or a screenshot of the location on a map.
- [X] Make a cleaner version of the CLI
