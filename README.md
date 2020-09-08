# fast5wrapper: a tiny crate to incorpolate with .fast5 file

- author: Bansho Masutani<banmasutani@gmail.com>
- language: Rust + Python

## Short summary
  This is a tiny crate to extract raw-signal/read id/event stream from fast5 file by using h5py library in python.

## How to use

0. clone this repo
```
hg clone https://ban-m@bitbucket.org/ban-m/fast5_wrapper/
```
1. Add your Cargo.toml
```
[dependencies]
fast5wrapper = {path = "path/to/this/repository"}
```
2. Import crate and use it.

For more detail, see source code or rustdoc.

## Requirement

- Rust 1.2 or later
- Python 2.7
- Python 3.3 to 3.6
- h5py 
- ont_fast5_api