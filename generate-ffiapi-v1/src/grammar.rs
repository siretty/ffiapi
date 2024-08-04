use crate::{
    ast::{Ast, Document, Input, Src},
    parse::{ParseError, ParseResult, self},
};
use pest_derive::Parser;
use std::{
    path::Path,
    rc::Rc,
};
use pest::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct Grammar;

impl Grammar {
    pub fn read_and_parse(path: impl AsRef<Path>) -> ParseResult<Ast<Document>> {
        let path = path.as_ref();

        let text = std::fs::read_to_string(path)
            .expect("failed reading file contents to string");
        let text = Rc::from(text.into_boxed_str());

        let mut ast_document = Self::parse_text(text)?;
        if let Some(src) = ast_document.1.as_mut() {
            let document_path = src.input.path.clone();
            src.input.path = document_path.or(Some(Rc::new(path.to_path_buf())));
        }

        Ok(ast_document)
    }

    pub fn parse_text(text: Rc<str>) -> ParseResult<Ast<Document>> {
        let mut input_inner = <Grammar as Parser<Rule>>::parse(Rule::Document, text.as_ref())
            .map_err(|err| ParseError::GrammarFailsToMatch(Box::new(err)))?;

        let input = Input {
            path: None,
            text: Some(text.clone()),
        };

        let ast_document = parse::generic::next(&input, &mut input_inner, parse::document)?;

        Ok(ast_document)
    }
}
