# uclcli - cli for libucl

## Usage examples

Decompression:

```
$ unucl --help
unucl 0.1
Kjell Braden <kjell.braden@bmw.de>
libucl (NRV) decompressor

USAGE:
    unucl [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -i, --input <FILE>         Sets the input file to use [defaults to stdin]
    -o, --output <FILE>        Sets the output file to use [defaults to stdout]
    -b, --buffersize <SIZE>    Sets the decompression buffer size - set this if you know how much data to expect after
                               decompression [defaults to 512MB]
```

Compression:
```
$ ucl --help
ucl 0.1
Kjell Braden <kjell.braden@bmw.de>
libucl (NRV) compressor

USAGE:
    ucl [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -i, --input <FILE>     Sets the input file to use [defaults to stdin]
    -o, --output <FILE>    Sets the output file to use [defaults to stdout]
```

## License
As it links to libucl, uclcli is licensed under GPLv3.

```
Copyright (C) 2020-2021  BMW Group

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or any
later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
```
