use std::fs;
use std::io::{self, Read};
use std::path::PathBuf;
use std::env::current_dir;
use crate::builder::packer;

pub struct Builder {
    base_build: Vec<u8>,
}

impl Builder {
    pub fn new() -> Self {
        Builder {
             base_build: Vec::new(),
        }
    }

    // FROM
    pub fn config_base(&mut self, base: &str) {
        // all .wasm builds are local for now,
        // TODO: move to reg, cache locally
        match base {
            "scratch" => {
                println!("Scratch Build Started...");
            },
            "ruby:3.0" => {
                println!("...Ruby Build Started...\n");
                self.read_file("ruby.wasm");
                println!("Ruby Base Image located \n");
                let length = self.base_build.len();
            },
            "python:3" => {
            },
            "alpine:latest" => {
            },
            "ubuntu:latest" => {
            },
            "debian:latest" => {
            },
            "centos:latest" => {
            },
            "nginx:alpine" => {
            },
            _ => {
                println!("Base '{}' not recognized.", base);
            }
        }
    }

    // ADD, COPY
    pub fn bundle_fs(&mut self, guest_dir: &str, host_dir: &str, output_file: &str) -> io::Result<()> {
        // Map directories as needed
        let map_dirs = vec![(PathBuf::from(guest_dir), PathBuf::from(host_dir))];

        // Call the pack function from the packer module
        let output_bytes = packer::pack(&self.base_build, map_dirs)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        // Write the output bytes to the specified file
        let output_path = PathBuf::from(output_file);
        fs::write(output_path, output_bytes)?;

        Ok(())
    }

    // COMMAND, RUN, ENTRYPOINT
    pub fn expose_exec_interfaces(&self) {
    }

    pub fn build(&self, wasm_only: bool) {
    }

    fn read_file(&mut self, filename: &str) -> io::Result<()> {
        let mut path = PathBuf::from("./base_builds");
        path.push(filename);

        let mut file = fs::File::open(path)?;
        let mut contents = Vec::new();
        file.read_to_end(&mut contents)?;

        self.base_build = contents;
        Ok(())
    }
}
