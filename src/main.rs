use std::{
    env,
    process::{Command, Stdio},
    sync::Arc,
    thread::{self, sleep},
};

use opentelemetry::{global::shutdown_tracer_provider, trace::TraceError, KeyValue};
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::{trace as sdktrace, Resource};
use tracing::{instrument, Span};
use tracing_subscriber::{layer::SubscriberExt, Registry};

mod exit;
mod sync;

#[tokio::main]
async fn main() {
    let tracer = init_tracer().expect("Failed to initialize tracer.");
    let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);
    let subscriber = Registry::default().with(telemetry);
    // tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    tracing::subscriber::with_default(subscriber, || {
        shim_main();
    });

    shutdown_tracer_provider();
}

#[instrument(skip_all, parent = Span::current(), level= "Info")]
pub fn a1() {
    sleep(std::time::Duration::from_secs(1));
    println!("a1");
}

#[instrument(skip_all, parent = Span::current(), level= "Info")]
pub fn a2() {
    sleep(std::time::Duration::from_secs(2));
    println!("a2");
}

#[instrument(skip_all, parent = Span::current(), level= "Info")]
fn wait(exit_copy: Arc<exit::ExitSignal>) {
    exit_copy.wait();
}

#[instrument]
fn shim_main() {
    let exit = Arc::new(exit::ExitSignal::default());
    let exit_copy = exit.clone();
    let os_args: Vec<_> = env::args_os().collect();
    match os_args.get(1) {
        Some(arg) if arg == "1" => {
            a1();

            spawn();

            let join = thread::spawn(move || {
                sleep(std::time::Duration::from_secs(3));
                exit.signal();
            });

            join.join().unwrap();
        }
        Some(arg) if arg == "2" => {
            a2();

            wait(exit_copy);
        }
        _ => println!("Usage: {} <1|2>", os_args[0].to_string_lossy()),
    }
}

#[instrument]
fn spawn() {
    let cmd = env::current_exe().unwrap();
    let cwd = env::current_dir().unwrap();
    let mut command = Command::new(cmd);
    command.current_dir(cwd).args(["2"]);

    command
        .stdout(Stdio::null())
        .stdin(Stdio::null())
        .stderr(Stdio::null());

    command.spawn().unwrap();
}

fn init_tracer() -> Result<opentelemetry_sdk::trace::Tracer, TraceError> {
    opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .tonic()
                .with_endpoint("http://localhost:4317"),
        )
        .with_trace_config(
            sdktrace::config().with_resource(Resource::new(vec![KeyValue::new(
                "service.name",
                "instance3",
            )])),
        )
        // .install_batch(runtime::Tokio)
        .install_simple()
}
