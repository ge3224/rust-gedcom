use crate::parser::{Parsable, Parser, ParsingError};
use crate::tokenizer::Token;

#[cfg(feature = "json")]
use serde::{Deserialize, Serialize};

type Xref = String;

#[derive(Debug)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub struct FamilyLink(pub Xref, pub Relation, pub Option<Pedigree>);

#[derive(Debug)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub enum Relation {
    Spouse,
    Child,
}

#[derive(Debug)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub enum Pedigree {
    Adopted,
    Birth,
    Foster,
    Sealing,
}

impl FamilyLink {
    #[must_use]
    pub fn new(xref: Xref, tag: &str) -> FamilyLink {
        let link_type = match tag {
            "FAMC" => Relation::Child,
            "FAMS" => Relation::Spouse,
            _ => panic!("Unrecognized family type tag: {}", tag),
        };
        FamilyLink(xref, link_type, None)
    }

    pub fn set_pedigree(&mut self, pedigree_text: &str) {
        self.2 = match pedigree_text.to_lowercase().as_str() {
            "adopted" => Some(Pedigree::Adopted),
            "birth" => Some(Pedigree::Birth),
            "foster" => Some(Pedigree::Foster),
            "sealing" => Some(Pedigree::Sealing),
            _ => panic!("Unrecognized family link pedigree: {}", pedigree_text),
        };
    }
}

impl Parsable<FamilyLink> for FamilyLink {
    fn parse(parser: &mut Parser, level: u8) -> Result<FamilyLink, ParsingError> {
        let tag = parser.take_tag();
        let relation = match tag {
            "FAMC" => Relation::Child,
            "FAMS" => Relation::Spouse,
            _ => panic!("Unrecognized family type tag: {}", tag),
        };
        let mut link = FamilyLink(parser.take_line_value(), relation, None);

        loop {
            if let Token::Level(cur_level) = parser.tokenizer.current_token {
                if cur_level <= level {
                    break;
                }
            }
            match &parser.tokenizer.current_token {
                Token::Tag(tag) => match tag.as_str() {
                    "PEDI" => link.set_pedigree(parser.take_line_value().as_str()),
                    _ => parser.skip_current_tag(level + 1, "FamilyLink"),
                },
                Token::Level(_) => parser.tokenizer.next_token(),
                _ => parser.handle_unexpected_token(level + 1, "FamilyLink"),
            }
        }

        Ok(link)
    }
}
