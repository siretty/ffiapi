use askama::Template;
use crate::ast::*;
use std::rc::Rc;

// #[derive(Template)]
// #[template(path = "cpp-api/api.hpp.in", escape = "none", syntax = "hpp")]
// pub struct ApiHpp {
//     pub api_name: Rc<str>,
//     pub modules: Vec<Rc<Module>>,
// }
//
// #[derive(Template)]
// #[template(path = "cpp-api/common.hpp.in", escape = "none", syntax = "hpp")]
// pub struct BitsApiCommonHpp {
//     pub api_name: Rc<str>,
// }
//
// #[derive(Template)]
// #[template(path = "cpp-api/module.hpp.in", escape = "none", syntax = "hpp")]
// pub struct BitsModuleHpp {
//     pub module: Rc<Module>,
// }
//
// #[derive(Template)]
// #[template(path = "cpp-api/module-impl-prefix.ipp.in", escape = "none", syntax = "hpp")]
// pub struct BitsModuleImplPrefixHpp {
//     pub module: Rc<Module>,
// }
//
// #[derive(Template)]
// #[template(path = "cpp-api/module-ffi.ipp.in", escape = "none", syntax = "hpp")]
// pub struct BitsModuleFfiHpp {
//     pub module: Rc<Module>,
// }
//
// #[derive(Template)]
// #[template(path = "cpp-api/module-impl-suffix.ipp.in", escape = "none", syntax = "hpp")]
// pub struct BitsModuleImplSuffixHpp {
//     pub module: Rc<Module>,
// }
//
// pub mod filters {
//     use askama::Result as FilterResult;
//     use crate::ast::*;
//     use std::rc::Rc;
//
//     pub struct CppParameterList(Vec<Rc<Parameter>>);
//
//     impl core::fmt::Display for CppParameterList {
//         fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> std::fmt::Result {
//             let mut it = self.0.iter().peekable();
//             loop {
//                 if let Some(parameter) = it.next() {
//                     write!(f, "{} {}", CppType(parameter.type_.clone()), parameter.name())?;
//                 } else {
//                     break;
//                 }
//
//                 if it.peek().is_some() {
//                     write!(f, ", ")?;
//                 }
//             }
//             Ok(())
//         }
//     }
//
//     pub fn parameter_list(parameters: Vec<Rc<Parameter>>) -> FilterResult<String> {
//         Ok(CppParameterList(parameters).to_string())
//     }
//
//
//     pub struct CppFfiParameterList(Vec<Rc<Parameter>>);
//
//     impl core::fmt::Display for CppFfiParameterList {
//         fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> std::fmt::Result {
//             let mut it = self.0.iter().peekable();
//             loop {
//                 if let Some(parameter) = it.next() {
//                     write!(f, "{} {}", CppFfiType(parameter.type_.clone()), parameter.name())?;
//                 } else {
//                     break;
//                 }
//
//                 if it.peek().is_some() {
//                     write!(f, ", ")?;
//                 }
//             }
//             Ok(())
//         }
//     }
//
//     pub fn ffi_parameter_list(parameters: Vec<Rc<Parameter>>) -> FilterResult<String> {
//         Ok(CppFfiParameterList(parameters).to_string())
//     }
//
//     pub struct CppFfiArgumentList(Vec<(Rc<Identifier>, Rc<Type>)>);
//
//     impl core::fmt::Display for CppFfiArgumentList {
//         fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
//             fn matches(qi: &Vec<Rc<Identifier>>, strings: &[&str]) -> bool {
//                 qi.iter()
//                     .map(|rc| rc.0.as_ref())
//                     .eq(strings.iter().map(|item| *item))
//             }
//
//             fn is_pass_by_value(qi: &Vec<Rc<Identifier>>) -> bool {
//                 if matches(qi, &["size"]) { return true; }
//                 if matches(qi, &["pointer"]) { return true; }
//                 if matches(qi, &["user_data"]) { return true; }
//                 false
//             }
//
//             fn write(f: &mut core::fmt::Formatter<'_>, name: &str, type_: &Type) -> core::fmt::Result {
//                 match type_ {
//                     Type::Qualified(qi, _src) if !is_pass_by_value(qi) => {
//                         write!(f, "{}.interface_data", name)
//                     }
//                     Type::Ref(type_, _src) | Type::Mut(type_, _src) => {
//                         write!(f, "&")?;
//                         write(f, name, type_)
//                     }
//                     _ => {
//                         write!(f, "{}", name)
//                     }
//                 }
//             }
//
//             let mut it = self.0.iter().peekable();
//             loop {
//                 if let Some((name, type_)) = it.next() {
//                     write(f, name.as_str(), type_.as_ref())?;
//                 } else {
//                     break;
//                 }
//
//                 if it.peek().is_some() {
//                     write!(f, ", ")?;
//                 }
//             }
//             Ok(())
//         }
//     }
//
//     pub fn ffi_argument_list(parameters: Vec<Rc<Parameter>>) -> FilterResult<String> {
//         Ok(CppFfiArgumentList(
//             parameters.iter()
//                 .map(|parameter| (parameter.name.clone(), parameter.type_.clone()))
//                 .collect()
//         ).to_string())
//     }
//
//     pub fn ffi_return_value(return_type: Rc<Type>) -> FilterResult<String> {
//         Ok(CppFfiArgumentList(vec![
//             (Rc::new(Identifier("_result".into(), None)), Rc::new(Type::Mut(return_type, None)))
//         ]).to_string())
//     }
//
//     pub struct CppQualifiedIdentifier(Vec<Rc<Identifier>>);
//
//     impl core::fmt::Display for CppQualifiedIdentifier {
//         fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> std::fmt::Result {
//             let mut it = self.0.iter().peekable();
//             loop {
//                 if let Some(identifier) = it.next() {
//                     write!(f, "{}", identifier.0)?;
//                 } else {
//                     break;
//                 }
//
//                 if it.peek().is_some() {
//                     write!(f, "::")?;
//                 }
//             }
//             Ok(())
//         }
//     }
//
//     pub struct CppType(Rc<Type>);
//
//     impl core::fmt::Display for CppType {
//         fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> std::fmt::Result {
//             match self.0.as_ref() {
//                 Type::Ref(type_, ..) => {
//                     match type_.as_ref() {
//                         Type::Ref(..) | Type::Mut(..) => write!(f, "{} const &", CppType(type_.clone()))?,
//                         _ => write!(f, "const {} &", CppType(type_.clone()))?,
//                     }
//                 }
//                 Type::Mut(type_, ..) => write!(f, "{} &", CppType(type_.clone()))?,
//                 Type::Slice(type_, ..) => write!(f, "/* TODO */")?,
//                 Type::Array(type_, size, ..) => write!(f, "std::array<{}, {}>", CppType(type_.clone()), size.0)?,
//                 Type::CodeType(identifier, ..) => write!(f, "{}_code", identifier.0)?,
//                 Type::Qualified(identifiers, ..) => write!(f, "{}", CppQualifiedIdentifier(identifiers.clone()))?,
//             }
//             Ok(())
//         }
//     }
//
//     pub fn cpp_type(type_: Rc<Type>) -> FilterResult<String> {
//         Ok(CppType(type_).to_string())
//     }
//
//     pub fn cpp_type_or_void(type_: Option<Rc<Type>>) -> FilterResult<String> {
//         let string = match type_ {
//             Some(type_) => cpp_type(type_)?,
//             None => "void".to_string(),
//         };
//         Ok(string)
//     }
//
//     pub struct CppFfiType(Rc<Type>);
//
//     impl core::fmt::Display for CppFfiType {
//         fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> std::fmt::Result {
//             match self.0.as_ref() {
//                 Type::Ref(type_, ..) => {
//                     match type_.as_ref() {
//                         Type::Ref(..) | Type::Mut(..) => write!(f, "{} const *", CppFfiType(type_.clone()))?,
//                         _ => write!(f, "const {} *", CppFfiType(type_.clone()))?,
//                     }
//                 }
//                 Type::Mut(type_, ..) => write!(f, "{} *", CppFfiType(type_.clone()))?,
//                 Type::Slice(type_, ..) => write!(f, "/* TODO */")?,
//                 Type::Array(type_, size, ..) => write!(f, "std::array<{}, {}>", CppFfiType(type_.clone()), size.0)?,
//                 Type::CodeType(identifier, ..) => write!(f, "{}_code", identifier.0)?,
//                 Type::Qualified(identifiers, ..) => write!(f, "{}", CppQualifiedIdentifier(identifiers.clone()))?,
//             }
//             Ok(())
//         }
//     }
//
//     pub fn cpp_ffi_type(type_: Rc<Type>) -> FilterResult<String> {
//         Ok(CppFfiType(type_).to_string())
//     }
//
//     pub fn cpp_ffi_type_or_void(type_: Option<Rc<Type>>) -> FilterResult<String> {
//         let string = match type_ {
//             Some(type_) => cpp_ffi_type(type_)?,
//             None => "void".to_string(),
//         };
//         Ok(string)
//     }
// }