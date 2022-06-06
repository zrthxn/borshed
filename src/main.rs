#![feature(rustc_private)]

extern crate rustc_ast_pretty;
extern crate rustc_driver;
extern crate rustc_error_codes;
extern crate rustc_errors;
extern crate rustc_interface;
extern crate rustc_hash;
extern crate rustc_session;
extern crate rustc_span;

use std::{env, path, process, str, collections};
use rustc_ast_pretty::pprust::item_to_string;
use rustc_session::{config, utils};
use rustc_errors::registry;

fn main() -> rustc_interface::interface::Result<()> {
    let filename = env::args().nth(1).expect("No file name given.");
    
    let out = process::Command::new("rustc")
        .arg("--print=sysroot")
        .current_dir(".")
        .output()
        .unwrap();
    let sysroot = str::from_utf8(&out.stdout).unwrap().trim();
    let mut borsch_derive_paths_btree = collections::BTreeSet::new();
    borsch_derive_paths_btree.insert(utils::CanonicalizedPath::new(path::Path::new("/Users/justin/Developer/mlh-fellowship/borshed/vendor/libborsh_derive.dylib")));
    let mut borsch_paths_btree = collections::BTreeSet::new();
    borsch_paths_btree.insert(utils::CanonicalizedPath::new(path::Path::new("/Users/justin/Developer/mlh-fellowship/borshed/vendor/libborsh.rlib")));
    let mut externs_btree = collections::BTreeMap::new();
    externs_btree.insert(
        String::from("borsh_derive"),
        config::ExternEntry {
            location: config::ExternLocation::ExactPaths(borsch_derive_paths_btree),
            is_private_dep: false,
            add_prelude: true,
            nounused_dep: false,
        }
    );
    externs_btree.insert(
        String::from("borsh"),
        config::ExternEntry {
            location: config::ExternLocation::ExactPaths(borsch_paths_btree),
            is_private_dep: false,
            add_prelude: true,
            nounused_dep: false,
        }
    );
    let config = rustc_interface::Config {
        opts: config::Options {
            maybe_sysroot: Some(path::PathBuf::from(sysroot)),
            externs: config::Externs::new(externs_btree),
            ..config::Options::default()
        },
        input: config::Input::File(path::PathBuf::from(filename)),
        diagnostic_output: rustc_session::DiagnosticOutput::Default,
        crate_cfg: rustc_hash::FxHashSet::default(),
        crate_check_cfg: config::CheckCfg::default(),
        input_path: None,
        output_dir: None,
        output_file: None,
        file_loader: None,
        lint_caps: rustc_hash::FxHashMap::default(),
        parse_sess_created: None,
        register_lints: None,
        override_queries: None,
        make_codegen_backend: None,
        registry: registry::Registry::new(&rustc_error_codes::DIAGNOSTICS),
    };
    rustc_interface::run_compiler(config, |compiler| {
        println!("Compiling…\n");
        compiler.enter(|queries| {
            let ast_krate = queries.parse()?.take();
            for item in ast_krate.items {
                println!("{}", item_to_string(&item));
            }
            queries.expansion()?;
            queries.prepare_outputs()?;
            queries.global_ctxt()?;
            queries.ongoing_codegen()?;
            queries.linker()
        })
        .and_then(|linker| {
            linker.link()?;
            Ok(())
        });
    });
    Ok(())
}

macro_rules! get_struct {
    ($( $x: expr ), *) => {
        
    };
}
