use crate::grammar::Rule;
use derive_debug::Dbg;
use pest::iterators::Pair;
use std::rc::Rc;

// Support Types

#[derive(Clone, Debug)]
pub struct Input {
    pub path: Option<Rc<str>>,
    pub text: Option<Rc<str>>,
}

#[derive(Clone, Debug)]
pub struct Src {
    pub input: Input,
    pub start: usize,
    pub end: usize,
}

impl Src {
    pub fn from_pair(input: &Input, pair: &Pair<Rule>) -> Self {
        let span = pair.as_span();
        Src {
            input: input.clone(),
            start: span.start(),
            end: span.end(),
        }
    }
}

// AST Types

#[derive(Dbg)]
pub struct Document {
    #[dbg(skip)]
    pub src: Option<Src>,
    pub items: Vec<DocumentItem>,
}

impl Document {
    pub fn modules(&self) -> Vec<Rc<Module>> {
        self.items.iter()
            .filter_map(|item| match item {
                DocumentItem::Module(rc_module) => Some(rc_module.clone()),
                _ => None,
            })
            .collect()
    }
}

#[derive(Debug)]
pub enum DocumentItem {
    Enumeration(Rc<Enumeration>),
}

#[derive(Dbg)]
pub struct Enumeration {
    #[dbg(skip)]
    pub src: Option<Src>,
    pub name: Identifier,
}

#[derive(Dbg)]
pub struct EnumerationItem {
    #[dbg(skip)]
    pub stc: Option<Src>,
}

