#![allow(non_snake_case)]
use std::process::exit;
use futures::executor::block_on;

use bodo_connect::cmd;
use clap::Parser;

#[tokio::main]
async fn main() {
    let mut cmd = cmd::Cmd::parse();
    exit(match block_on(cmd.main()) {
        Ok(..) => 0,
        Err(e) => {
            e.print_error();
            e.exit_code()
        }
    })
}

