# What is Director?

Director prints out directories and their sizes, sorted by size. 

## Why did I make Director?

My laptop was running out of space, and Director helps me find the largest folders and files. I tried using the 'du' command, but Director is better because
- it is faster
- it sorts results
- it prints out sizes in a readable format

## Usage

This repo contains a file called director.exe
```
director DIRECTORY
```

## Examples

Pass in nothing to use the current directory
```
director
```

Pass in a specific directory
```
director ../
```

## Build commands

Build for development (must have rust installed)
```
cargo build
```

Build release
```
build.sh
```