// #[derive(Dbg)]
// pub struct Module {
//     #[dbg(skip)]
//     pub src: Option<Src>,
//     pub name: (Rc<Identifier>, Rc<Identifier>),
//     pub code: Rc<IntegerLiteral>,
//     pub items: Vec<ModuleItem>,
// }
//
// impl Module {
//     pub fn api_name(&self) -> Rc<str> {
//         self.name.0.0.clone()
//     }
//
//     pub fn module_name(&self) -> Rc<str> {
//         self.name.1.0.clone()
//     }
//
//     pub fn code(&self) -> Rc<str> {
//         self.code.0.clone()
//     }
//
//     pub fn errors(&self) -> Vec<Rc<Error>> {
//         self.items.iter()
//             .filter_map(|item| match item {
//                 ModuleItem::Error(rc_error) => Some(rc_error.clone()),
//                 _ => None,
//             })
//             .collect()
//     }
//
//     pub fn commands(&self) -> Vec<Rc<Command>> {
//         self.items.iter()
//             .filter_map(|item| match item {
//                 ModuleItem::Command(rc_command) => Some(rc_command.clone()),
//                 _ => None,
//             })
//             .collect()
//     }
//
//     pub fn events(&self) -> Vec<Rc<Event>> {
//         self.items.iter()
//             .filter_map(|item| match item {
//                 ModuleItem::Event(rc_event) => Some(rc_event.clone()),
//                 _ => None,
//             })
//             .collect()
//     }
//
//     pub fn interfaces(&self) -> Vec<Rc<Interface>> {
//         self.items.iter()
//             .filter_map(|item| match item {
//                 ModuleItem::Interface(rc_interface) => Some(rc_interface.clone()),
//                 _ => None,
//             })
//             .collect()
//     }
// }
//
// #[derive(Debug)]
// pub enum ModuleItem {
//     Error(Rc<Error>),
//     Command(Rc<Command>),
//     Event(Rc<Event>),
//     Interface(Rc<Interface>),
// }
//
// #[derive(Dbg)]
// pub struct Error {
//     #[dbg(skip)]
//     pub src: Option<Src>,
//     pub name: Rc<Identifier>,
//     pub code: Rc<IntegerLiteral>,
// }
//
// impl Error {
//     pub fn name(&self) -> Rc<str> {
//         self.name.0.clone()
//     }
//
//     pub fn code(&self) -> Rc<str> {
//         self.code.0.clone()
//     }
// }
//
// #[derive(Dbg)]
// pub struct Command {
//     #[dbg(skip)]
//     pub src: Option<Src>,
//     pub name: Rc<Identifier>,
//     pub code: Rc<IntegerLiteral>,
//     pub members: Vec<Rc<Member>>,
// }
//
// impl Command {
//     pub fn name(&self) -> Rc<str> {
//         self.name.0.clone()
//     }
//
//     pub fn code(&self) -> Rc<str> {
//         self.code.0.clone()
//     }
// }
//
// #[derive(Dbg)]
// pub struct Event {
//     #[dbg(skip)]
//     pub src: Option<Src>,
//     pub name: Rc<Identifier>,
//     pub code: Rc<IntegerLiteral>,
//     pub members: Vec<Rc<Member>>,
// }
//
// impl Event {
//     pub fn name(&self) -> Rc<str> {
//         self.name.0.clone()
//     }
//
//     pub fn code(&self) -> Rc<str> {
//         self.code.0.clone()
//     }
// }
//
// #[derive(Dbg)]
// pub struct Member {
//     #[dbg(skip)]
//     pub src: Option<Src>,
//     pub name: Rc<Identifier>,
//     pub type_: Rc<Type>,
// }
//
// impl Member {
//     pub fn name(&self) -> Rc<str> {
//         self.name.0.clone()
//     }
// }
//
// #[derive(Dbg)]
// pub struct Interface {
//     #[dbg(skip)]
//     pub src: Option<Src>,
//     pub name: Rc<Identifier>,
//     pub items: Vec<InterfaceItem>,
// }
//
// impl Interface {
//     pub fn name(&self) -> Rc<str> {
//         self.name.0.clone()
//     }
//
//     pub fn ctors(&self) -> Vec<Rc<InterfaceCtor>> {
//         self.items.iter()
//             .filter_map(|item| match item {
//                 InterfaceItem::Ctor(rc) => Some(rc.clone()),
//                 _ => None,
//             })
//             .collect()
//     }
//
//     pub fn dtors(&self) -> Vec<Rc<InterfaceDtor>> {
//         self.items.iter()
//             .filter_map(|item| match item {
//                 InterfaceItem::Dtor(rc) => Some(rc.clone()),
//                 _ => None,
//             })
//             .collect()
//     }
//
//     pub fn methods(&self) -> Vec<Rc<InterfaceMethod>> {
//         self.items.iter()
//             .filter_map(|item| match item {
//                 InterfaceItem::Method(rc) => Some(rc.clone()),
//                 _ => None,
//             })
//             .collect()
//     }
//
//     pub fn verbatim_items(&self, a: impl AsRef<str>, b: impl AsRef<str>) -> Vec<Rc<Verbatim>> {
//         self.filter_items(|item| match item {
//             InterfaceItem::Verbatim(rc) => {
//                 if rc.a.as_str() == a.as_ref() && rc.b.as_str() == b.as_ref() {
//                     Some(rc.clone())
//                 } else {
//                     None
//                 }
//             }
//             _ => None,
//         })
//     }
//
//     fn filter_items<T>(&self, f: impl Fn(&InterfaceItem) -> Option<T>) -> Vec<T> {
//         self.items.iter()
//             .filter_map(f)
//             .collect()
//     }
// }
//
// #[derive(Debug)]
// pub enum InterfaceItem {
//     Ctor(Rc<InterfaceCtor>),
//     Dtor(Rc<InterfaceDtor>),
//     Method(Rc<InterfaceMethod>),
//     Verbatim(Rc<Verbatim>),
// }
//
// #[derive(Dbg)]
// pub struct InterfaceCtor {
//     #[dbg(skip)]
//     pub src: Option<Src>,
//     pub name: Rc<Identifier>,
//     pub parameters: Vec<Rc<Parameter>>,
// }
//
// impl InterfaceCtor {
//     pub fn name(&self) -> Rc<str> {
//         self.name.0.clone()
//     }
// }
//
// #[derive(Dbg)]
// pub struct InterfaceDtor {
//     #[dbg(skip)]
//     pub src: Option<Src>,
//     pub name: Rc<Identifier>,
// }
//
// impl InterfaceDtor {
//     pub fn name(&self) -> Rc<str> {
//         self.name.0.clone()
//     }
// }
//
// #[derive(Dbg)]
// pub struct InterfaceMethod {
//     #[dbg(skip)]
//     pub src: Option<Src>,
//     pub name: Rc<Identifier>,
//     pub parameters: Vec<Rc<Parameter>>,
//     pub return_type: Option<Rc<Type>>,
// }
//
// impl InterfaceMethod {
//     pub fn name(&self) -> Rc<str> {
//         self.name.0.clone()
//     }
// }
//
// #[derive(Dbg)]
// pub struct Verbatim {
//     #[dbg(skip)]
//     pub src: Option<Src>,
//     pub a: Rc<Identifier>,
//     pub b: Rc<Identifier>,
//     pub content: Rc<VerbatimContent>,
// }
//
// impl Verbatim {
//     pub fn a(&self) -> Rc<str> {
//         self.a.0.clone()
//     }
//
//     pub fn b(&self) -> Rc<str> {
//         self.b.0.clone()
//     }
// }
//
// #[derive(Dbg)]
// pub struct Parameter {
//     #[dbg(skip)]
//     pub src: Option<Src>,
//     pub name: Rc<Identifier>,
//     pub type_: Rc<Type>,
// }
//
// impl Parameter {
//     pub fn name(&self) -> Rc<str> {
//         self.name.0.clone()
//     }
// }
//
// #[derive(Dbg)]
// pub struct VerbatimContent(pub Rc<str>, #[dbg(skip)] pub Option<Src>);
//
// impl VerbatimContent {
//     pub fn as_str(&self) -> &str {
//         self.0.as_ref()
//     }
// }
//
// #[derive(Dbg)]
// pub enum Type {
//     Ref(Rc<Type>, #[dbg(skip)] Option<Src>),
//     Mut(Rc<Type>, #[dbg(skip)] Option<Src>),
//     Slice(Rc<Type>, #[dbg(skip)] Option<Src>),
//     Array(Rc<Type>, Rc<IntegerLiteral>, #[dbg(skip)] Option<Src>),
//     CodeType(Rc<Identifier>, #[dbg(skip)] Option<Src>),
//     Qualified(Vec<Rc<Identifier>>, #[dbg(skip)] Option<Src>),
// }

#[derive(Dbg)]
pub struct Identifier(pub Rc<str>, #[dbg(skip)] pub Option<Src>);

impl Identifier {
    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }
}

#[derive(Dbg)]
pub struct IntegerLiteral(pub Rc<str>, #[dbg(skip)] pub Option<Src>);
