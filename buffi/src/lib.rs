// Copyright (C) 2024 by GiGa infosystems

//! This code is used to generate the c++ side API bindings for a Rust API
//! based on the rustdoc json output
//!
//! It generates the following files:
//!
//! * functions.hpp, containing the c++ side function definitions
//! * types.hpp, containing types for any type used in the generated function signatures
//! * serde.hpp, bincode.hpp, binary.hpp, containing helper code used for the (de)serialization implementation
//!
#![doc=include_str!("../../README.md")]

use serde::{Deserialize, Serialize};
use serde_generate::SourceInstaller;
use std::borrow::Cow;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::ffi::OsStr;
use std::fmt::Write as _;
use std::fs;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write as _;
use std::path::PathBuf;
use std::path::{Component, Path};
use std::process::{Output, Stdio};

const FUNCTION_PREFIX: &str = "buffi";

#[derive(Debug, serde::Deserialize)]
struct WorkspaceMetadata {
    target_directory: String,
}

/// A Config object that provides information for the generation of C/C++ code
#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    /// The namespace that should be used in the C++ code (required)
    pub namespace: String,
    /// The name of the API library that is built (important for Rustdoc, required)
    pub api_lib_name: String,
    /// The name of your API crate (important for Rustdoc, required)
    pub parent_crate: String,
    /// All crates that include the types you use in your API, needs to at least include your API crate (required)
    pub rustdoc_crates: Vec<String>,
    /// In case the file names of the generated C/C++ should have a prefix, put this here
    pub file_prefix: Option<String>,
    /// Copyright header to be included in every C/C++ file
    pub copyright_header: Option<String>,
    /// Generated-by header to be included in every C/C++ file
    pub generated_by_header: Option<String>,
    /// In case you need to set any feature flags for build process of Rustdoc, add them here
    pub crate_feature_flags: Option<Vec<String>>,
    /// Add some additional rustdoc flags here, can be useful for debugging
    pub rustdoc_flags: Option<Vec<String>>,
}

impl Config {
    /// Create a new config object by only setting required fields
    pub fn new(
        namespace: String,
        api_lib_name: String,
        parent_crate: String,
        rustdoc_crates: Vec<String>,
    ) -> Self {
        Self {
            namespace,
            api_lib_name,
            parent_crate,
            rustdoc_crates,
            file_prefix: None,
            copyright_header: None,
            generated_by_header: None,
            crate_feature_flags: None,
            rustdoc_flags: None,
        }
    }

    /// Add some additional flags that should be passed when creating the rustdocs
    pub fn extend_rustdoc_flags(&mut self, flags: Vec<String>) {
        if let Some(rustdoc_flags) = self.rustdoc_flags.as_mut() {
            rustdoc_flags.extend(flags);
        } else {
            self.rustdoc_flags = Some(flags);
        }
    }
}

struct ItemResolver {
    base_path: String,
    doc_types: rustdoc_types::Crate,
    other_crates: RefCell<HashMap<String, rustdoc_types::Crate>>,
}

impl ItemResolver {
    fn new(json_path: String, api_lib_name: &str) -> Self {
        let content = std::fs::read_to_string(json_path.clone() + api_lib_name + ".json").unwrap();
        let doc_types = serde_json::from_str(&content).unwrap();
        Self {
            base_path: json_path,
            doc_types,
            other_crates: RefCell::new(HashMap::new()),
        }
    }

    // this function expects a fully qualified path.
    fn resolve_by_path(
        &self,
        path: &str,
        parent_crate: &str,
        requested_item: rustdoc_types::ItemKind,
    ) -> rustdoc_types::Path {
        let mut parts = path.split("::").collect::<Vec<_>>();
        if parts[0] == "crate" {
            parts[0] = parent_crate;
        }
        let id = {
            let mut other_crates = self.other_crates.borrow_mut();
            let map = if parts[0] == parent_crate {
                &self.doc_types
            } else {
                other_crates.entry(parts[0].to_owned()).or_insert_with(|| {
                    self.load_extern_crate_doc(parts[0], &format!("(needed for {path:?})"))
                })
            };
            let (id, summary) = map
                .paths
                .iter()
                .find(|(_, i)| i.path == parts)
                .expect("It's there");
            if summary.kind == requested_item {
                id.clone()
            } else {
                panic!(
                    "Incompatible type: Expected {requested_item:?}, Got {:?}",
                    summary.kind
                );
            }
        };
        rustdoc_types::Path {
            name: parts[parts.len() - 1].to_owned(),
            id,
            args: None,
        }
    }

    fn resolve_index(
        &self,
        t: Option<&rustdoc_types::Path>,
        id: &rustdoc_types::Id,
        parent_crate: &str,
    ) -> rustdoc_types::Item {
        let mut other_crates = self.other_crates.borrow_mut();

        let candidates = std::iter::once(&self.doc_types)
            .chain(other_crates.values())
            .filter_map(|c| c.index.get(id))
            .collect::<Vec<_>>();
        match &candidates as &[&rustdoc_types::Item] {
            [i] => return rustdoc_types::Item::clone(i),
            [] => {
                // handled by the code below
            }
            items => {
                // we might get several candidates. In that case check that:
                //
                // * We resolve against the local crate (indicated by '0' in the beginning)
                // * There is a candidate coming from this crate (indicated by the parent_crate)
                //   argument
                let matches_parent_crate = items
                    .iter()
                    .find(|i| extract_crate_from_span(i) == parent_crate);
                match matches_parent_crate {
                    Some(t) if id.0.starts_with('0') => {
                        return rustdoc_types::Item::clone(t);
                    }
                    _ => {
                        panic!("Cannot decide what's the correct candidate")
                    }
                }
            }
        }

        // expect possibly multiple matching entries?
        let mut matched_ids = Vec::with_capacity(1);
        if let Some(item) = self.doc_types.paths.get(id) {
            matched_ids.push(item.clone());
        }
        for c in other_crates.values() {
            if let Some(s) = c.paths.get(id) {
                matched_ids.push(s.clone());
            }
        }

        // use the first matching entry
        for crate_id in matched_ids {
            // we need to resolve other crates by name
            // not by crate-id as these id's are not stable across
            // different crates
            let crate_name = crate_id.path.first().unwrap().clone();
            let other_index = other_crates.entry(crate_name.clone()).or_insert_with(|| {
                self.load_extern_crate_doc(&crate_name, &format!("(needed for {t:?})"))
            });
            if let Some(item) = other_index.index.get(id) {
                return item.clone();
            } else {
                // This is just guessing the right item at this point
                // This likely needs improvements
                // TODO: Fix this as soon as the generated rustdoc contains the right information
                // (Check on compiler updates)
                let name = crate_id.path.last().unwrap();
                let item = other_index.index.values().find(|i| {
                    i.name.as_ref() == Some(name)
                        && matches!(
                            (&i.inner, &crate_id.kind),
                            (
                                rustdoc_types::ItemEnum::Struct(_),
                                rustdoc_types::ItemKind::Struct
                            ) | (
                                rustdoc_types::ItemEnum::Enum(_),
                                rustdoc_types::ItemKind::Enum
                            )
                        )
                });
                if let Some(item) = item {
                    return item.clone();
                }
            }
        }
        panic!(
            "Unknown id: {:?}, crate: {:?} (full type:{:?})",
            id, parent_crate, t
        );
    }

