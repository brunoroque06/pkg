mod buffer;
mod io;
mod npm;
mod pip;
mod proc;
mod registry;

use std::path::Path;

use crate::{
    buffer::Buffer,
    io::read_file,
    npm::Npm,
    pip::Pip,
    proc::{Cmd, run_all},
    registry::Registry,
};

fn main() -> Result<(), String> {
    let file = Path::new("package.json");

    let regs: [&dyn Registry; 2] = [&Npm, &Pip];

    let reg = regs
        .iter()
        .find(|r| r.can_handle(file))
        .ok_or(format!("cannot handle file {}", file.to_string_lossy()))?;

    let src = read_file(file)?;
    let syntax = Buffer::new(src, reg.manifest_type())?;

    let deps = syntax
        .get_pairs("devDependencies")
        .ok_or("no dependencies found")?;

    let cmds = deps.iter().map(|d| reg.latest_version(d.key)).collect();

    let outs = run_all(cmds)?;

    let latest = deps
        .iter()
        .zip(&outs)
        .map(|(d, o)| (d.key, reg.latest_version_parse(o)))
        .collect::<Vec<_>>();

    for (d, l) in latest {
        println!("{d}: {l}");
    }

    let versions = run_all(vec![Cmd::new("npm", ["view", "prettier", "version"])])?;
    println!("versions: {}", versions.concat());

    Ok(())
}
