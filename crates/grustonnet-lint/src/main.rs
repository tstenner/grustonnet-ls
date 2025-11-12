use std::path::PathBuf;

use clap::Parser;
use env_logger::Env;
use grustonnet_ls_lib::{
    diagnostics::filter::JsonnetDiagnosticFilter, server::jsonnet::JsonnetServer,
};
use language_server::diagnostics::DiagnosticFilter;
use language_server::utils::{UriHelper, rope::RopeHelper};
use lsp_types::{DiagnosticSeverity, Uri};
use miette::{LabeledSpan, miette};
use ropey::Rope;
use std::collections::VecDeque;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    path: PathBuf,

    #[arg(long, short)]
    jpaths: Vec<String>,

    #[arg(long, default_value_t = 2)]
    fail_exit_code: i32,
}

trait SeverityMap {
    fn to_miette(&self) -> miette::Severity;
}

impl SeverityMap for DiagnosticSeverity {
    fn to_miette(&self) -> miette::Severity {
        match *self {
            DiagnosticSeverity::WARNING => miette::Severity::Warning,
            DiagnosticSeverity::ERROR => miette::Severity::Error,
            DiagnosticSeverity::INFORMATION | DiagnosticSeverity::HINT => miette::Severity::Advice,
            _ => miette::Severity::default(),
        }
    }
}

#[tokio::main]
async fn main() {
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

            std::process::Command::new(exe)
                .args(args)
                .spawn()
                .expect("Could not spawn child process")
                .wait()
                .unwrap();
            std::process::exit(0);
        }
    }

    #[cfg(feature = "tracing")]
    tracy_client::Client::start();
    let args = Args::parse();

    env_logger::Builder::from_env(Env::default().default_filter_or("fatal")).init();

    let paths: Vec<PathBuf> = if args.path.is_dir() {
        glob::glob(&format!("{}/**/*.*sonnet", args.path.to_str().unwrap()))
            .unwrap()
            .filter_map(|g| {
                if g.as_ref().ok()?.is_file() {
                    Some(g.ok()?)
                } else {
                    None
                }
            })
            .collect()
    } else {
        vec![args.path.clone()]
    };
    let server = JsonnetServer::default();
    server
        .configuration
        .write()
        .unwrap()
        .jsonnet
        .jpaths
        .extend(args.jpaths);
    server.cache.ast_generator.jsonnet.set_root_dir(".");
    // TODO: this needs to go (how many TODOs do I have for this cursed config?)
    server
        .cache
        .ast_generator
        .jsonnet
        .set_config(&server.configuration.read().unwrap().jsonnet);
    let filter = JsonnetDiagnosticFilter::new(server.cache.clone());
    let mut found_issues = false;
    for path in &paths {
        let diags = server.get_diagnostics(&Uri::from_path(path).unwrap());
        let diags = filter.filter_diagnostics(&Uri::from_path(path).unwrap(), diags);
        let content = std::fs::read_to_string(path).unwrap();
        if !diags.is_empty() {
            eprintln!("Lint results for {:?}", path);
            found_issues = true;
        }
        for diag in &diags {
            let source = content.clone();
            let rope = Rope::from_str(&source);
            let start = rope.get_index(diag.diagnostics.range.start);
            let end = rope.get_index(diag.diagnostics.range.end);
            let fix_text = if !diag.code_actions.is_empty() {
                Some(" (fix available in language server)".to_string())
            } else {
                None
            };
            let report = miette!(
                labels = vec![LabeledSpan::at(
                    start..end,
                    format!(
                        "{}{}",
                        diag.diagnostics.message.clone(),
                        fix_text.unwrap_or_default()
                    )
                ),],
                severity = diag
                    .diagnostics
                    .severity
                    .unwrap_or(DiagnosticSeverity::ERROR)
                    .to_miette(),
                "Linter result"
            )
            .with_source_code(source);
            eprintln!("{:?}", report)
        }
    }
    if found_issues {
        std::process::exit(args.fail_exit_code);
    }
}
