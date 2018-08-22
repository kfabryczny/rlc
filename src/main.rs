#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate clap;

use clap::{Arg, App};

#[get("/")]
fn index() -> &'static str {
    "Rust log collector ;-)"
}

fn main() {
    let _matches = App::new("Rust log collector")
        .version("0.1.0")
        .author("klodus@gmail.com")
        .about("Rust log collector")
        .arg(Arg::with_name("ip")
             .long("ip")
             .required(true)
             .help("docker IP address"))
        .arg(Arg::with_name("port")
             .long("port")
             .required(true)
             .help("docker port"))
        .arg(Arg::with_name("path")
             .long("path")
             .required(true)
             .help("destination directory"))          
        .get_matches();


    rocket::ignite().mount("/", routes![index]).launch();
}


