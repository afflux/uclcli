/*
 * unucl - command line decompressor using libucl
 * Copyright (C) 2020-2021  BMW Group
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

#[macro_use]
extern crate clap;

use std::fs::OpenOptions;
use std::io::{self, Read, Write};

use anyhow::{Context, Result};
use memmap::MmapMut;

use uclcli::{decompress, decompress_into_buffer, ucl_init};

const DEFAULT_BUFFER_SIZE: u32 = 512 * 1024 * 1024;
fn main() -> Result<()> {
    let matches = clap_app!(unucl =>
        (version: "0.1")
        (author: "Kjell Braden <kjell.braden@bmw.de>")
        (about: "libucl (NRV) decompressor")
        (@arg INPUT: -i --input [FILE] "Sets the input file to use [defaults to stdin]")
        (@arg OUTPUT: -o --output [FILE] "Sets the output file to use [defaults to stdout]")
        (@arg bufsize: -b --buffersize [SIZE] "Sets the decompression buffer size - set this if you know how much data to expect after decompression [defaults to 512MB]")
    )
    .get_matches();

    ucl_init();

    let buffer_size = matches
        .value_of("bufsize")
        .map(|x| x.parse::<u32>().context("failed to parse --buffersize"))
        .unwrap_or(Ok(DEFAULT_BUFFER_SIZE))?;

    let mut input: Box<dyn Read> = match matches.value_of("INPUT") {
        Some(path) => Box::new(
            OpenOptions::new()
                .read(true)
                .open(&path)
                .context("could not open input file")?,
        ),
        None => Box::new(io::stdin()),
    };

    let mut inbuffer = Vec::new();
    input.read_to_end(&mut inbuffer)?;

    let output_filename = matches.value_of("OUTPUT");
    match output_filename {
        Some(path) => {
            let file = OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .open(&path)
                .context("could not create output file")?;
            file.set_len(buffer_size.into())
                .context("could not resize output file")?;

            let numbytes = unsafe {
                let mut mmap = MmapMut::map_mut(&file).context("failed to map output file")?;
                let nb =
                    decompress_into_buffer(&inbuffer, &mut mmap).context("decompression failed")?;
                mmap.flush().context("failed to write output")?;
                nb
            };
            file.set_len(numbytes.into())
                .context("failed to truncate output file")?;
        }
        None => {
            let dst = decompress(&inbuffer, buffer_size).context("decompression failed")?;
            io::stdout().write_all(&dst)?;
        }
    }

    Ok(())
}
