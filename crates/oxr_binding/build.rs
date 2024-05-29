use std::env;
use std::path::PathBuf;

use tracing::info;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::fmt::SubscriberBuilder;

fn main() {
    let file_appender = RollingFileAppender::new(Rotation::NEVER, "build_logs", "build.log");
    let subscriber = SubscriberBuilder::default()
        .with_writer(file_appender)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let dst = cmake::Config::new("oxr_sdk")
        .define("CMAKE_INSTALL_LIBDIR", "lib")
        .define("DYNAMIC_LOADER", "OFF")
        .profile("Release")
        .define("BUILD_TESTS", "OFF")
        .define("BUILD_API_LAYERS", "OFF")
        .build();

    println!(
        "cargo:rustc-link-search=native={}",
        dst.join("lib").display()
    );

    println!("cargo:rustc-link-lib=static=openxr_loader");
    println!("cargo:rustc-link-lib=pathcch");

    let header_paths = vec!["oxr_sdk/include/openxr/openxr.h"];

    let header_paths: Vec<PathBuf> = header_paths
        .iter()
        .map(|path| PathBuf::from(path).canonicalize().unwrap())
        .collect();

    let path_strs = header_paths
        .iter()
        .map(|path| path.to_str().expect("Path is not a valid string"))
        .collect::<Vec<&str>>();

    let mut builder = bindgen::Builder::default();

    for headers_path_str in path_strs.iter() {
        builder = builder.header(*headers_path_str);
    }

    let bindings = builder
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    info!("Writing to {:?}.", env::current_dir());

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let crate_path = env::current_dir().unwrap();
    bindings
        .write_to_file(crate_path.join("src").join("bindings.rs"))
        .expect("Couldn't write bindings!");
}