//! `stacklyd`, the per user Stackly daemon.
//!
//! The daemon wires every dependency explicitly here. For now it resolves its
//! data directory, logs where it would run, and exits; the IPC server comes
//! with the daemon build.

use std::process::ExitCode;

use clap::Parser;
use stackly_core::DaemonInfo;

/// The per user Stackly daemon.
#[derive(Debug, Parser)]
#[command(version = stackly_core::daemon_info().version, about)]
struct Args {}

fn main() -> ExitCode {
    let _args = Args::parse();
    tracing_subscriber::fmt()
        .json()
        .with_writer(std::io::stderr)
        .init();
    run(stackly_core::daemon_info())
}

#[cfg(target_os = "macos")]
fn run(info: DaemonInfo) -> ExitCode {
    use stackly_platform::DirectoryResolver;
    use stackly_platform_macos::MacosDirectoryResolver;
    use stackly_protocol::ProtocolVersion;

    match MacosDirectoryResolver.app_support_dir() {
        Ok(dir) => {
            tracing::info!(
                version = info.version,
                protocol_major = ProtocolVersion::CURRENT.major,
                protocol_minor = ProtocolVersion::CURRENT.minor,
                app_support_dir = %dir.display(),
                "stacklyd started; the IPC server is not built yet, exiting",
            );
            ExitCode::SUCCESS
        }
        Err(err) => {
            tracing::error!(error = %err, "cannot resolve the Stackly data directory");
            ExitCode::FAILURE
        }
    }
}

#[cfg(not(target_os = "macos"))]
fn run(info: DaemonInfo) -> ExitCode {
    tracing::error!(
        version = info.version,
        "stacklyd only runs on macOS for now"
    );
    ExitCode::FAILURE
}
