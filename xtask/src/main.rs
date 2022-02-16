use std::process;
use xtask_wasm::{anyhow::Result, cargo_metadata, clap};

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

            download_blueprint_css("yewprint-playground")?;

            if arg.release {
                let result = arg
                    .static_dir_path("yewprint-playground/static")
                    .app_name("yewprint-playground")
                    .run("yewprint-playground")?;

                xtask_wasm::WasmOpt::level(1)
                    .shrink(2)
                    .optimize(result.wasm)?;
            } else {
                arg.static_dir_path("yewprint-playground/static")
                    .app_name("yewprint-playground")
                    .run("yewprint-playground")?;
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

fn download_blueprint_css(package: impl AsRef<std::path::Path>) -> Result<()> {
    let css_path = package.as_ref().join("static").join("blueprint.css");

    if !css_path.exists() {
        log::info!("downloading blueprint's css in {}", &css_path.display());
        yewprint_css::download_css(&css_path)?;
    }

    Ok(())
}
