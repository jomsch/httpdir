# httpdir WIP,MVP

Serve a directory over http.   
Inspired from `python -m http.server`.  
**Caution use httpdir only in secure/local networks**


## Features

- [x] async
- [x] file upload
- [x] group by `directoryies`, `files`, `none`
- [x] sort by `atoz`, `ztoa`,
- [x] ignore dot files by default
- [ ] css styling(desktop and mobile)
- [ ] show file size & sortable by size
- [ ] ? bulk download of files 
- [ ] ? ignore specific files/filetypes
- [ ] ? setting for max directory depth


## Usage

```
Serve a direcotry over http

USAGE:
    httpdir [FLAGS] [OPTIONS] [dir]

FLAGS:
    -h, --help             Prints help information
        --show-dotfiles
    -V, --version          Prints version information

OPTIONS:
        --first-group-by <group-by>     [default: directories]
    -p, --port <port>                   [default: 8888]
        --sort <sort>                   [default: atoz]

ARGS:
    <dir>     [default: ./]
```

## Build


```
cargo build --release
```



