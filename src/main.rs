mod buffer;
mod io;
mod npm;
mod pip;
mod proc;
mod registry;

use std::path::Path;

use crate::{
    buffer::{Buffer, Lang},
    io::read_file,
    proc::{Cmd, run_all},
};

fn main() -> Result<(), String> {
    let src = read_file(Path::new("package.json"))?;
    println!("{src}");

    let syntax = Buffer::new(src, Lang::Json)?;

    let node = syntax.get_pairs("devDependencies");

    if let Some(deps) = node {
        for dep in deps {
            println!("{} - {}", dep.key, dep.value);
        }
    }

    let versions = run_all(vec![Cmd::new("npm", ["view", "prettier", "version"])])?;
    println!("versions: {}", versions.concat());

    // let outs = fan_out()

    // let mut parser = Parser::new();
    // parser
    //     .set_language(&tree_sitter_json::LANGUAGE.into())
    //     .unwrap();

    // let tree = parser.parse(&src, None).unwrap();

    // let root = tree.root_node().named_child(0).unwrap();

    // let name = get_node(root, &src, "name").unwrap();
    // let name_pos = name.start_position();
    // let name_val = name.utf8_text(src.as_bytes()).unwrap();
    // println!("{name_val}:{name_pos}");

    // let dev_deps = get_node(root, &src, "devDependencies").unwrap();
    // let prettier = get_node(dev_deps, &src, "prettier").unwrap();
    // let prettier_pos = prettier.start_position();
    // let prettier_val = prettier.utf8_text(src.as_bytes()).unwrap();
    // println!("{prettier_val}:{prettier_pos}");

    // let updated = format!(
    //     "{}{}{}",
    //     &src[..prettier.start_byte()],
    //     "\"new version!\"",
    //     &src[prettier.end_byte()..]
    // );

    // write_file(Path::new("package-update.json"), &updated)?;

    Ok(())
}
