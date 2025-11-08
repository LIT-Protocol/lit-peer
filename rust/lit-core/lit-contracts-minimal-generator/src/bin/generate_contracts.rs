use ethers::prelude::*;
use std::fs::{copy, read_dir, write};
fn main() {
    // process lit contracts
    let result = read_dir("../lit-blockchain/abis/");

    if result.is_err() {
        return;
    }

    let files = result.unwrap();
    for file in files.flatten() {
        let file_path = file.path().canonicalize().unwrap();
        let abi_source = file_path.to_str().unwrap();
        let abi_source_lite =
            abi_source.replace("/lit-blockchain/abis/", "/lit-blockchain-lite/abis/");
        let result = Abigen::from_file(abi_source);

        match result {
            Ok(res) => {
                let result = res.add_derive("serde::Serialize");
                let result = result.unwrap();
                let result = result.add_derive("serde::Deserialize");
                let result = result.unwrap();

                if let Ok(bindings) = result.generate() {
                    let source_file_name =
                        format!("../lit-blockchain/src/contracts/{:}.rs", &bindings.module_name());

                    // replace absolute path with relative
                    let as_str = bindings.to_string();
                    let as_str = as_str
                        .replace(abi_source, file.path().to_str().unwrap())
                        .replace("../lit-blockchain/abis/", "../../abis/");

                    write(source_file_name, as_str.clone()).expect("Could not write file.");

                    let source_file_lite_name = format!(
                        "../lit-blockchain-lite/src/contracts/{:}.rs",
                        &bindings.module_name()
                    );

                    // this is for lit-blockchain-lite, which is used for the lite node monitor service and is a WASM target ( web ui )
                    let as_str = make_safe_for_wasm(as_str);
                    write(source_file_lite_name, as_str).expect("Could not write file.");

                    copy(abi_source, abi_source_lite).expect("Could not copy file.");
                }
            }
            Err(..) => {
                println!("Error generating ABI for {:?}:  {:?}", file_path, result.unwrap_err());
            }
        }
    }

    // make the contents safe for wasm
    // we need to replace some of the __abi internals with function calls to reduce local stack usage.
    fn make_safe_for_wasm(contents: String) -> String {
        let new_contents = make_chunk_safe_for_wasm(contents, "functions", "Function");
        let new_contents = make_chunk_safe_for_wasm(new_contents, "events", "Event");
        let new_contents = make_chunk_safe_for_wasm(new_contents, "errors", "AbiError");
        new_contents
    }

    fn make_chunk_safe_for_wasm(contents: String, chunk_prefix: &str, chunk_type: &str) -> String {
        let function_abi = r#"#[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {"#;

        let new_function = r#"#[allow(deprecated)]
        fn __abi_{chunk_prefix}() -> std::collections::BTreeMap<String, Vec<::ethers::core::abi::ethabi::{chunk_type}>> {
        {function_combine}
        std::collections::BTreeMap::from(
        {function_content}
        )
        }
        
        {function_abi}
        "#;

        let btreemap_delimiter = r#"),
                ("#;

        let close_delimiter = r#")
        ]"#;
        let open_delimiter = r#"[
        ("#;

        let new_function = new_function.replace("{chunk_prefix}", chunk_prefix);
        let new_function = new_function.replace("{chunk_type}", chunk_type);
        let function_name = &format!("__abi_{}()", chunk_prefix);

        let chunk_name = &format!("{}: ::core::convert::From::from([", chunk_prefix);
        let start = match contents.find(chunk_name) {
            Some(start) => start + chunk_name.len() - 1, // -1 to remove the [
            None => return contents,
        };
        // let start = contents.find(chunk_name).unwrap() + chunk_name.len() - 1; // -1 to remove the [
        let old_function_content = contents[start..].to_string();
        let end = old_function_content.find("])").unwrap() + 1; // +1 to remove the ]
        let old_function_content = old_function_content[..end].to_string();

        // Ugg.  If any individual function is too large, we need to split it up.
        let function_contentparts =
            old_function_content.split(btreemap_delimiter).collect::<Vec<&str>>();
        let split_size = 50;
        let function_split = function_contentparts.len() / split_size;
        if function_split > 0 {
            let mut sub_function_names = Vec::new();
            let mut sub_functions = Vec::new();

            for i in 0..=function_split {
                let offset = i * split_size;
                // take the first split-size items and join them with the btree delimiter
                let sub_function_content = function_contentparts
                    .iter()
                    .skip(offset)
                    .take(split_size)
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>();
                let sub_function_content = sub_function_content.join(btreemap_delimiter);
                let sub_function_content = if i == 0 {
                    format!("{}{}", sub_function_content, close_delimiter)
                } else if i == function_split {
                    format!("{}{}", open_delimiter, sub_function_content)
                } else {
                    format!("{}{}{}", open_delimiter, sub_function_content, close_delimiter)
                };

                let sub_function = new_function.replace("() ->", &format!("_{}() ->", (i + 1)));
                let sub_function =
                    sub_function.replace("{function_content}", &sub_function_content);
                let sub_function = sub_function.replace("{function_abi}", "");
                let sub_function = sub_function.replace("{function_combine}", "");
                // println!("{}", sub_function[..100].to_string());
                sub_function_names.push(format!("__abi_{}_{}()", chunk_prefix, (i + 1)));
                sub_functions.push(sub_function);
            }

            let function_combine_var =
                sub_function_names[0].clone().replace("__", "").replace("_1()", "");
            let first_sub_function_name = sub_function_names[0].clone();
            let mut function_combine =
                format!("let mut {} = {};", function_combine_var.clone(), first_sub_function_name);
            for sub_function_name in sub_function_names {
                if &sub_function_name != &first_sub_function_name {
                    function_combine = function_combine + "\r\n";
                    function_combine = function_combine
                        + format!(
                            "{}.append(&mut {}); \r\n",
                            function_combine_var, sub_function_name
                        )
                        .as_str();
                }
            }

            let new_function_abis = sub_functions.join("\r\n") + "\r\n" + function_abi;

            let new_contents = contents.replace(&old_function_content, function_name);
            let new_function = new_function.replace("{function_abi}", &new_function_abis);
            let new_function = new_function.replace("{function_content}", &function_combine_var);
            let new_function = new_function.replace("{function_combine}", &function_combine);

            let new_contents = new_contents.replace(function_abi, &new_function);

            return new_contents;
        }

        let new_contents = contents.replace(&old_function_content, function_name);
        let new_function = new_function.replace("{function_abi}", function_abi);
        let new_function = new_function.replace("{function_content}", &old_function_content);
        let new_function = new_function.replace("{function_combine}", "");

        let new_contents = new_contents.replace(function_abi, &new_function);

        new_contents
    }
}
