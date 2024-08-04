use crate::{
    ast::{Document, Input},
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
    pub fn read_and_parse(path: impl AsRef<Path>) -> ParseResult<Document> {
        let path = path.as_ref();

        let text = std::fs::read_to_string(path)
            .expect("failed reading file contents to string");
        let text = Rc::from(text.into_boxed_str());

        let mut input_inner = <Grammar as Parser<Rule>>::parse(Rule::Document, text.as_ref())
            .map_err(|err| ParseError::GrammarFailsToMatch(Box::new(err)))?;

        let input = Input {
            path: Some(path.as_ref().into()),
            text: Some(text),
        };

        let document = parse::generic::next(&input, &mut input_inner, parse::document)?;

        Ok(document)
    }
}