    fn load_extern_crate_doc(
        &self,
        crate_name: &str,
        additional_message: &str,
    ) -> rustdoc_types::Crate {
        let content = std::fs::read_to_string(self.base_path.clone() + crate_name + ".json")
            .unwrap_or_else(|_| {
                panic!(
                    "Failed to find docs for `{}` {}",
                    &crate_name, additional_message
                );
            });
        serde_json::from_str(&content).unwrap()
    }
}

enum TypeCache {
    NeedToPopulate,
    Cached(
        Vec<(
            serde_reflection::Format,
            Option<serde_reflection::ContainerFormat>,
        )>,
    ),
}

pub fn generate_bindings(out_dir: &Path, config: Config) {
    if !out_dir.exists() {
        panic!("Out directory does not exist");
    }

    let (target_directory, handle) = generate_docs(
        &config.api_lib_name,
        &config.rustdoc_crates,
        config.crate_feature_flags.as_ref().unwrap_or(&Vec::new()),
        config.rustdoc_flags.as_ref().unwrap_or(&Vec::new()),
    );

    let mut failed = false;
    if let Ok(handle) = handle {
        if handle.status.success() {
            let resolver = ItemResolver::new(target_directory + "/doc/", &config.api_lib_name);
            let mut type_map = HashMap::new();
            let out_dir = out_dir.display().to_string();
            generate_type_definitions(&resolver, &out_dir, &mut type_map, &config);
            generate_function_definitions(
                resolver,
                &out_dir,
                &mut type_map,
                FUNCTION_PREFIX,
                &config,
            );
        } else {
            failed = true;
        }
    } else {
        failed = true;
    }

    if !failed {
        println!("Finished, wrote bindings to `{}`", out_dir.display());
    }

    if failed {
        eprintln!("Failed to generate bindings");
        std::process::exit(1);
    }
}

pub fn generate_docs(
    api_lib_name: &String,
    rustdoc_crates: &[String],
    crate_flags: &[String],
    rustdoc_flags: &[String],
) -> (String, Result<Output, std::io::Error>) {
    print!("Gather workspace metadata:");
    std::io::stdout().flush().expect("Flushing does not fail");
    let metadata = std::process::Command::new("cargo")
        .arg("metadata")
        .arg("--format-version=1")
        .stderr(Stdio::inherit())
        .output()
        .expect("Failed to get workspace metadata");
    println!(" OK");

    let WorkspaceMetadata { target_directory } = serde_json::from_slice(&metadata.stdout).unwrap();
    // remove all old json doc files (if any exist), important in case the configuration has changed
    let doc_directory = target_directory.to_owned() + "/doc";
    if matches!(fs::exists(&doc_directory), Ok(true)) {
        for entry in fs::read_dir(doc_directory).unwrap() {
            let file_path = entry.unwrap().path();
            if file_path.extension().and_then(|s| s.to_str()) == Some("json") {
                fs::remove_file(file_path).unwrap();
            }
        }
    }

    if rustdoc_crates.is_empty() {
        eprintln!("Need at least one input crate to create bindings!");
        std::process::exit(1);
    }

    // only build documentation for our own crates for now
    let mut args = vec!["--no-deps"];
    let crate_args: Vec<_> = rustdoc_crates
        .iter()
        .flat_map(|crate_name| vec!["-p", crate_name])
        .collect();
    let crate_flag_args: Vec<_> = crate_flags
        .iter()
        .flat_map(|crate_and_flag| vec!["-F", crate_and_flag])
        .collect();
    args.extend(crate_args);
    args.extend(crate_flag_args);
    args.extend(rustdoc_flags.iter().map(|s| s as &str));

    let bootstrap_crates = vec![api_lib_name].into_iter().chain(rustdoc_crates).fold(
        String::new(),
        |cumulated, crate_name| {
            let crate_name = crate_name.replace("-", "_");
            cumulated + &format!(",{crate_name}")
        },
    );
    // this works because `rustdoc_crates` has at least one entry
    let bootstrap_crates = &bootstrap_crates[1..bootstrap_crates.len()];

    println!("Compile rustdocs:");
    let mut rustdoc_command = std::process::Command::new("cargo");

    rustdoc_command
        .arg("doc")
        .args(args)
        .env("RUSTC_BOOTSTRAP", bootstrap_crates)
        .env("RUSTDOCFLAGS", "-Z unstable-options --output-format json ")
        .env("CARGO_TARGET_DIR", &target_directory)
        .stderr(Stdio::inherit())
        .stdout(Stdio::inherit());

    let handle = rustdoc_command.output();
    (target_directory, handle)
}

