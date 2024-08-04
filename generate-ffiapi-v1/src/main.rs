//use crate::templates::BitsModuleHpp;
use std::collections::HashSet;

mod ast;
mod grammar;
mod parse;
mod templates;

use self::grammar::Grammar;

fn main() {
    env_logger::builder()
        .default_format()
        .filter_level(log::LevelFilter::Warn)
        .parse_env("GENERATE_FFIAPI_V1_LOG")
        .init();

    let path = std::env::args().next()
        .expect("failed to read ffiapi file from input path");

    let document = match Grammar::read_and_parse(path) {
        Ok(document) => document,
        Err(err) => {
            log::error!("failed to parse document:\n{}", err);
            return;
        }
    };

    //let modules = document.modules();

    //let mut api_names = HashSet::new();

    // for module in modules.iter() {
    //     use askama::Template;
    //     use templates::{BitsApiCommonHpp, BitsModuleHpp, BitsModuleImplPrefixHpp, BitsModuleFfiHpp, BitsModuleImplSuffixHpp};

    //     let api_name = module.api_name();

    //     if api_names.insert(api_name.clone()) {
    //         std::fs::remove_dir_all("target/generated")
    //             .expect("failed to remove target directories");

    //         std::fs::create_dir_all(format!("target/generated/bits/{}/modules", api_name))
    //             .expect("failed to create target directories");

    //         let template = BitsApiCommonHpp { api_name: api_name.clone() };
    //         let code = template.render()
    //             .expect("failed to render template");

    //         println!("{}", code);

    //         std::fs::write(format!("target/generated/bits/{}/common.hpp", api_name), code)
    //             .expect("failed to save code");
    //     }

    //     let module_name = module.module_name();

    //     render_into_file(
    //         BitsModuleHpp { module: module.clone() },
    //         format!("target/generated/bits/{}/modules/{}.hpp", api_name, module_name),
    //     );

    //     render_into_file(
    //         BitsModuleImplPrefixHpp { module: module.clone() },
    //         format!("target/generated/bits/{}/modules/{}-impl-prefix.ipp", api_name, module_name),
    //     );

    //     render_into_file(
    //         BitsModuleFfiHpp { module: module.clone() },
    //         format!("target/generated/bits/{}/modules/{}-ffi.ipp", api_name, module_name),
    //     );

    //     render_into_file(
    //         BitsModuleImplSuffixHpp { module: module.clone() },
    //         format!("target/generated/bits/{}/modules/{}-impl-suffix.ipp", api_name, module_name),
    //     );
    // }

    // for api_name in api_names.iter() {
    //     use askama::Template;
    //     use templates::ApiHpp;

    //     let api_modules = document
    //         .modules().iter()
    //         .filter_map(|module| if &module.api_name() == api_name { Some(module.clone()) } else { None })
    //         .collect();

    //     let template = ApiHpp { api_name: api_name.clone(), modules: api_modules };
    //     let code = template.render()
    //         .expect("failed to render template");

    //     println!("{}", code);

    //     std::fs::write(format!("target/generated/{}.hpp", api_name), code)
    //         .expect("failed to save code");
    // }
}

fn render_into_file<T: askama::Template>(template: T, path: String) {
    let module_impl_code = template.render()
        .expect("failed to render template");

    println!("{}", module_impl_code);

    std::fs::write(path, module_impl_code)
        .expect("failed to save code");
}
