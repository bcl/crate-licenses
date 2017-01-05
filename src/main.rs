//! Extract crate license information for a project
//!
// Copyright (C) 2017
// Red Hat, Inc.  All rights reserved.
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation; either version 2 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

//! The expected input is the JSON output from cargo metadata, either as a file
//! or piped to it on stdin.

#![feature(proc_macro)]

use std::env;
use std::io::{self, Read};
use std::fs::File;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;

#[derive(Deserialize, Debug)]
struct Details {
    name: String,
    version: String,
    id: String,
    source: String,
    #[serde(default = "missing_license")]
    license: String,
    #[serde(default = "missing_lic_file")]
    license_file: String
}
#[derive(Deserialize, Debug)]
struct Packages {
    packages: Vec<Details>
}

fn missing_license() -> String {
    "MISSING".to_string()
}

fn missing_lic_file() -> String {
    "none".to_string()
}

fn main() {

    // Process args, if Nothing, or '-' is passed, read from stdin, otherwise treat it as a path
    let filename = env::args()
                        .nth(1)
                        .unwrap_or("".to_string());

    let mut buffer = String::new();
    match filename.as_str() {
        ""|"-" => {
            io::stdin()
                .read_to_string(&mut buffer)
                .expect("Problem reading input from stdin")
        }
        filename => {
            File::open(filename)
                .expect(&format!("Error opening {} for reading.", filename))
                .read_to_string(&mut buffer)
                .expect(&format!("Problem reading input from {}", filename))
        }
    };
    // Parse it as JSON
    let metadata: Packages = serde_json::from_str(&buffer).unwrap();

    for pkg in metadata.packages {
        println!("{:25} {:20} {:10}", pkg.name, pkg.license, pkg.license_file);
    }
}