fn generate_function_definitions(
    res: ItemResolver,
    out_dir: &str,
    type_map: &mut HashMap<rustdoc_types::Type, TypeCache>,
    function_prefix: &str,
    config: &Config,
) {
    let namespace = &config.namespace;
    let file_prefix = config.file_prefix.as_ref().unwrap_or(&config.api_lib_name);

    let out_dir = PathBuf::from(out_dir);
    let mut extern_c_functions = res
        .doc_types
        .index
        .values()
        .filter_map(|item| {
            if let rustdoc_types::ItemEnum::Function(ref func) = item.inner {
                if matches!(func.header.abi, rustdoc_types::Abi::C { .. }) {
                    let s = generate_extern_c_function_def(item.name.as_deref().unwrap(), func);
                    Some(s)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    // ensure that we always emit these functions in the same order
    extern_c_functions.sort();

    let mut free_standing_functions = res
        .doc_types
        .index
        .values()
        .filter(is_free_standing_impl)
        .collect::<Vec<_>>();

    free_standing_functions.sort_by_key(|f| f.name.as_ref());

    let mut relevant_impls = res
        .doc_types
        .index
        .values()
        .filter(is_relevant_impl)
        .flat_map(|item| {
            if let rustdoc_types::ItemEnum::Impl(ref impl_) = item.inner {
                impl_
                    .items
                    .iter()
                    .map(|id| res.resolve_index(None, id, &config.parent_crate))
                    .filter(|item| matches!(item.inner, rustdoc_types::ItemEnum::Function(_)))
                    .map(move |i| (&impl_.for_, i))
            } else {
                unreachable!()
            }
        })
        .fold(HashMap::<_, Vec<_>>::new(), |mut acc, (t, i)| {
            acc.entry(t).or_default().push(i);
            acc
        })
        .into_iter()
        .map(|(n, mut items)| {
            items.sort_by_key(|i| i.name.clone());
            (n, items)
        })
        .collect::<Vec<_>>();

    // ensure that we always order the type definitions in the same way
    relevant_impls.sort_by_key(|(t, _)| {
        if let rustdoc_types::Type::ResolvedPath(ref p) = t {
            get_name_without_path(&p.name)
        } else {
            unreachable!()
        }
    });
    let extern_c_header = out_dir.join(format!("{file_prefix}_api_functions.hpp"));
    let mut extern_c_header = BufWriter::new(File::create(extern_c_header).unwrap());
    write_function_header(&mut extern_c_header, config);
    writeln!(extern_c_header, "#include <cstdint>").unwrap();
    writeln!(extern_c_header).unwrap();
    for (t, _) in relevant_impls.iter() {
        if let rustdoc_types::Type::ResolvedPath(p) = t {
            let name = get_name_without_path(&p.name);
            writeln!(extern_c_header, "struct {};\n", name).unwrap();
        } else {
            unreachable!()
        }
    }
    for function in extern_c_functions {
        writeln!(extern_c_header, "{function}").unwrap();
    }
    extern_c_header.flush().unwrap();

    for (t, impls) in relevant_impls {
        if let rustdoc_types::Type::ResolvedPath(p) = t {
            let name = get_name_without_path(&p.name);
            let type_header =
                out_dir.join(format!("{file_prefix}_{}.hpp", name.to_ascii_lowercase()));
            let mut writer = BufWriter::new(File::create(type_header).unwrap());
            write_function_header(&mut writer, config);
            writeln!(writer, "#include \"{file_prefix}_api_functions.hpp\"\n").unwrap();
            writeln!(writer, "#include \"{namespace}.hpp\"\n").unwrap();

            writeln!(writer).unwrap();
            writeln!(writer, "namespace {namespace} {{").unwrap();
            writeln!(writer).unwrap();
            writeln!(writer, "class {name}Holder {{").unwrap();
            writeln!(writer, "    {name}* inner;").unwrap();
            writeln!(writer, "public:").unwrap();
            writeln!(writer, "    {name}Holder({name}* ptr) {{").unwrap();
            writeln!(writer, "        this->inner = ptr;").unwrap();
            writeln!(writer, "    }}\n").unwrap();
            for impl_ in impls {
                if let rustdoc_types::ItemEnum::Function(ref m) = impl_.inner {
                    generate_function_def(
                        m,
                        &res,
                        &impl_,
                        &mut writer,
                        type_map,
                        function_prefix,
                        config,
                        Some(t),
                    );
                }
            }
            writeln!(writer, "}};\n").unwrap();
            writeln!(writer, "}}  // end of namespace {namespace}").unwrap();
            writer.flush().unwrap();
        }
    }

    let free_standing_function_header =
        out_dir.join(format!("{file_prefix}_free_standing_functions.hpp"));
    let mut free_standing_function_header =
        BufWriter::new(File::create(free_standing_function_header).unwrap());

    write_function_header(&mut free_standing_function_header, config);
    writeln!(
        free_standing_function_header,
        "#include \"{file_prefix}_api_functions.hpp\"\n"
    )
    .unwrap();
    writeln!(
        free_standing_function_header,
        "#include \"{namespace}.hpp\"\n"
    )
    .unwrap();

    writeln!(free_standing_function_header).unwrap();
    writeln!(free_standing_function_header, "namespace {namespace} {{").unwrap();
    writeln!(free_standing_function_header).unwrap();

    for item in &free_standing_functions {
        if let rustdoc_types::ItemEnum::Function(ref f) = item.inner {
            generate_function_def(
                f,
                &res,
                item,
                &mut free_standing_function_header,
                type_map,
                function_prefix,
                config,
                None,
            );
            writeln!(free_standing_function_header).unwrap();
        }
    }

    writeln!(
        free_standing_function_header,
        "}}  // end of namespace {namespace}"
    )
    .unwrap();
    free_standing_function_header.flush().unwrap();
}

fn write_function_header(out_functions: &mut BufWriter<File>, config: &Config) {
    if let Some(copyright_header) = &config.copyright_header {
        writeln!(out_functions, "// {copyright_header}").unwrap();
    }
    if let Some(generated_by) = &config.generated_by_header {
        writeln!(out_functions, "// {generated_by}").unwrap();
    }
    if config.copyright_header.is_some() || config.generated_by_header.is_some() {
        writeln!(out_functions).unwrap();
    }
    writeln!(out_functions, "#pragma once\n").unwrap();
    writeln!(out_functions, "#include <cstddef>").unwrap();
    writeln!(out_functions, "#include <limits>").unwrap();
}

#[allow(clippy::too_many_arguments)]
fn generate_function_def(
    m: &rustdoc_types::Function,
    res: &ItemResolver,
    item: &rustdoc_types::Item,
    out_functions: &mut BufWriter<File>,
    type_map: &mut HashMap<rustdoc_types::Type, TypeCache>,
    prefix: &str,
    config: &Config,
    impl_type: Option<&rustdoc_types::Type>,
) {
    let output_type = if let Some(ref tpe) = m.decl.output {
        let tpe = to_serde_reflect_type(
            tpe,
            res,
            &mut None,
            Vec::new(),
            &config.parent_crate,
            &config.namespace,
            type_map,
        );
        to_cpp_type_name(&tpe.last().unwrap().0)
    } else {
        unimplemented!()
    };
    let inputs = m
        .decl
        .inputs
        .iter()
        .map(|(name, tpe)| {
            if name == "self" {
                let impl_type_path = impl_type
                    .map(|tpe| {
                        let rustdoc_types::Type::ResolvedPath(path) = tpe else {
                            panic!("Impl type must be a resolved path");
                        };
                        path
                    })
                    .expect("we have an impl type for impl functions");
                return (name, get_name_without_path(&impl_type_path.name).to_owned());
            }
            let reflect_type = to_serde_reflect_type(
                tpe,
                res,
                &mut None,
                Vec::new(),
                &config.parent_crate,
                &config.namespace,
                type_map,
            );
            let type_string = reflect_type
                .last()
                .map(|(f, _)| to_cpp_type_name(f))
                .unwrap_or_else(|| panic!("Unknown type: {:?}", tpe));
            (name, type_string)
        })
        .collect::<Vec<_>>();
    let return_output_type = match m.decl.output {
        Some(rustdoc_types::Type::ResolvedPath(ref p))
            if get_name_without_path(&p.name) == "Result" =>
        {
            if let Some(rustdoc_types::GenericArgs::AngleBracketed { args, .. }) = p.args.as_deref()
            {
                if let rustdoc_types::GenericArg::Type(tpe) = &args[0] {
                    let tpe = to_serde_reflect_type(
                        tpe,
                        res,
                        &mut None,
                        Vec::new(),
                        &config.parent_crate,
                        &config.namespace,
                        type_map,
                    );
                    Cow::Owned(to_cpp_type_name(&tpe.last().unwrap().0))
                } else {
                    unreachable!()
                }
            } else {
                unreachable!()
            }
        }
        Some(rustdoc_types::Type::ResolvedPath(ref p))
            if get_name_without_path(&p.name) == "String" =>
        {
            Cow::Owned(to_cpp_type_name(&serde_reflection::Format::Str))
        }
        _ => Cow::Borrowed(&output_type as &str),
    };
    if let Some(ref docs) = item.docs {
        for line in docs.lines() {
            writeln!(out_functions, "    // {line}").unwrap()
        }
    }
    write!(
        out_functions,
        "    inline {return_output_type} {}(",
        item.name.as_ref().unwrap()
    )
    .unwrap();
    for (idx, (name, tpe)) in inputs.iter().filter(|(n, _)| *n != "self").enumerate() {
        if idx != 0 {
            write!(out_functions, ", ").unwrap();
        }
        write!(out_functions, "const {tpe}& {name}").unwrap();
    }
    writeln!(out_functions, ") {{").unwrap();
    for (name, tpe) in &inputs {
        if *name == "self" {
            continue;
        }
        writeln!(
            out_functions,
            "        auto serializer_{name} = serde::BincodeSerializer();"
        )
        .unwrap();
        writeln!(
            out_functions,
            "        serde::Serializable<{tpe}>::serialize({name}, serializer_{name});"
        )
        .unwrap();
        writeln!(out_functions, "        std::vector<uint8_t> {name}_serialized = std::move(serializer_{name}).bytes();").unwrap();
    }
    writeln!(out_functions, "        uint8_t* out_ptr = nullptr;").unwrap();
    writeln!(out_functions).unwrap();
    write!(
        out_functions,
        "        size_t res_size = {}_{}(",
        prefix,
        item.name.as_deref().unwrap(),
    )
    .unwrap();
    for (name, _) in inputs.iter() {
        if *name == "self" {
            write!(out_functions, "this->inner, ").unwrap();
        } else {
            write!(
                out_functions,
                "{name}_serialized.data(), {name}_serialized.size(), "
            )
            .unwrap();
        }
    }
    writeln!(out_functions, "&out_ptr);").unwrap();
    writeln!(out_functions).unwrap();
    writeln!(
        out_functions,
        "        std::vector<uint8_t> serialized_result(out_ptr, out_ptr + res_size);"
    )
    .unwrap();
    writeln!(
        out_functions,
        "        {output_type} out = {output_type}::bincodeDeserialize(serialized_result);"
    )
    .unwrap();
    writeln!(
        out_functions,
        "        {}_free_byte_buffer(out_ptr, res_size);",
        prefix
    )
    .unwrap();
    writeln!(out_functions).unwrap();
    if matches!(m.decl.output, Some(rustdoc_types::Type::ResolvedPath(ref p)) if get_name_without_path(&p.name) == "Result")
    {
        writeln!(
            out_functions,
            "        if (out.value.index() == 0) {{ // Ok"
        )
        .unwrap();
        if return_output_type == "void" {
            writeln!(out_functions, "            return;").unwrap();
        } else {
            writeln!(
                out_functions,
                "            auto ok = std::get<0>(out.value);"
            )
            .unwrap();
            writeln!(out_functions, "            return std::get<0>(ok.value);").unwrap();
        }
        writeln!(out_functions, "        }} else {{ // Err").unwrap();
        writeln!(
            out_functions,
            "            auto err = std::get<1>(out.value);"
        )
        .unwrap();
        writeln!(
            out_functions,
            "            auto error = std::get<0>(err.value);"
        )
        .unwrap();
        writeln!(out_functions, "            throw error;").unwrap();
        writeln!(out_functions, "        }}").unwrap();
    } else {
        writeln!(out_functions, "        return out;").unwrap();
    }
    writeln!(out_functions, "    }}\n").unwrap();
}

fn generate_type_definitions(
    res: &ItemResolver,
    out_types: &str,
    type_map: &mut HashMap<rustdoc_types::Type, TypeCache>,
    config: &Config,
) {
    let comments = serde_generate::DocComments::new();
    let mut comments = Some(comments);
    let mut types_for_impls = res
        .doc_types
        .index
        .values()
        .filter(|i| is_relevant_impl(i) || is_free_standing_impl(i))
        .flat_map(|item| {
            if let rustdoc_types::ItemEnum::Impl(ref impl_) = item.inner {
                impl_
                    .items
                    .iter()
                    .map(|id| res.resolve_index(None, id, &config.parent_crate))
                    .filter(|item| matches!(item.inner, rustdoc_types::ItemEnum::Function(_)))
                    .collect()
            } else if let rustdoc_types::ItemEnum::Function(ref _f) = item.inner {
                vec![item.clone()]
            } else {
                unreachable!()
            }
        })
        .flat_map(|m| {
            if let rustdoc_types::ItemEnum::Function(ref m) = m.inner {
                m.decl
                    .inputs
                    .iter()
                    .map(|(_, t)| t.clone())
                    .chain(
                        m.decl
                            .output
                            .as_ref()
                            .map(|e| vec![e.clone()])
                            .unwrap_or_default(),
                    )
                    .collect::<Vec<_>>()
            } else {
                unreachable!()
            }
        })
        .collect::<Vec<_>>();
    types_for_impls.dedup();
    let registry = types_for_impls
        .into_iter()
        .map(|t| {
            to_serde_reflect_type(
                &t,
                res,
                &mut comments,
                Vec::new(),
                &config.parent_crate,
                &config.namespace,
                type_map,
            )
        })
        .flat_map(|types| {
            types.into_iter().filter_map(|(format, container)| {
                let container = container?;
                if let serde_reflection::Format::TypeName(n) = format {
                    Some((n, container))
                } else {
                    None
                }
            })
        })
        .collect::<serde_reflection::Registry>();

    let config = serde_generate::CodeGeneratorConfig::new(config.namespace.to_owned())
        .with_comments(comments.unwrap())
        .with_encodings([serde_generate::Encoding::Bincode]);
    let installer = serde_generate::cpp::Installer::new(PathBuf::from(out_types));
    installer.install_module(&config, &registry).unwrap();
    installer.install_serde_runtime().unwrap();
    installer.install_bincode_runtime().unwrap();
}

fn to_cpp_type_name(f: &serde_reflection::Format) -> String {
    match f {
        serde_reflection::Format::Variable(_) => unimplemented!(),
        serde_reflection::Format::TypeName(_) => to_type_name(f).into_owned(),
        serde_reflection::Format::Unit => unimplemented!(),
        serde_reflection::Format::Bool => String::from("bool"),
        serde_reflection::Format::I8 => String::from("int8_t"),
        serde_reflection::Format::I16 => String::from("int16_t"),
        serde_reflection::Format::I32 => String::from("int32_t"),
        serde_reflection::Format::I64 => String::from("int64_t"),
        serde_reflection::Format::I128 => unimplemented!(),
        serde_reflection::Format::U8 => String::from("uint8_t"),
        serde_reflection::Format::U16 => String::from("uint16_t"),
        serde_reflection::Format::U32 => String::from("uint32_t"),
        serde_reflection::Format::U64 => String::from("uint64_t"),
        serde_reflection::Format::U128 => unimplemented!(),
        serde_reflection::Format::F32 => String::from("float"),
        serde_reflection::Format::F64 => String::from("double"),
        serde_reflection::Format::Char => unimplemented!(),
        serde_reflection::Format::Str => String::from("std::string"),
        serde_reflection::Format::Bytes => unimplemented!(),
        serde_reflection::Format::Option(t) => {
            format!("std::optional<{}>", to_cpp_type_name(t))
        }
        serde_reflection::Format::Seq(p) => {
            format!("std::vector<{}>", to_cpp_type_name(p))
        }
        serde_reflection::Format::Map { .. } => unimplemented!(),
        serde_reflection::Format::Tuple(d) if d.is_empty() => String::from("void"),
        serde_reflection::Format::Tuple(_) => unimplemented!(),
        serde_reflection::Format::TupleArray { .. } => unimplemented!(),
    }
}

fn to_type_name(f: &serde_reflection::Format) -> Cow<str> {
    match f {
        serde_reflection::Format::Variable(_) => unimplemented!(),
        serde_reflection::Format::TypeName(n) => Cow::Borrowed(n),
        serde_reflection::Format::Unit => unimplemented!(),
        serde_reflection::Format::Bool => Cow::Borrowed("bool"),
        serde_reflection::Format::I8 => Cow::Borrowed("i8"),
        serde_reflection::Format::I16 => Cow::Borrowed("i16"),
        serde_reflection::Format::I32 => Cow::Borrowed("i32"),
        serde_reflection::Format::I64 => Cow::Borrowed("i64"),
        serde_reflection::Format::I128 => unimplemented!(),
        serde_reflection::Format::U8 => Cow::Borrowed("u8"),
        serde_reflection::Format::U16 => Cow::Borrowed("u16"),
        serde_reflection::Format::U32 => Cow::Borrowed("u32"),
        serde_reflection::Format::U64 => Cow::Borrowed("u64"),
        serde_reflection::Format::U128 => unimplemented!(),
        serde_reflection::Format::F32 => Cow::Borrowed("f32"),
        serde_reflection::Format::F64 => Cow::Borrowed("f64"),
        serde_reflection::Format::Char => unimplemented!(),
        serde_reflection::Format::Str => Cow::Borrowed("String"),
        serde_reflection::Format::Bytes => unimplemented!(),
        serde_reflection::Format::Option(t) => Cow::Owned(format!("Option_{}", to_type_name(t))),
        serde_reflection::Format::Seq(t) => Cow::Owned(format!("Vec_{}", to_type_name(t))),
        serde_reflection::Format::Map { .. } => unimplemented!(),
        serde_reflection::Format::Tuple(d) if d.is_empty() => Cow::Borrowed("void"),
        serde_reflection::Format::Tuple(d) => {
            dbg!(d);
            unimplemented!()
        }
        serde_reflection::Format::TupleArray { .. } => unimplemented!(),
    }
}

fn to_serde_reflect_type(
    t: &rustdoc_types::Type,
    crate_map: &ItemResolver,
    comment_map: &mut Option<serde_generate::DocComments>,
    parent_args: Vec<rustdoc_types::GenericArg>,
    parent_crate: &str,
    namespace: &str,
    type_map: &mut HashMap<rustdoc_types::Type, TypeCache>,
) -> Vec<(
    serde_reflection::Format,
    Option<serde_reflection::ContainerFormat>,
)> {
    use serde_reflection::{ContainerFormat, Format};

    /// This is here for DRY (used by primitives and arrays.)
    fn reflect_primitive(p: &rustdoc_types::Type) -> Vec<(Format, Option<ContainerFormat>)> {
        let rustdoc_types::Type::Primitive(ref p) = p else {
            unreachable!("Primitive!")
        };
        match p.as_ref() {
            "i64" => {
                vec![(Format::I64, None)]
            }
            "i32" => {
                vec![(Format::I32, None)]
            }
            "i16" => {
                vec![(Format::I16, None)]
            }
            "i8" => {
                vec![(Format::I8, None)]
            }
            "bool" => {
                vec![(Format::Bool, None)]
            }
            "f64" => {
                vec![(Format::F64, None)]
            }
            "f32" => {
                vec![(Format::F32, None)]
            }
            "u8" => {
                vec![(Format::U8, None)]
            }
            "u16" => {
                vec![(Format::U16, None)]
            }
            "u32" => {
                vec![(Format::U32, None)]
            }
            "u64" => {
                vec![(Format::U64, None)]
            }
            "usize" if size_of::<usize>() == 8 => {
                // TODO: This, properly.
                vec![(Format::U64, None)]
            }
            "usize" if size_of::<usize>() == 4 => {
                // TODO: This, properly.
                vec![(Format::U32, None)]
            }
            "usize" => {
                panic!("Invalid size of usize.");
            }
            _ => {
                dbg!(p);
                unimplemented!()
            }
        }
    }

    let recursive_type = match type_map.get(t) {
        Some(TypeCache::Cached(t)) => return t.clone(),
        Some(TypeCache::NeedToPopulate) => true,
        None => {
            type_map.insert(t.clone(), TypeCache::NeedToPopulate);
            false
        }
    };

    let r = match t {
        rustdoc_types::Type::ResolvedPath(p) if get_name_without_path(&p.name) == "Result" => {
            let mut out = Vec::new();
            let (ok, error) = if let Some(rustdoc_types::GenericArgs::AngleBracketed {
                args, ..
            }) = p.args.as_deref()
            {
                let ok = &args[0];
                let ok = if let rustdoc_types::GenericArg::Type(tpe) = ok {
                    to_serde_reflect_type(
                        tpe,
                        crate_map,
                        comment_map,
                        Vec::new(),
                        parent_crate,
                        namespace,
                        type_map,
                    )
                } else {
                    unreachable!()
                };
                let err = if let Some((id, _)) =
                    crate_map.doc_types.index.iter().find(|(_, item)| {
                        item.name.as_deref().map(get_name_without_path) == Some("SerializableError")
                    }) {
                    let t = rustdoc_types::Type::ResolvedPath(rustdoc_types::Path {
                        name: "SerializableError".into(),
                        id: id.clone(),
                        args: None,
                    });
                    to_serde_reflect_type(
                        &t,
                        crate_map,
                        comment_map,
                        Vec::new(),
                        parent_crate,
                        namespace,
                        type_map,
                    )
                } else {
                    unreachable!("Could not find docs for `SerializableError`! Maybe the `errors` module or the type itself is still private?")
                };
                (ok, err)
            } else {
                unreachable!()
            };
            let mut result_enum = BTreeMap::new();
            result_enum.insert(
                0,
                serde_reflection::Named {
                    name: "Ok".into(),
                    value: serde_reflection::VariantFormat::Tuple(vec![ok
                        .last()
                        .unwrap()
                        .0
                        .clone()]),
                },
            );
            result_enum.insert(
                1,
                serde_reflection::Named {
                    name: "Err".into(),
                    value: serde_reflection::VariantFormat::Tuple(vec![error
                        .last()
                        .unwrap()
                        .0
                        .clone()]),
                },
            );
            let ok_name = to_type_name(&ok.last().unwrap().0);
            let err_name = to_type_name(&error.last().unwrap().0);
            let name = format!("Result_{ok_name}_{err_name}");
            out.extend(ok);
            out.extend(error);
            out.push((
                Format::TypeName(name),
                Some(ContainerFormat::Enum(result_enum)),
            ));

            out
        }
        rustdoc_types::Type::ResolvedPath(p) if get_name_without_path(&p.name) == "String" => {
            vec![(Format::Str, None)]
        }
        rustdoc_types::Type::ResolvedPath(p) if get_name_without_path(&p.name) == "Vec" => {
            if let Some(rustdoc_types::GenericArgs::AngleBracketed { args, .. }) = p.args.as_deref()
            {
                if let rustdoc_types::GenericArg::Type(tpe) = &args[0] {
                    let mut inner = to_serde_reflect_type(
                        tpe,
                        crate_map,
                        comment_map,
                        Vec::new(),
                        parent_crate,
                        namespace,
                        type_map,
                    );
                    let last = inner.last().unwrap().0.clone();
                    inner.push((Format::Seq(Box::new(last)), None));
                    inner
                } else {
                    unreachable!()
                }
            } else {
                unreachable!()
            }
        }
        rustdoc_types::Type::ResolvedPath(p) if get_name_without_path(&p.name) == "Option" => {
            if let Some(rustdoc_types::GenericArgs::AngleBracketed { args, .. }) = p.args.as_deref()
            {
                if let rustdoc_types::GenericArg::Type(tpe) = &args[0] {
                    let mut inner = to_serde_reflect_type(
                        tpe,
                        crate_map,
                        comment_map,
                        Vec::new(),
                        parent_crate,
                        namespace,
                        type_map,
                    );
                    let last = inner.last().unwrap().0.clone();
                    inner.push((Format::Option(Box::new(last)), None));
                    inner
                } else {
                    unreachable!()
                }
            } else {
                unreachable!()
            }
        }
        rustdoc_types::Type::ResolvedPath(p) if get_name_without_path(&p.name) == "Box" => {
            let t = match p.args.as_deref() {
                Some(rustdoc_types::GenericArgs::AngleBracketed { args, .. })
                    if args.len() == 1 =>
                {
                    if let Some(rustdoc_types::GenericArg::Type(t)) = args.first() {
                        t
                    } else {
                        unreachable!()
                    }
                }
                Some(_) | None => unreachable!(),
            };
            if recursive_type {
                let name = match t {
                    rustdoc_types::Type::ResolvedPath(p) => get_name_without_path(&p.name),
                    _ => unreachable!(),
                };
                // we need an explicit early return here as we **don't** want to
                // update the type map with the preliminary result
                return vec![(Format::TypeName(name.to_owned()), None)];
            } else {
                to_serde_reflect_type(
                    t,
                    crate_map,
                    comment_map,
                    parent_args,
                    parent_crate,
                    namespace,
                    type_map,
                )
            }
        }
        rustdoc_types::Type::ResolvedPath(p) => {
            let t = crate_map.resolve_index(Some(p), &p.id, parent_crate);
            let parent_crate = extract_crate_from_span(&t);
            if let Some(comment_map) = comment_map {
                if let Some(ref doc) = t.docs {
                    comment_map.insert(vec![namespace.to_owned(), p.name.clone()], doc.clone());
                }
            }
            if let rustdoc_types::ItemEnum::Struct(rustdoc_types::Struct {
                kind: rustdoc_types::StructKind::Plain { ref fields, .. },
                ..
            }) = t.inner
            {
                return generate_exported_struct(
                    fields,
                    crate_map,
                    comment_map,
                    p,
                    parent_args,
                    &parent_crate,
                    namespace,
                    type_map,
                    recursive_type,
                );
            }
            if let rustdoc_types::ItemEnum::Struct(rustdoc_types::Struct {
                kind: rustdoc_types::StructKind::Unit {},
                ..
            }) = t.inner
            {
                return generate_exported_struct(
                    &[],
                    crate_map,
                    comment_map,
                    p,
                    parent_args,
                    &parent_crate,
                    namespace,
                    type_map,
                    recursive_type,
                );
            }
            if let rustdoc_types::ItemEnum::Enum(ref e) = t.inner {
                return generate_exported_enum(
                    e,
                    crate_map,
                    comment_map,
                    p,
                    &parent_crate,
                    namespace,
                    type_map,
                    recursive_type,
                );
            }
            if let rustdoc_types::ItemEnum::TypeAlias(ref t) = t.inner {
                return to_serde_reflect_type(
                    &t.type_,
                    crate_map,
                    comment_map,
                    parent_args,
                    &parent_crate,
                    namespace,
                    type_map,
                );
            }
            dbg!(t);
            unimplemented!()
        }
        rustdoc_types::Type::DynTrait(_) => unimplemented!(),
        rustdoc_types::Type::Generic(p) => {
            if parent_args.len() == 1 {
                if let rustdoc_types::GenericArg::Type(ref t) = &parent_args[0] {
                    to_serde_reflect_type(
                        t,
                        crate_map,
                        comment_map,
                        Vec::new(),
                        parent_crate,
                        namespace,
                        type_map,
                    )
                } else {
                    unimplemented!("Only types are accepted here?")
                }
            } else {
                dbg!(parent_args);
                dbg!(p);
                unimplemented!("Unsure how to resolve multiple args here??")
            }
        }
        rustdoc_types::Type::Primitive(_) => reflect_primitive(t),
        rustdoc_types::Type::FunctionPointer(_) => unimplemented!(),
        rustdoc_types::Type::Tuple(tup) => {
            let mut out = Vec::new();
            let mut fields = Vec::with_capacity(tup.len());
            for f in tup {
                let r = to_serde_reflect_type(
                    f,
                    crate_map,
                    comment_map,
                    Vec::new(),
                    parent_crate,
                    namespace,
                    type_map,
                );
                let f = r.last().map(|a| a.0.clone()).unwrap();
                out.extend(r);
                fields.push(f);
            }
            out.push((Format::Tuple(fields), None));
            out
        }
        rustdoc_types::Type::Slice(_) => unimplemented!(),
        rustdoc_types::Type::Array { type_, len } => {
            let size = len.parse::<usize>().expect("Array len should be a number");
            let t = reflect_primitive(type_)[0].0.clone();
            vec![(
                Format::TupleArray {
                    content: Box::new(t),
                    size,
                },
                None,
            )]
        }
        rustdoc_types::Type::ImplTrait(_) => unimplemented!(),
        rustdoc_types::Type::Infer => unimplemented!(),
        rustdoc_types::Type::RawPointer { .. } => unimplemented!(),
        rustdoc_types::Type::Pat { .. } => unimplemented!(),

        rustdoc_types::Type::BorrowedRef { type_, .. } => {
            if let rustdoc_types::Type::Generic(s) = &**type_ {
                if s == "Self" {
                    return Vec::new();
                }
            }
            dbg!(t);
            unimplemented!()
        }
        rustdoc_types::Type::QualifiedPath { .. } => unimplemented!(),
    };

    type_map.insert(t.clone(), TypeCache::Cached(r.clone()));
    r
}

fn extract_crate_from_span(t: &rustdoc_types::Item) -> String {
    let p = &t.span.as_ref().expect("Span is set").filename;
    let mut components = p.components().peekable();
    let crate_name = match components.next() {
        Some(Component::Normal(el)) => {
            // that's a relative path in the project itself
            // we do walk down from the source files to the actual crate
            // name
            // This only works for reasonable default source setups,
            // such as those that have a src directory and where
            // the parent directory is named as the crate?
            // Fixme: It might be useful to use actual metadata from
            // cargo metadata to make this more robust
            let mut rev_components = components
                .rev()
                // skip everything before src as it's inside the
                // source directory
                .skip_while(|el| *el != Component::Normal(OsStr::new("src")))
                .skip(1); // need to skip "src" itself
            let Component::Normal(next) = rev_components.next().unwrap_or(Component::Normal(el))
            else {
                panic!("Could not resolve source path");
            };
            let s = next.to_str().expect("We expect an UTF-8 Path");
            // crate names do not contain `-` but `_`
            s.replace('-', "_")
        }
        Some(Component::RootDir | Component::Prefix(_)) => {
            // that's likely a path tho the cargo registry
            // So go to `.cargo`, remove 3 additional directories
            // and get the crate name
            // It's something like
            // `/home/weiznich/.cargo/registry/src/github.com-1ecc6299db9ec823/giga-segy-core-0.3.2/src/header_structs.rs`
            loop {
                match components.next() {
                    Some(Component::Normal(e))
                        if (e == ".cargo" || e == "cargo")
                            && matches!(components.peek(), Some(Component::Normal(e)) if *e == "registry") =>
                    {
                        break
                    }
                    None => panic!("Unexpected end of path: {}", p.display()),
                    _ => {}
                }
            }
            // "registry"
            components.next();
            // "src"
            components.next();
            // "github.com-*"
            components.next();
            let Some(Component::Normal(el)) = components.next() else {
                panic!("Expect a normal path element")
            };
            // that's cratename-version
            let s = el.to_str().expect("We expect an UTF-8 Path");
            // split from the back as the crate name might contain a `-` as well
            let Some((s, _)) = s.rsplit_once('-') else {
                panic!("Expect a versioned crate name")
            };
            // crate names do not contain `-` but `_`
            s.replace('-', "_")
        }
        _ => panic!("We expect a relative or absolute path here"),
    };
    crate_name
}

// we can't simply replace `parent_crate` and `namespace` by `config` because this function will
// be called by `to_serde_reflect_type` which can't hold a `config` (because `parent_crate` will be
// changed by the function itself and needs to stay mutable)
#[allow(clippy::too_many_arguments)]
fn generate_exported_enum(
    e: &rustdoc_types::Enum,
    crate_map: &ItemResolver,
    comment_map: &mut Option<BTreeMap<Vec<String>, String>>,
    p: &rustdoc_types::Path,
    parent_crate: &str,
    namespace: &str,
    type_map: &mut HashMap<rustdoc_types::Type, TypeCache>,
    recursive_type: bool,
) -> Vec<(
    serde_reflection::Format,
    Option<serde_reflection::ContainerFormat>,
)> {
    use serde_reflection::{ContainerFormat, Format};

    let mut out = Vec::new();
    let container_format = if recursive_type {
        // we can skip that the second time
        None
    } else {
        let mut enum_def = BTreeMap::new();
        for (id, variant) in e.variants.iter().enumerate() {
            let v = crate_map.resolve_index(None, variant, parent_crate);
            if let Some(comment_map) = comment_map {
                if let Some(ref docs) = v.docs {
                    comment_map.insert(
                        vec![
                            namespace.to_owned(),
                            p.name.clone(),
                            v.name.clone().unwrap(),
                        ],
                        docs.clone(),
                    );
                }
            }
            match v.inner {
                rustdoc_types::ItemEnum::Variant(rustdoc_types::Variant {
                    kind: rustdoc_types::VariantKind::Plain,
                    ..
                }) => {
                    enum_def.insert(
                        id as u32,
                        serde_reflection::Named {
                            name: v.name.clone().unwrap(),
                            value: serde_reflection::VariantFormat::Unit,
                        },
                    );
                }
                rustdoc_types::ItemEnum::Variant(rustdoc_types::Variant {
                    kind: rustdoc_types::VariantKind::Tuple(ref t),
                    ..
                }) => {
                    let mut variants = Vec::new();
                    for id in t {
                        if let Some(t) = id
                            .as_ref()
                            .map(|id| crate_map.resolve_index(None, id, parent_crate))
                        {
                            if let rustdoc_types::ItemEnum::StructField(ref tpe) = t.inner {
                                // check for a custom serde attribute here
                                // this allows us to specify different types for the c++ side
                                // we expect that we always set a fully qualified path to an type there
                                // (we control that, as it's our source, so that shouldn't be an problem)
                                if let Some(serde_type) = t.attrs.iter().find_map(|a| {
                                    let pref = a.strip_prefix("#[serde(with = \"")?;
                                    Some(&pref[..pref.len() - 3])
                                }) {
                                    let item = crate_map.resolve_by_path(
                                        serde_type,
                                        parent_crate,
                                        rustdoc_types::ItemKind::Struct,
                                    );
                                    let tpe = rustdoc_types::Type::ResolvedPath(item);
                                    let tps = to_serde_reflect_type(
                                        &tpe,
                                        crate_map,
                                        comment_map,
                                        Vec::new(),
                                        parent_crate,
                                        namespace,
                                        type_map,
                                    );
                                    variants.push(tps.last().unwrap().0.clone());
                                    out.extend(tps);
                                } else {
                                    let tps = to_serde_reflect_type(
                                        tpe,
                                        crate_map,
                                        comment_map,
                                        Vec::new(),
                                        parent_crate,
                                        namespace,
                                        type_map,
                                    );
                                    variants.push(tps.last().unwrap().0.clone());
                                    out.extend(tps);
                                }
                            }
                        }
                    }
                    if variants.len() == 1 {
                        let x = Box::new(variants.pop().expect("We have one. See above."));
                        enum_def.insert(
                            id as u32,
                            serde_reflection::Named {
                                name: v.name.clone().unwrap(),
                                value: serde_reflection::VariantFormat::NewType(x),
                            },
                        );
                    } else {
                        enum_def.insert(
                            id as u32,
                            serde_reflection::Named {
                                name: v.name.clone().unwrap(),
                                value: serde_reflection::VariantFormat::Tuple(variants),
                            },
                        );
                    }
                }
                rustdoc_types::ItemEnum::Variant(rustdoc_types::Variant {
                    kind: rustdoc_types::VariantKind::Struct { ref fields, .. },
                    ..
                }) => {
                    let mut variants = Vec::new();
                    for id in fields {
                        let t = crate_map.resolve_index(None, id, parent_crate);
                        if let rustdoc_types::ItemEnum::StructField(ref tpe) = t.inner {
                            let tps = to_serde_reflect_type(
                                tpe,
                                crate_map,
                                comment_map,
                                Vec::new(),
                                parent_crate,
                                namespace,
                                type_map,
                            );
                            variants.push(serde_reflection::Named {
                                name: t.name.unwrap(),
                                value: tps.last().unwrap().0.clone(),
                            });
                            out.extend(tps);
                        }
                    }

                    enum_def.insert(
                        id as u32,
                        serde_reflection::Named {
                            name: v.name.clone().unwrap(),
                            value: serde_reflection::VariantFormat::Struct(variants),
                        },
                    );
                }
                _ => unimplemented!(),
            }
        }
        Some(ContainerFormat::Enum(enum_def))
    };
    let name = get_name_without_path(&p.name);
    out.push((Format::TypeName(name.to_owned()), container_format));
    out
}

#[allow(clippy::too_many_arguments)]
fn generate_exported_struct(
    fields: &[rustdoc_types::Id],
    crate_map: &ItemResolver,
    comment_map: &mut Option<BTreeMap<Vec<String>, String>>,
    p: &rustdoc_types::Path,
    parent_args: Vec<rustdoc_types::GenericArg>,
    parent_crate: &str,
    namespace: &str,
    type_map: &mut HashMap<rustdoc_types::Type, TypeCache>,
    recursive_type: bool,
) -> Vec<(
    serde_reflection::Format,
    Option<serde_reflection::ContainerFormat>,
)> {
    use serde_reflection::{ContainerFormat, Format};

    let mut out = Vec::new();
    let mut name = get_name_without_path(&p.name).to_owned();
    if let Some(rustdoc_types::GenericArgs::AngleBracketed { args, .. }) = p.args.as_deref() {
        for arg in args {
            if let rustdoc_types::GenericArg::Type(ref t) = arg {
                let tpe = to_serde_reflect_type(
                    t,
                    crate_map,
                    comment_map,
                    parent_args.clone(),
                    parent_crate,
                    namespace,
                    type_map,
                )
                .pop()
                .unwrap()
                .0;
                name = format!("{name}_{}", to_type_name(&tpe));
            }
        }
    }
    let container_format = if recursive_type {
        // we don't need that for a recursive type
        None
    } else {
        let fields = fields
            .iter()
            .map(|id| crate_map.resolve_index(None, id, parent_crate))
            .filter_map(|s| {
                if let Some(ref mut comment_map) = comment_map {
                    if let Some(ref doc) = s.docs {
                        comment_map.insert(
                            vec![
                                namespace.to_owned(),
                                p.name.clone(),
                                s.name.clone().unwrap(),
                            ],
                            doc.clone(),
                        );
                    }
                }
                if let rustdoc_types::ItemEnum::StructField(ref tpe) = s.inner {
                    let parent_args = if let Some(rustdoc_types::GenericArgs::AngleBracketed {
                        args,
                        bindings,
                    }) = p.args.as_deref()
                    {
                        if args.is_empty() && bindings.is_empty() {
                            Vec::new()
                        } else if parent_args.len() == 1
                            && args.len() == 1
                            && matches!(
                                &args[0],
                                rustdoc_types::GenericArg::Type(rustdoc_types::Type::Generic(_))
                            )
                        {
                            parent_args.clone()
                        } else {
                            args.clone()
                        }
                    } else {
                        Vec::new()
                    };
                    Some((
                        s.name.clone().unwrap(),
                        to_serde_reflect_type(
                            tpe,
                            crate_map,
                            comment_map,
                            parent_args,
                            parent_crate,
                            namespace,
                            type_map,
                        ),
                    ))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        let mut struct_fields = Vec::with_capacity(fields.len());
        for (name, tpe) in fields {
            let format = tpe.last().unwrap().0.clone();
            struct_fields.push(serde_reflection::Named {
                name,
                value: format,
            });
            out.extend(tpe);
        }
        Some(ContainerFormat::Struct(struct_fields))
    };
    out.push((Format::TypeName(name), container_format));
    out
}

fn is_relevant_impl(item: &&rustdoc_types::Item) -> bool {
    if !item
        .attrs
        .contains(&String::from("#[cfg(not(generated_extern_impl))]"))
    {
        return false;
    }
    matches!(item.inner, rustdoc_types::ItemEnum::Impl(_))
}

fn is_free_standing_impl(item: &&rustdoc_types::Item) -> bool {
    if !item
        .attrs
        .contains(&String::from("#[cfg(not(generated_extern_impl))]"))
    {
        return false;
    }
    matches!(item.inner, rustdoc_types::ItemEnum::Function(_))
}

fn to_c_type(tpe: &rustdoc_types::Type) -> String {
    match tpe {
        rustdoc_types::Type::ResolvedPath(p) => {
            let mut ret = get_name_without_path(&p.name).trim().to_string();
            if ret == "c_char" {
                String::from("char")
            } else {
                if let Some(rustdoc_types::GenericArgs::AngleBracketed { args, .. }) =
                    p.args.as_deref()
                {
                    for arg in args {
                        if let rustdoc_types::GenericArg::Type(t) = arg {
                            write!(ret, "_{}", to_c_type(t)).unwrap();
                        }
                    }
                }
                ret
            }
        }
        rustdoc_types::Type::DynTrait(_) => unimplemented!(),
        rustdoc_types::Type::Generic(_) => unimplemented!(),
        rustdoc_types::Type::Primitive(p) if p == "u8" => String::from("std::uint8_t"),
        rustdoc_types::Type::Primitive(p) if p == "usize" => String::from("size_t"),
        rustdoc_types::Type::Primitive(p) if p == "u16" => String::from("std::uint16_t"),
        rustdoc_types::Type::Primitive(p) => p.clone(),
        rustdoc_types::Type::FunctionPointer(_) => String::new(),
        rustdoc_types::Type::Tuple(_) => unimplemented!(),
        rustdoc_types::Type::Slice(_) => unimplemented!(),
        rustdoc_types::Type::Array { .. } => unimplemented!(),
        rustdoc_types::Type::ImplTrait(_) => unimplemented!(),
        rustdoc_types::Type::Infer => unimplemented!(),
        rustdoc_types::Type::RawPointer { mutable, type_ } => {
            let mut out = if *mutable {
                String::new()
            } else {
                String::from("const ")
            };
            write!(out, "{}*", to_c_type(type_)).unwrap();
            out
        }
        rustdoc_types::Type::BorrowedRef { .. } => String::new(),
        rustdoc_types::Type::QualifiedPath { .. } => unimplemented!(),
        rustdoc_types::Type::Pat { .. } => unimplemented!(),
    }
}

fn generate_extern_c_function_def(name: &str, func: &rustdoc_types::Function) -> String {
    let mut out = String::from("extern \"C\" ");
    write!(
        out,
        "{} ",
        func.decl
            .output
            .as_ref()
            .map(to_c_type)
            .unwrap_or_else(|| "void".into())
    )
    .unwrap();

    let args = func
        .decl
        .inputs
        .iter()
        .map(|(name, tpe)| {
            let mut out = to_c_type(tpe);
            write!(out, " {name}").unwrap();
            out
        })
        .collect::<Vec<_>>()
        .join(", ");
    write!(out, "{name}({args});").unwrap();
    out
}

fn get_name_without_path(name: &str) -> &str {
    // sometimes the name include the full path now
    name.rsplit_once("::").map(|(_, e)| e).unwrap_or(name)
}
