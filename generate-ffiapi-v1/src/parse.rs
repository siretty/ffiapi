mod parse_error;
mod parse_result;
mod assertions;
pub mod generic;

use crate::{
    ast::*,
    grammar::{Grammar, Rule},
};
use pest::{Parser, iterators::{Pair, Pairs}};
use self::{
    assertions::*,
    generic::{
        next as _parse_next,
        last as _parse_last,
        homogeneous_remainder as _parse_homogeneous_remainder,
        homogeneous_run as _parse_homogeneous_run,
    },
};
use std::rc::Rc;

pub use self::{
    parse_error::ParseError,
    parse_result::ParseResult,
};

pub fn document(input: &Input, input_pair: Pair<Rule>) -> ParseResult<Document> {
    assert_as_rule_eq!(input_pair, Rule::Document);
    let src = Some(Src::from_pair(input, &input_pair));

    let mut items = Vec::new();

    for pair in input_pair.into_inner() {
        match pair.as_rule() {
            Rule::DocumentItem => {
                let model = document_item(input, pair)?;
                items.push(model);
            }
            Rule::EOI => { /* no-op */ }
            rule => assert_other_rule!(rule),
        }
    }

    Ok(Document { src, items })
}

fn document_item(input: &Input, input_pair: Pair<Rule>) -> ParseResult<DocumentItem> {
    assert_as_rule_eq!(input_pair, Rule::DocumentItem);
    let input_inner = input_pair.into_inner();

    let item_pair = checked_take_last_some!(input_inner);
    match item_pair.as_rule() {
        Rule::Enumeration => {
            let model = (input, item_pair)?;
            Ok(DocumentItem::Enumeration(model))
        }
        rule => assert_other_rule!(rule),
    }
}

