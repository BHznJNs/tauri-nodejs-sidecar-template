// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use clap::Parser;
use daigent_lib::Args;

fn main() {
    let args = Args::parse();
    daigent_lib::run(args);
}
