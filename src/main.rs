use anyhow::Result;
use notify::{RecursiveMode, Watcher};
use std::{collections::HashSet, iter::FromIterator, process::Command};
use structopt::StructOpt;
use wasm_run::prelude::*;

#[wasm_run::main(
    pre_build = pre_build,
    watch = watch,
)]
#[derive(StructOpt, Debug)]
enum Cli {}

fn pre_build(_args: &DefaultBuildArgs, profile: BuildProfile, command: &mut Command) -> Result<()> {
    match profile {
        BuildProfile::Profiling | BuildProfile::Release => {
            command.args(&["--features", "wee_alloc"]);
        }
        BuildProfile::Dev => {
            command.args(&["--features", "console_error_panic_hook"]);
        }
    }

    Ok(())
}

fn watch(args: &DefaultServeArgs, watcher: &mut RecommendedWatcher) -> Result<(), anyhow::Error> {
    let metadata = args.build_args().metadata();

    let _ = watcher.watch("static", RecursiveMode::Recursive);

    let members: HashSet<_> = HashSet::from_iter(&metadata.workspace_members);

    for package in metadata.packages.iter().filter(|x| members.contains(&x.id)) {
        let _ = watcher.watch(&package.manifest_path, RecursiveMode::Recursive);
        let _ = watcher.watch(
            package.manifest_path.parent().unwrap().join("src"),
            RecursiveMode::Recursive,
        );
    }

    Ok(())
}
