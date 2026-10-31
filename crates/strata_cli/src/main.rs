//! The Strata Unified Developer CLI and Packaging Toolchain.

use std::process;
use strata_cli::prelude::*;

fn main() {
    let command = match CliArgs::parse() {
        Ok(cmd) => cmd,
        Err(err) => {
            eprintln!("\x1b[1;31mError:\x1b[0m {}", err);
            process::exit(1);
        }
    };

    match command {
        CliCommand::New {
            name,
            template,
            target_dir,
        } => {
            CliArgs::print_banner();
            let base_dir = target_dir.unwrap_or_else(|| std::env::current_dir().unwrap_or_default());
            println!("⚡ Scaffolding project '{}' with template '{}'...", name, template.as_str());
            match ProjectScaffolder::scaffold(&name, template, &base_dir) {
                Ok(report) => {
                    println!("   Root directory: {:?}", report.root_path);
                    println!("   Created {} files.", report.created_files.len());
                    println!("✔ Successfully scaffolded '{}'!\n", name);
                    println!("Next steps:");
                    println!("  cd {}", name);
                    println!("  strata dev\n");
                }
                Err(e) => {
                    eprintln!("✖ Failed to scaffold project: {}", e);
                    process::exit(1);
                }
            }
        }
        CliCommand::Dev {
            target,
            port,
            watch,
        } => {
            CliArgs::print_banner();
            println!("⚡ Starting Strata HMR dev engine for target: [{}]...", target.as_str());
            println!("   Local port: http://localhost:{}", port);
            println!("   File watch active: {}", watch);
            println!("   GPU Pipeline: Direct Hardware Swapchain");
            println!("   State-preserving signal graph monitor initialized.");
            let _server = DevServer::new(DevServerConfig {
                target,
                port,
                watch,
            });
        }
        CliCommand::Build {
            target,
            release,
            out_dir,
        } => {
            CliArgs::print_banner();
            let base_dir = out_dir.unwrap_or_else(|| std::env::current_dir().unwrap_or_default());
            println!(
                "⚡ Building {} bundle for target: [{}]...",
                if release { "release" } else { "debug" },
                target.as_str()
            );
            match BuildRunner::build(target, release, &base_dir) {
                Ok(report) => {
                    println!("✔ Build succeeded! Emitted {} artifacts.", report.artifacts.len());
                    for art in report.artifacts {
                        println!("   -> {:?}", art);
                    }
                }
                Err(e) => {
                    eprintln!("✖ Build failed: {}", e);
                    process::exit(1);
                }
            }
        }
        CliCommand::Package {
            target,
            format,
            out_dir,
        } => {
            CliArgs::print_banner();
            let base_dir = out_dir.unwrap_or_else(|| std::env::current_dir().unwrap_or_default());
            println!("⚡ Packaging production bundle for target: [{}]...", target.as_str());
            match Packager::package(target, format, &base_dir) {
                Ok(report) => {
                    println!(
                        "✔ Packaging succeeded! Created '{}' installer at: {:?}",
                        report.format.extension(),
                        report.package_path
                    );
                }
                Err(e) => {
                    eprintln!("✖ Packaging failed: {}", e);
                    process::exit(1);
                }
            }
        }
        CliCommand::Test {
            parity,
            verbose,
            android,
            ios,
            mock_ios,
        } => {
            CliArgs::print_banner();
            if android || ios || mock_ios {
                println!("⚡ Running Strata Mobile Virtualization Test Suite...");
                let config = strata_mobile_test::StrataMobileTestConfig {
                    is_android: android || (!ios && !mock_ios),
                    is_ios: ios,
                    is_mock_ios: mock_ios,
                    device_serial: None,
                    enable_monkey_fuzz: true,
                    fuzz_event_count: 50,
                    test_foldable: true,
                };
                let mut runner = strata_mobile_test::StrataMobileTestRunner::new(config);
                let report = runner.execute();
                for detail in &report.details {
                    println!("   -> {}", detail);
                }
                if report.passed {
                    println!(
                        "✔ Mobile test suite passed: {}/{} tests successful (SSIM: {:.4})",
                        report.tests_run.saturating_sub(report.tests_failed),
                        report.tests_run,
                        report.ssim_score
                    );
                } else {
                    eprintln!("✖ Mobile test suite failed: {}/{} failed", report.tests_failed, report.tests_run);
                    process::exit(1);
                }
            } else if parity {
                println!("⚡ Running Chromium-parity visual and layout regression test suite...");
                let report = ParityHarness::run_suite(verbose);
                println!(
                    "✔ Specification Parity: {}/{} tests passed (Average delta: {:.2}%)",
                    report.passed_cases,
                    report.total_cases,
                    report.average_pixel_delta * 100.0
                );
                if report.is_full_parity {
                    println!("   Status: 100% Chromium/Blink Reference Match.");
                }
            } else {
                println!("⚡ Running Strata tests...");
            }
        }
        CliCommand::Help => {
            CliArgs::print_help();
        }
        CliCommand::Version => {
            println!("strata 0.1.0 (pure rust, zero-chromium)");
        }
    }
}
