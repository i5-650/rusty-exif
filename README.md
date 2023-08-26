# rusty-exif
<img src="froggy.png" alt="rusty-exif-icon" style="width:300px;height:auto;"/>

A simple exif tool for the command line written in Rust. 

## Usage
```
Usage: rusty-exif [OPTIONS]

Options:
  -e, --export <EXPORT>  
  -f, --file <FILE>      
  -F, --folder <FOLDER>  
  -s,                    Create a single json file for each image
  -h, --help             Print help information
  -V, --version          Print version information
```

## Examples
### Read exif data from an image and save it to a json file
```
rusty-exif -f image.jpg -e exif.json
```
### Read exif data from multiple images and save it to a json file
```
rusty-exif -F folder -e exif.json
```
### Read exif data from a file and print everything
```
rusty-exif -f image.jpg
```

## To-do
- [x] Read exif data
- [x] Write exif data in a json file (for one or multiple files)
- [ ] Write exif data in the image file (one or multiple files)
- [ ] Add a GUI
- [X] Add the argument to convert GPS coordinates into a google maps link
- [ ] Add the argument to convert GPS coordinates into an address and/or a screenshot of the location on a map.
- [ ] Modify parameters to use rusty-exif the same way as exiftool