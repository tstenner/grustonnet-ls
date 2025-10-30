use clap::Parser;
use env_logger::Env;
use grustonnet_ls_lib::server::{config::Configuration, jsonnet::JsonnetServer};
use language_server::server::{LSPConnection, LSPServerManager};
use schemars::schema_for;
use std::collections::VecDeque;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(long)]
    export_config_schema: bool,

    #[arg(long, short)]
    port: Option<u16>,
}

#[tokio::main]
async fn main() {
    // Go seems to scan the stack an will panic upon encountering a 0x1 pointer.
    // However, Rust does use this value in some cases
    // Just setting the variable is not enough. Therefore we'll set the environment
    // variable and restart the current program
    // If this turns out to be a problem we'll need to switch to an ipc based solution

    if std::env::var("GODEBUG").is_err() {
        // At this point we are single threaded. Therefore this is safe

        unsafe {
            std::env::set_var("GODEBUG", "invalidptr=0,cgocheck=0");
        }

        let exe = std::env::current_exe().expect("Could not get path to the current executable");

        // On Unix we can just use execvp and replace the current process
        #[cfg(unix)]
        {
            let args: VecDeque<String> = std::env::args().collect();
            let err = exec::execvp(&exe, &args);
            eprintln!("Failed to restart with GODEBUG: {}", err);
            std::process::exit(1);
        }
        // Windows does not support essential features and therefore we just spawn a child process
        // and pass over stdin. This results in more memory usage, but that is the life on Windows
        #[cfg(not(unix))]
        {
            let mut args: VecDeque<String> = std::env::args().collect();
            println!("Args {:?}", args);
            // Pop first argument = executable
            args.pop_front();

            //std::process::Command::new(exe)
            //    .args(args)
            //    .spawn()
            //    .expect("Could not spawn child process")
            //    .wait()
            //    .unwrap();
            std::process::exit(0);
        }
    }

    #[cfg(feature = "tracing")]
    tracy_client::Client::start();
    let args = Args::parse();

    if args.export_config_schema {
        println!(
            "{}",
            serde_json::to_string_pretty(&schema_for!(Configuration)).unwrap()
        );
        return;
    }
    env_logger::Builder::from_env(Env::default().default_filter_or("warn")).init();
    let connection = if let Some(port) = args.port {
        LSPConnection::new_network(port)
    } else {
        LSPConnection::default()
    };
    let server = LSPServerManager {
        server: JsonnetServer::new(connection),
    };
    server.run().unwrap();
}
