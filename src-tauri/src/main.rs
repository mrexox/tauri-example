// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[macro_use]
extern crate dotenvy_macro;

fn main() {
    dotenvy::dotenv().ok();

    app_lib::run();
}