// fn parse_module(input: &Input, pair: Pair<Rule>) -> ParseResult<Rc<Module>> {
//     assert_as_rule_eq!(pair, Rule::Module);
//     let src = Some(Src::from_pair(input, &pair));
//     let mut inner = pair.into_inner();
//
//     let name = (
//         _parse_next(input, &mut inner, parse_identifier)?,
//         _parse_next(input, &mut inner, parse_identifier)?,
//     );
//     let code = _parse_next(input, &mut inner, parse_number)?;
//     let items = _parse_homogeneous_remainder(input, inner, parse_module_item, Rule::ModuleItem)?;
//
//     Ok(Rc::new(Module { src, name, code, items }))
// }
//
// fn parse_module_item(input: &Input, input_pair: Pair<Rule>) -> ParseResult<ModuleItem> {
//     assert_as_rule_eq!(input_pair, Rule::ModuleItem);
//     let input_inner = input_pair.into_inner();
//
//     let item_pair = checked_take_last_some!(input_inner);
//     match item_pair.as_rule() {
//         Rule::Error => {
//             let model = parse_error(input, item_pair)?;
//             Ok(ModuleItem::Error(model))
//         }
//         Rule::Command => {
//             let model = parse_command(input, item_pair)?;
//             Ok(ModuleItem::Command(model))
//         }
//         Rule::Event => {
//             let model = parse_event(input, item_pair)?;
//             Ok(ModuleItem::Event(model))
//         }
//         Rule::Interface => {
//             let model = parse_interface(input, item_pair)?;
//             Ok(ModuleItem::Interface(model))
//         }
//         rule => assert_other_rule!(rule),
//     }
// }
//
// fn parse_error(input: &Input, pair: Pair<Rule>) -> ParseResult<Rc<Error>> {
//     assert_as_rule_eq!(pair, Rule::Error);
//     let src = Some(Src::from_pair(input, &pair));
//     let mut inner = pair.into_inner();
//
//     let name = _parse_next(input, &mut inner, parse_identifier)?;
//     let code = _parse_next(input, &mut inner, parse_number)?;
//
//     Ok(Rc::new(Error { src, name, code }))
// }
//
// fn parse_command(input: &Input, pair: Pair<Rule>) -> ParseResult<Rc<Command>> {
//     assert_as_rule_eq!(pair, Rule::Command);
//     let src = Some(Src::from_pair(input, &pair));
//     let mut inner = pair.into_inner();
//
//     let name = _parse_next(input, &mut inner, parse_identifier)?;
//     let code = _parse_next(input, &mut inner, parse_number)?;
//     let members = _parse_homogeneous_remainder(input, inner, parse_member, Rule::Member)?;
//
//     Ok(Rc::new(Command { src, name, code, members }))
// }
//
// fn parse_event(input: &Input, pair: Pair<Rule>) -> ParseResult<Rc<Event>> {
//     assert_as_rule_eq!(pair, Rule::Event);
//     let src = Some(Src::from_pair(input, &pair));
//     let mut inner = pair.into_inner();
//
//     let name = _parse_next(input, &mut inner, parse_identifier)?;
//     let code = _parse_next(input, &mut inner, parse_number)?;
//     let members = _parse_homogeneous_remainder(input, inner, parse_member, Rule::Member)?;
//
//     Ok(Rc::new(Event { src, name, code, members }))
// }
//
// fn parse_member(input: &Input, pair: Pair<Rule>) -> ParseResult<Rc<Member>> {
//     assert_as_rule_eq!(pair, Rule::Member);
//     let src = Some(Src::from_pair(input, &pair));
//     let mut inner = pair.into_inner();
//
//     let name = _parse_next(input, &mut inner, parse_identifier)?;
//     let type_ = _parse_last(input, inner, parse_type)?;
//
//     Ok(Rc::new(Member { src, name, type_ }))
// }
//
// fn parse_interface(input: &Input, pair: Pair<Rule>) -> ParseResult<Rc<Interface>> {
//     assert_as_rule_eq!(pair, Rule::Interface);
//     let src = Some(Src::from_pair(input, &pair));
//     let mut inner = pair.into_inner();
//
//     let name = _parse_next(input, &mut inner, parse_identifier)?;
//     let items = _parse_homogeneous_remainder(input, inner, parse_interface_item, Rule::InterfaceItem)?;
//
//     Ok(Rc::new(Interface { src, name, items }))
// }
//
// fn parse_interface_item(input: &Input, pair: Pair<Rule>) -> ParseResult<InterfaceItem> {
//     assert_as_rule_eq!(pair, Rule::InterfaceItem);
//     let inner = pair.into_inner();
//
//     let item_pair = checked_take_last_some!(inner);
//     match item_pair.as_rule() {
//         Rule::InterfaceCtor => {
//             let model = parse_interface_ctor(input, item_pair)?;
//             Ok(InterfaceItem::Ctor(model))
//         }
//         Rule::InterfaceDtor => {
//             let model = parse_interface_dtor(input, item_pair)?;
//             Ok(InterfaceItem::Dtor(model))
//         }
//         Rule::InterfaceMethod => {
//             let model = parse_interface_method(input, item_pair)?;
//             Ok(InterfaceItem::Method(model))
//         }
//         Rule::Verbatim => {
//             let model = parse_verbatim(input, item_pair)?;
//             Ok(InterfaceItem::Verbatim(model))
//         }
//         rule => assert_other_rule!(rule),
//     }
// }
//
// fn parse_interface_ctor(input: &Input, pair: Pair<Rule>) -> ParseResult<Rc<InterfaceCtor>> {
//     assert_as_rule_eq!(pair, Rule::InterfaceCtor);
//     let src = Some(Src::from_pair(input, &pair));
//     let mut inner = pair.into_inner();
//
//     let name = _parse_next(input, &mut inner, parse_identifier)?;
//     let parameters = _parse_homogeneous_remainder(input, inner, parse_parameter, Rule::Parameter)?;
//
//     Ok(Rc::new(InterfaceCtor { src, name, parameters }))
// }
//
// fn parse_interface_dtor(input: &Input, pair: Pair<Rule>) -> ParseResult<Rc<InterfaceDtor>> {
//     assert_as_rule_eq!(pair, Rule::InterfaceDtor);
//     let src = Some(Src::from_pair(input, &pair));
//     let inner = pair.into_inner();
//
//     let name = _parse_last(input, inner, parse_identifier)?;
//
//     Ok(Rc::new(InterfaceDtor { src, name }))
// }
//
// fn parse_interface_method(input: &Input, pair: Pair<Rule>) -> ParseResult<Rc<InterfaceMethod>> {
//     assert_as_rule_eq!(pair, Rule::InterfaceMethod);
//     let src = Some(Src::from_pair(input, &pair));
//     let mut inner = pair.into_inner();
//
//     let name = _parse_next(input, &mut inner, parse_identifier)?;
//     let parameters = _parse_homogeneous_run(input, &mut inner, parse_parameter, Rule::Parameter)?;
//     let return_type = if inner.peek().is_some() {
//         Some(_parse_last(input, inner, parse_type)?)
//     } else {
//         None
//     };
//     println!("interface method {} -> {:?}", name.as_str(), return_type);
//
//     Ok(Rc::new(InterfaceMethod { src, name, parameters, return_type }))
// }
//
// fn parse_verbatim(input: &Input, pair: Pair<Rule>) -> ParseResult<Rc<Verbatim>> {
//     assert_as_rule_eq!(pair, Rule::Verbatim);
//     let src = Some(Src::from_pair(input, &pair));
//     let mut inner = pair.into_inner();
//
//     let a = _parse_next(input, &mut inner, parse_identifier)?;
//     let b = _parse_next(input, &mut inner, parse_identifier)?;
//     let content = _parse_last(input, inner, parse_verbatim_content)?;
//
//     Ok(Rc::new(Verbatim { src, a, b, content }))
// }
//
// fn parse_parameter(input: &Input, pair: Pair<Rule>) -> ParseResult<Rc<Parameter>> {
//     assert_as_rule_eq!(pair, Rule::Parameter);
//     let src = Some(Src::from_pair(input, &pair));
//     let mut inner = pair.into_inner();
//
//     let name = _parse_next(input, &mut inner, parse_identifier)?;
//     let type_ = _parse_last(input, inner, parse_type)?;
//
//     Ok(Rc::new(Parameter { src, name, type_ }))
// }
//
// fn parse_verbatim_content(input: &Input, pair: Pair<Rule>) -> ParseResult<Rc<VerbatimContent>> {
//     assert_as_rule_eq!(pair, Rule::VerbatimContent);
//     let src = Some(Src::from_pair(input, &pair));
//
//     let str = pair.as_str();
//     if str.is_empty() { assert_invalid_identifier!(str) }
//
//     Ok(Rc::new(VerbatimContent(str.into(), src)))
// }
//
// fn parse_identifier(input: &Input, pair: Pair<Rule>) -> ParseResult<Rc<Identifier>> {
//     assert_as_rule_eq!(pair, Rule::Identifier);
//     let src = Some(Src::from_pair(input, &pair));
//
//     let str = pair.as_str();
//     if str.is_empty() { assert_invalid_identifier!(str) }
//
//     Ok(Rc::new(Identifier(str.into(), src)))
// }
//
// // fn parse_integer_type(input_pair: Pair<Rule>) -> ParseResult<IntegerType> {
// //     assert_as_rule_eq!(input_pair, Rule::IntegerType);
// //
// //     let integer_type = match input_pair.as_str() {
// //         "u64" => IntegerType::U64,
// //         other => panic!("{}: {:?} is not a valid integer type", BUG_PREFIX, other),
// //     };
// //
// //     Ok(integer_type)
// // }
// //
// // fn parse_code_type(input_pair: Pair<Rule>) -> ParseResult<CodeType> {
// //     assert_as_rule_eq!(input_pair, Rule::CodeType);
// //
// //     let integer_type = match input_pair.as_str() {
// //         "command" => CodeType::Command,
// //         "error" => CodeType::Error,
// //         "event" => CodeType::Event,
// //         "module" => CodeType::Module,
// //         other => panic!("{}: {:?} is not a valid integer type", BUG_PREFIX, other),
// //     };
// //
// //     Ok(integer_type)
// // }
// //
// // fn parse_code_number(input_pair: Pair<Rule>) -> ParseResult<CodeNumber> {
// //     assert_as_rule_eq!(input_pair, Rule::CodeNumber);
// //
// //     let number = _parse_number(input_pair)?;
// //     Ok(CodeNumber(number))
// // }
//
// fn parse_number(input: &Input, pair: Pair<Rule>) -> ParseResult<Rc<Number>> {
//     assert_as_rule_eq!(pair, Rule::Number);
//     let src = Some(Src::from_pair(input, &pair));
//     let number = Number(pair.as_str().into(), src);
//     Ok(Rc::new(number))
// }
//
// fn parse_type(input: &Input, pair: Pair<Rule>) -> ParseResult<Rc<Type>> {
//     assert_as_rule_eq!(pair, Rule::Type);
//     let src = Some(Src::from_pair(input, &pair));
//     let inner = pair.into_inner();
//     let inner_pair = checked_take_last_some!(inner);
//     let inner_rule = inner_pair.as_rule();
//
//     match inner_rule {
//         Rule::RefType => {
//             let model = _parse_last(input, inner_pair.into_inner(), parse_type)?;
//             Ok(Rc::new(Type::Ref(model, src)))
//         }
//         Rule::MutType => {
//             let model = _parse_last(input, inner_pair.into_inner(), parse_type)?;
//             Ok(Rc::new(Type::Mut(model, src)))
//         }
//         Rule::SliceType => {
//             let model = _parse_last(input, inner_pair.into_inner(), parse_type)?;
//             Ok(Rc::new(Type::Slice(model, src)))
//         }
//         Rule::ArrayType => {
//             let mut inner_inner = inner_pair.into_inner();
//             let kind = _parse_next(input, &mut inner_inner, parse_type)?;
//             let size = _parse_last(input, inner_inner, parse_number)?;
//             Ok(Rc::new(Type::Array(kind, size, src)))
//         }
//         Rule::CodeType => {
//             let name = _parse_last(input, inner_pair.into_inner(), parse_identifier)?;
//             Ok(Rc::new(Type::CodeType(name, src)))
//         }
//         Rule::Qualified => {
//             let inner_inner = inner_pair.into_inner();
//             let qi = _parse_homogeneous_remainder(input, inner_inner, parse_identifier, Rule::Identifier)?;
//             Ok(Rc::new(Type::Qualified(qi, src)))
//         }
//         rule => assert_other_rule!(rule),
//     }
// }