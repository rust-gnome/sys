extern crate pkg_config;

use pkg_config::{Config, Error};
use std::env;
use std::io::prelude::*;
use std::io;
use std::process;

fn main() {
    if let Err(s) = find() {
        let _ = writeln!(io::stderr(), "{}", s);
        process::exit(1);
    }
}

fn find() -> Result<(), Error> {
    let package_name = "gdk-pixbuf-2.0";
    let shared_libs = ["gdk_pixbuf-2.0"];
    let version = if cfg!(feature = "v2_36_8") {
        "2.36.8"
    } else if cfg!(feature = "v2_36") {
        "2.36"
    } else if cfg!(feature = "v2_32") {
        "2.32"
    } else if cfg!(feature = "v2_30") {
        "2.30"
    } else if cfg!(feature = "v2_28") {
        "2.28"
    } else {
        "2.26"
    };

    if let Ok(lib_dir) = env::var("GTK_LIB_DIR") {
        for lib_ in shared_libs.iter() {
            println!("cargo:rustc-link-lib=dylib={}", lib_);
        }
        println!("cargo:rustc-link-search=native={}", lib_dir);
        return Ok(())
    }

    let mut config = Config::new();
    config.atleast_version(version);
    config.print_system_libs(false);
    match config.probe(package_name) {
        Ok(_) => Ok(()),
        Err(Error::EnvNoPkgConfig(_)) | Err(Error::Command { .. }) => {
            for lib_ in shared_libs.iter() {
                println!("cargo:rustc-link-lib=dylib={}", lib_);
            }
            Ok(())
        }
        Err(err) => Err(err),
    }
}

