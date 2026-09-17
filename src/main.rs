mod args;
mod buffer;
mod cargo;
mod io;
mod npm;
mod pip;
mod proc;
mod registry;

use std::path::Path;

use crate::{
    args::parse_args,
    buffer::Buffer,
    cargo::Cargo,
    io::{read_file, write_file},
    npm::Npm,
    pip::Pip,
    proc::run_all,
    registry::Registry,
};

fn main() -> Result<(), String> {
    let args = parse_args();

    let file = Path::new(&args.file);

    let regs: [&dyn Registry; 3] = [&Cargo, &Npm, &Pip];

    let reg = regs
        .iter()
        .find(|r| r.supports(file))
        .ok_or(format!("cannot handle {}", file.to_string_lossy()))?;

    let src = read_file(file)?;
    let buf = Buffer::new(src, reg.manifest())?;

    let pairs = buf.query_pairs(&reg.deps_query())?;

    let deps = pairs
        .into_iter()
        .map(|p| reg.dep_parse(p))
        .collect::<Result<Vec<_>, String>>()?;

    let cmds = deps.iter().map(|d| reg.cmd_version(d.key)).collect();

    let outs = run_all(cmds, args.concurrency)?;

    let latest = deps
        .into_iter()
        .zip(&outs)
        .map(|(d, o)| {
            let value = reg.cmd_parse(o)?;
            Ok(d.with_value(value))
        })
        .collect::<Result<Vec<_>, String>>()?;

    let edits = latest
        .into_iter()
        .filter(|e| e.pair.value != e.value)
        .collect::<Vec<_>>();

    if edits.is_empty() {
        println!("No packages to upgrade")
    } else {
        println!("Upgrading:");
        for e in edits.iter() {
            println!("\t{} {} -> {}", e.pair.key, e.pair.value, e.value);
        }
        let replaced = buf.replace(edits);
        write_file(file, &replaced)?;
    }

    Ok(())
}
