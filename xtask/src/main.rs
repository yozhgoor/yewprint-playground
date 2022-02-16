use std::process;
use std::fs;
use std::path::Path;
use xtask_wasm::{anyhow::Result, clap, metadata};

#[derive(clap::Parser)]
enum Opt {
    Dist(xtask_wasm::Dist),
    Watch(xtask_wasm::Watch),
    Start(xtask_wasm::DevServer),
}

fn main() -> Result<()> {
    let opt: Opt = clap::Parser::parse();

    env_logger::builder()
        .filter(Some("xtask"), log::LevelFilter::Info)
        .init();

    match opt {
        Opt::Dist(arg) => {
            log::info!("Generating package...");

            if arg.release {
                let result = arg
                    .static_dir_path("yewprint-playground/static")
                    .app_name("yewprint-playground")
                    .run("yewprint-playground")?;

                download_blueprint_css(result.dist_dir)?;

                xtask_wasm::WasmOpt::level(1)
                    .shrink(2)
                    .optimize(result.wasm)?;
            } else {
                let result = arg.static_dir_path("yewprint-playground/static")
                    .app_name("yewprint-playground")
                    .run("yewprint-playground")?;

                download_blueprint_css(result.dist_dir)?;
            }
        }
        Opt::Watch(arg) => {
            let mut command = process::Command::new("cargo");
            command.args(["xtask", "dist"]);

            arg.run(command)?;
        }
        Opt::Start(arg) => {
            arg.arg("dist").start(xtask_wasm::default_dist_dir(false))?;
        }
    }

    Ok(())
}

fn download_blueprint_css(dest: impl AsRef<Path>) -> Result<()> {
    let css_path = metadata().target_directory.join("blueprint.css");

    if !css_path.exists() {
        yewprint_css::download_css(&css_path)?;
    }

    if !dest.as_ref().join("blueprint.css").exists() {
        fs::copy(css_path, dest.as_ref().join("blueprint.css"))?;
    }

    Ok(())
}
