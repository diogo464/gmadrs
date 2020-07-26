# GMADRS
![Rust](https://github.com/diogo464/gmadrs/workflows/Rust/badge.svg)
[![Crates.io version](https://img.shields.io/crates/v/gmadrs.svg)](https://crates.io/crates/gmadrs)

Command line utility to interact with .gma files, garry's mod addons.

* [Extract](#cmd-extract)
* [Create](#cmd-create)
* [Info](#cmd-info)
* [List](#cmd-list)
* [Contents](#cmd-contents)
* [Compress](#cmd-compress)
* [Uncompress](#cmd-uncompress)

## Commands
Extracts the contents of a .gma file


### [Extract](#cmd-extract)
```console
user@pc:~$ gmadrs extract --help 
```
```
gmadrs-extract 
Extracts a .gma file

USAGE:
    gmadrs extract [OPTIONS] <file>

ARGS:
    <file>    The file to decompress

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -d, --dir <dir>    The output directory

```



### [Create](#cmd-create)
```console
user@pc:~$ gmadrs create --help 
```
```
gmadrs-create 
Creates a .gma file from a folder

The folder should have the standard addon folder structure https://wiki.facepunch.com/gmod/Workshop_Addon_Creation

USAGE:
    gmadrs create <dir> <file>

ARGS:
    <dir>     
            The directory where addon is
    <file>    
            The file name

FLAGS:
    -h, --help       
            Prints help information

    -V, --version    
            Prints version information


```



### [Info](#cmd-info)
```console
user@pc:~$ gmadrs info --help 
```
```
gmadrs-info 
Prints information about a .gma file

USAGE:
    gmadrs info <file>

ARGS:
    <file>    The file to list

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

```



### [List](#cmd-list)
```console
user@pc:~$ gmadrs list --help 
```
```
gmadrs-list 
Lists the files in a gma archive

USAGE:
    gmadrs list [FLAGS] <file>

ARGS:
    <file>    The file to list

FLAGS:
    -h, --help       Prints help information
    -s, --size       Outputs the file sizes
    -V, --version    Prints version information

```



### [Contents](#cmd-contents)
```console
user@pc:~$ gmadrs contents --help 
```
```
gmadrs-contents 
Writes the contents of a file inside the archive to stdout

USAGE:
    gmadrs contents <file> <file-to-output>

ARGS:
    <file>              The gma file
    <file-to-output>    The file to print the contents of

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

```



### [Compress](#cmd-compress)
```console
user@pc:~$ gmadrs compress --help 
```
```
gmadrs-compress 
Compresses a given .gma file

USAGE:
    gmadrs compress [OPTIONS] <file>

ARGS:
    <file>    The file to decompress

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -o, --output <output>    The output file name

```



### [Uncompress](#cmd-uncompress)
```console
user@pc:~$ gmadrs uncompress --help 
```
```
gmadrs-uncompress 
Uncompresses a given .gma file

USAGE:
    gmadrs uncompress [OPTIONS] <file>

ARGS:
    <file>    The file to decompress

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -o, --output <output>    The output file name

```
