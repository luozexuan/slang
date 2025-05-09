// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

// This file is generated; we can't reasonably satisfy some of these lints.
#![allow(
    clippy::if_not_else,
    clippy::too_many_lines,
    clippy::unused_self,
    clippy::struct_excessive_bools,
    clippy::similar_names,
    unused_imports
)]

use semver::Version;

use crate::cst;
use crate::cst::{
    EdgeLabel, IsLexicalContext, LexicalContext, LexicalContextType, NonterminalKind, TerminalKind,
};
use crate::parser::lexer::{KeywordScan, Lexer, ScannedTerminal};
use crate::parser::parser_support::{
    ChoiceHelper, OneOrMoreHelper, OptionalHelper, ParserContext, ParserFunction, ParserResult,
    PrecedenceHelper, SeparatedHelper, SequenceHelper, ZeroOrMoreHelper,
};
use crate::parser::scanner_macros::{
    scan_char_range, scan_chars, scan_choice, scan_keyword_choice, scan_none_of,
    scan_not_followed_by, scan_one_or_more, scan_optional, scan_sequence, scan_zero_or_more,
};
use crate::parser::ParseOutput;
use crate::utils::LanguageFacts;

#[derive(Debug)]
pub struct Parser {
    #[allow(dead_code)]
    pub(crate) version_is_at_least_1_0_0: bool,
    language_version: Version,
}

#[derive(thiserror::Error, Debug)]
pub enum ParserInitializationError {
    #[error("Unsupported language version '{0}'.")]
    UnsupportedLanguageVersion(Version),
}

impl Parser {
    pub fn create(
        language_version: Version,
    ) -> std::result::Result<Self, ParserInitializationError> {
        if LanguageFacts::ALL_VERSIONS
            .binary_search(&language_version)
            .is_ok()
        {
            Ok(Self {
                version_is_at_least_1_0_0: Version::new(1, 0, 0) <= language_version,
                language_version,
            })
        } else {
            Err(ParserInitializationError::UnsupportedLanguageVersion(
                language_version,
            ))
        }
    }

    pub fn language_version(&self) -> &Version {
        &self.language_version
    }

    pub fn parse_file_contents(&self, input: &str) -> ParseOutput {
        self.parse_nonterminal(NonterminalKind::SourceUnit, input)
    }
    pub fn parse_nonterminal(&self, kind: NonterminalKind, input: &str) -> ParseOutput {
        match kind {
            NonterminalKind::AdditionExpression => {
                Self::addition_expression.parse(self, input, kind)
            }
            NonterminalKind::Expression => Self::expression.parse(self, input, kind),
            NonterminalKind::Literal => Self::literal.parse(self, input, kind),
            NonterminalKind::MemberAccessExpression => {
                Self::member_access_expression.parse(self, input, kind)
            }
            NonterminalKind::NegationExpression => {
                Self::negation_expression.parse(self, input, kind)
            }
            NonterminalKind::SeparatedIdentifiers => {
                Self::separated_identifiers.parse(self, input, kind)
            }
            NonterminalKind::SourceUnit => Self::source_unit.parse(self, input, kind),
            NonterminalKind::SourceUnitMember => Self::source_unit_member.parse(self, input, kind),
            NonterminalKind::SourceUnitMembers => {
                Self::source_unit_members.parse(self, input, kind)
            }
            NonterminalKind::Tree => Self::tree.parse(self, input, kind),
            NonterminalKind::TreeNode => Self::tree_node.parse(self, input, kind),
            NonterminalKind::TreeNodeChild => Self::tree_node_child.parse(self, input, kind),
            NonterminalKind::TreeNodeChildren => Self::tree_node_children.parse(self, input, kind),
        }
    }

    /********************************************
     *         Parser Functions
     ********************************************/

    #[allow(unused_assignments, unused_parens)]
    fn addition_expression(&self, input: &mut ParserContext<'_>) -> ParserResult {
        let result = self.expression(input);
        let ParserResult::Match(r#match) = &result else {
            return result;
        };
        match &r#match.nodes[..] {
            [cst::Edge {
                node: cst::Node::Nonterminal(node),
                ..
            }] if node.kind == NonterminalKind::Expression => match &node.children[..] {
                [inner @ cst::Edge {
                    node: cst::Node::Nonterminal(node),
                    ..
                }] if node.kind == NonterminalKind::AdditionExpression => {
                    ParserResult::r#match(vec![inner.clone()], r#match.expected_terminals.clone())
                }
                _ => ParserResult::default(),
            },
            _ => ParserResult::default(),
        }
    }

    #[allow(unused_assignments, unused_parens)]
    fn expression(&self, input: &mut ParserContext<'_>) -> ParserResult {
        let parse_left_addition_expression = |input: &mut ParserContext<'_>| {
            PrecedenceHelper::to_binary_operator(
                NonterminalKind::AdditionExpression,
                1u8,
                1u8 + 1,
                self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                    input,
                    TerminalKind::Plus,
                )
                .with_label(EdgeLabel::Operator),
            )
        };
        let parse_prefix_negation_expression = |input: &mut ParserContext<'_>| {
            PrecedenceHelper::to_prefix_operator(
                NonterminalKind::NegationExpression,
                3u8,
                self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                    input,
                    TerminalKind::Bang,
                )
                .with_label(EdgeLabel::Operator),
            )
        };
        let parse_postfix_member_access_expression = |input: &mut ParserContext<'_>| {
            PrecedenceHelper::to_postfix_operator(
                NonterminalKind::MemberAccessExpression,
                5u8,
                SequenceHelper::run(|mut seq| {
                    seq.elem_labeled(
                        EdgeLabel::Period,
                        self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                            input,
                            TerminalKind::Period,
                        ),
                    )?;
                    seq.elem_labeled(
                        EdgeLabel::Member,
                        self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                            input,
                            TerminalKind::Identifier,
                        ),
                    )?;
                    seq.finish()
                }),
            )
        };
        let prefix_operator_parser = |input: &mut ParserContext<'_>| {
            ChoiceHelper::run(input, |mut choice, input| {
                let result = parse_prefix_negation_expression(input);
                choice.consider(input, result)?;
                choice.finish(input)
            })
        };
        let primary_expression_parser = |input: &mut ParserContext<'_>| {
            ChoiceHelper::run(input, |mut choice, input| {
                let result = self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                    input,
                    TerminalKind::StringLiteral,
                );
                choice.consider(input, result)?;
                let result = self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                    input,
                    TerminalKind::Identifier,
                );
                choice.consider(input, result)?;
                choice.finish(input)
            })
            .with_label(EdgeLabel::Variant)
        };
        let postfix_operator_parser = |input: &mut ParserContext<'_>| {
            ChoiceHelper::run(input, |mut choice, input| {
                let result = parse_postfix_member_access_expression(input);
                choice.consider(input, result)?;
                choice.finish(input)
            })
        };
        let binary_operand_parser = |input: &mut ParserContext<'_>| {
            SequenceHelper::run(|mut seq| {
                seq.elem(ZeroOrMoreHelper::run(input, prefix_operator_parser))?;
                seq.elem(primary_expression_parser(input))?;
                seq.elem(ZeroOrMoreHelper::run(input, postfix_operator_parser))?;
                seq.finish()
            })
        };
        let binary_operator_parser = |input: &mut ParserContext<'_>| {
            ChoiceHelper::run(input, |mut choice, input| {
                let result = parse_left_addition_expression(input);
                choice.consider(input, result)?;
                choice.finish(input)
            })
        };
        let linear_expression_parser = |input: &mut ParserContext<'_>| {
            SequenceHelper::run(|mut seq| {
                seq.elem(binary_operand_parser(input))?;
                seq.elem(ZeroOrMoreHelper::run(input, |input| {
                    SequenceHelper::run(|mut seq| {
                        seq.elem(binary_operator_parser(input))?;
                        seq.elem(binary_operand_parser(input))?;
                        seq.finish()
                    })
                }))?;
                seq.finish()
            })
        };
        PrecedenceHelper::reduce_precedence_result(
            NonterminalKind::Expression,
            linear_expression_parser(input),
        )
        .with_kind(NonterminalKind::Expression)
    }

    #[allow(unused_assignments, unused_parens)]
    fn literal(&self, input: &mut ParserContext<'_>) -> ParserResult {
        ChoiceHelper::run(input, |mut choice, input| {
            let result = self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                input,
                TerminalKind::StringLiteral,
            );
            choice.consider(input, result)?;
            choice.finish(input)
        })
        .with_label(EdgeLabel::Variant)
        .with_kind(NonterminalKind::Literal)
    }

    #[allow(unused_assignments, unused_parens)]
    fn member_access_expression(&self, input: &mut ParserContext<'_>) -> ParserResult {
        let result = self.expression(input);
        let ParserResult::Match(r#match) = &result else {
            return result;
        };
        match &r#match.nodes[..] {
            [cst::Edge {
                node: cst::Node::Nonterminal(node),
                ..
            }] if node.kind == NonterminalKind::Expression => match &node.children[..] {
                [inner @ cst::Edge {
                    node: cst::Node::Nonterminal(node),
                    ..
                }] if node.kind == NonterminalKind::MemberAccessExpression => {
                    ParserResult::r#match(vec![inner.clone()], r#match.expected_terminals.clone())
                }
                _ => ParserResult::default(),
            },
            _ => ParserResult::default(),
        }
    }

    #[allow(unused_assignments, unused_parens)]
    fn negation_expression(&self, input: &mut ParserContext<'_>) -> ParserResult {
        let result = self.expression(input);
        let ParserResult::Match(r#match) = &result else {
            return result;
        };
        match &r#match.nodes[..] {
            [cst::Edge {
                node: cst::Node::Nonterminal(node),
                ..
            }] if node.kind == NonterminalKind::Expression => match &node.children[..] {
                [inner @ cst::Edge {
                    node: cst::Node::Nonterminal(node),
                    ..
                }] if node.kind == NonterminalKind::NegationExpression => {
                    ParserResult::r#match(vec![inner.clone()], r#match.expected_terminals.clone())
                }
                _ => ParserResult::default(),
            },
            _ => ParserResult::default(),
        }
    }

    #[allow(unused_assignments, unused_parens)]
    fn separated_identifiers(&self, input: &mut ParserContext<'_>) -> ParserResult {
        if self.version_is_at_least_1_0_0 {
            SeparatedHelper::run::<_, LexicalContextType::Default>(
                input,
                self,
                |input| {
                    self.parse_terminal_with_trivia::<LexicalContextType::Default>(
                        input,
                        TerminalKind::Identifier,
                    )
                    .with_label(EdgeLabel::Item)
                },
                TerminalKind::Period,
                EdgeLabel::Separator,
            )
        } else {
            ParserResult::no_match(vec![])
        }
        .with_kind(NonterminalKind::SeparatedIdentifiers)
    }

    #[allow(unused_assignments, unused_parens)]
    fn source_unit(&self, input: &mut ParserContext<'_>) -> ParserResult {
        self.source_unit_members(input)
            .with_label(EdgeLabel::Members)
            .with_kind(NonterminalKind::SourceUnit)
    }

    #[allow(unused_assignments, unused_parens)]
    fn source_unit_member(&self, input: &mut ParserContext<'_>) -> ParserResult {
        ChoiceHelper::run(input, |mut choice, input| {
            let result = self.tree(input);
            choice.consider(input, result)?;
            let result = self.expression(input);
            choice.consider(input, result)?;
            let result = self.separated_identifiers(input);
            choice.consider(input, result)?;
            let result = self.literal(input);
            choice.consider(input, result)?;
            choice.finish(input)
        })
        .with_label(EdgeLabel::Variant)
        .with_kind(NonterminalKind::SourceUnitMember)
    }

    #[allow(unused_assignments, unused_parens)]
    fn source_unit_members(&self, input: &mut ParserContext<'_>) -> ParserResult {
        OneOrMoreHelper::run(input, |input| {
            self.source_unit_member(input).with_label(EdgeLabel::Item)
        })
        .with_kind(NonterminalKind::SourceUnitMembers)
    }

    #[allow(unused_assignments, unused_parens)]
    fn tree(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            match SequenceHelper::run(|mut seq| {
                seq.elem_labeled(
                    EdgeLabel::Keyword,
                    self.parse_terminal_with_trivia::<LexicalContextType::Tree>(
                        input,
                        TerminalKind::TreeKeyword,
                    ),
                )?;
                seq.elem_labeled(
                    EdgeLabel::Name,
                    OptionalHelper::transform(
                        self.parse_terminal_with_trivia::<LexicalContextType::Tree>(
                            input,
                            TerminalKind::Identifier,
                        ),
                    ),
                )?;
                seq.elem_labeled(EdgeLabel::Node, self.tree_node(input))?;
                seq.finish()
            }) {
                result if result.matches_at_least_n_terminals(1u8) => {
                    seq.elem(
                        result.recover_until_with_nested_delims::<_, LexicalContextType::Tree>(
                            input,
                            self,
                            TerminalKind::Semicolon,
                        ),
                    )?;
                }
                result => {
                    seq.elem(result)?;
                }
            }
            seq.elem_labeled(
                EdgeLabel::Semicolon,
                self.parse_terminal_with_trivia::<LexicalContextType::Tree>(
                    input,
                    TerminalKind::Semicolon,
                ),
            )?;
            seq.finish()
        })
        .with_kind(NonterminalKind::Tree)
    }

    #[allow(unused_assignments, unused_parens)]
    fn tree_node(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            let mut delim_guard = input.open_delim(TerminalKind::CloseBracket);
            let input = delim_guard.ctx();
            seq.elem_labeled(
                EdgeLabel::OpenBracket,
                self.parse_terminal_with_trivia::<LexicalContextType::Tree>(
                    input,
                    TerminalKind::OpenBracket,
                ),
            )?;
            match self
                .tree_node_children(input)
                .with_label(EdgeLabel::Members)
            {
                result if result.matches_at_least_n_terminals(0u8) => {
                    seq.elem(
                        result.recover_until_with_nested_delims::<_, LexicalContextType::Tree>(
                            input,
                            self,
                            TerminalKind::CloseBracket,
                        ),
                    )?;
                }
                _ => {
                    return std::ops::ControlFlow::Break(ParserResult::no_match(vec![]));
                }
            }
            seq.elem_labeled(
                EdgeLabel::CloseBracket,
                self.parse_terminal_with_trivia::<LexicalContextType::Tree>(
                    input,
                    TerminalKind::CloseBracket,
                ),
            )?;
            seq.finish()
        })
        .with_kind(NonterminalKind::TreeNode)
    }

    #[allow(unused_assignments, unused_parens)]
    fn tree_node_child(&self, input: &mut ParserContext<'_>) -> ParserResult {
        ChoiceHelper::run(input, |mut choice, input| {
            let result = self.tree_node(input);
            choice.consider(input, result)?;
            let result = self.parse_terminal_with_trivia::<LexicalContextType::Tree>(
                input,
                TerminalKind::DelimitedIdentifier,
            );
            choice.consider(input, result)?;
            choice.finish(input)
        })
        .with_label(EdgeLabel::Variant)
        .with_kind(NonterminalKind::TreeNodeChild)
    }

    #[allow(unused_assignments, unused_parens)]
    fn tree_node_children(&self, input: &mut ParserContext<'_>) -> ParserResult {
        OneOrMoreHelper::run(input, |input| {
            self.tree_node_child(input).with_label(EdgeLabel::Item)
        })
        .with_kind(NonterminalKind::TreeNodeChildren)
    }

    #[allow(unused_assignments, unused_parens)]
    fn leading_trivia(&self, input: &mut ParserContext<'_>) -> ParserResult {
        OneOrMoreHelper::run(input, |input| {
            ChoiceHelper::run(input, |mut choice, input| {
                let result = self
                    .parse_terminal::<LexicalContextType::Default>(input, TerminalKind::Whitespace)
                    .with_label(EdgeLabel::LeadingTrivia);
                choice.consider(input, result)?;
                let result = self
                    .parse_terminal::<LexicalContextType::Default>(input, TerminalKind::EndOfLine)
                    .with_label(EdgeLabel::LeadingTrivia);
                choice.consider(input, result)?;
                let result = self
                    .parse_terminal::<LexicalContextType::Default>(
                        input,
                        TerminalKind::SingleLineComment,
                    )
                    .with_label(EdgeLabel::LeadingTrivia);
                choice.consider(input, result)?;
                let result = self
                    .parse_terminal::<LexicalContextType::Default>(
                        input,
                        TerminalKind::MultiLineComment,
                    )
                    .with_label(EdgeLabel::LeadingTrivia);
                choice.consider(input, result)?;
                choice.finish(input)
            })
        })
    }

    #[allow(unused_assignments, unused_parens)]
    fn trailing_trivia(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem(OptionalHelper::transform(
                self.parse_terminal::<LexicalContextType::Default>(input, TerminalKind::Whitespace)
                    .with_label(EdgeLabel::TrailingTrivia),
            ))?;
            seq.elem(OptionalHelper::transform(
                self.parse_terminal::<LexicalContextType::Default>(
                    input,
                    TerminalKind::SingleLineComment,
                )
                .with_label(EdgeLabel::TrailingTrivia),
            ))?;
            seq.elem(
                self.parse_terminal::<LexicalContextType::Default>(input, TerminalKind::EndOfLine)
                    .with_label(EdgeLabel::TrailingTrivia),
            )?;
            seq.finish()
        })
    }

    /********************************************
     *         Scanner Functions
     ********************************************/

    #[allow(unused_assignments, unused_parens)]
    fn ascii_escape(&self, input: &mut ParserContext<'_>) -> bool {
        scan_choice!(
            input,
            scan_chars!(input, 't'),
            scan_chars!(input, 'r'),
            scan_chars!(input, 'n'),
            scan_chars!(input, '\\'),
            scan_chars!(input, '\''),
            scan_chars!(input, '"'),
            scan_chars!(input, '\r'),
            scan_chars!(input, '\n')
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn delimited_identifier(&self, input: &mut ParserContext<'_>) -> bool {
        scan_sequence!(
            self.delimited_identifier_start(input),
            scan_zero_or_more!(input, self.delimited_identifier_part(input))
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn delimited_identifier_part(&self, input: &mut ParserContext<'_>) -> bool {
        scan_choice!(
            input,
            scan_chars!(input, '_'),
            scan_char_range!(input, 'a'..='z'),
            scan_char_range!(input, '0'..='9')
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn delimited_identifier_start(&self, input: &mut ParserContext<'_>) -> bool {
        scan_choice!(input, scan_char_range!(input, 'A'..='Z'))
    }

    #[allow(unused_assignments, unused_parens)]
    fn end_of_line(&self, input: &mut ParserContext<'_>) -> bool {
        scan_sequence!(
            scan_optional!(input, scan_chars!(input, '\r')),
            scan_chars!(input, '\n')
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn escape_sequence(&self, input: &mut ParserContext<'_>) -> bool {
        scan_sequence!(
            scan_chars!(input, '\\'),
            scan_choice!(
                input,
                self.ascii_escape(input),
                self.hex_byte_escape(input),
                self.unicode_escape(input)
            )
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn hex_byte_escape(&self, input: &mut ParserContext<'_>) -> bool {
        scan_sequence!(
            scan_chars!(input, 'x'),
            self.hex_character(input),
            self.hex_character(input)
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn hex_character(&self, input: &mut ParserContext<'_>) -> bool {
        scan_choice!(
            input,
            scan_char_range!(input, '0'..='9'),
            scan_char_range!(input, 'a'..='f'),
            scan_char_range!(input, 'A'..='F')
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn identifier(&self, input: &mut ParserContext<'_>) -> bool {
        self.raw_identifier(input)
    }

    #[allow(unused_assignments, unused_parens)]
    fn identifier_part(&self, input: &mut ParserContext<'_>) -> bool {
        scan_choice!(
            input,
            self.identifier_start(input),
            scan_char_range!(input, '0'..='9')
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn identifier_start(&self, input: &mut ParserContext<'_>) -> bool {
        scan_choice!(
            input,
            scan_chars!(input, '_'),
            scan_chars!(input, '$'),
            scan_char_range!(input, 'a'..='z'),
            scan_char_range!(input, 'A'..='Z')
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn multi_line_comment(&self, input: &mut ParserContext<'_>) -> bool {
        scan_sequence!(
            scan_not_followed_by!(input, scan_chars!(input, '/', '*'), scan_chars!(input, '*')),
            scan_zero_or_more!(
                input,
                scan_choice!(
                    input,
                    scan_none_of!(input, '*'),
                    scan_not_followed_by!(input, scan_chars!(input, '*'), scan_chars!(input, '/'))
                )
            ),
            scan_chars!(input, '*', '/')
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn raw_identifier(&self, input: &mut ParserContext<'_>) -> bool {
        scan_sequence!(
            self.identifier_start(input),
            scan_zero_or_more!(input, self.identifier_part(input))
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn single_line_comment(&self, input: &mut ParserContext<'_>) -> bool {
        scan_sequence!(
            scan_not_followed_by!(input, scan_chars!(input, '/', '/'), scan_chars!(input, '/')),
            scan_zero_or_more!(input, scan_none_of!(input, '\r', '\n'))
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn string_literal(&self, input: &mut ParserContext<'_>) -> bool {
        scan_sequence!(
            scan_chars!(input, '"'),
            scan_zero_or_more!(
                input,
                scan_choice!(
                    input,
                    self.escape_sequence(input),
                    scan_none_of!(input, '"', '\\', '\r', '\n')
                )
            ),
            scan_chars!(input, '"')
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn unicode_escape(&self, input: &mut ParserContext<'_>) -> bool {
        scan_sequence!(
            scan_chars!(input, 'u'),
            self.hex_character(input),
            self.hex_character(input),
            self.hex_character(input),
            self.hex_character(input)
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn whitespace(&self, input: &mut ParserContext<'_>) -> bool {
        scan_one_or_more!(
            input,
            scan_choice!(input, scan_chars!(input, ' '), scan_chars!(input, '\t'))
        )
    }
}

impl Lexer for Parser {
    fn leading_trivia(&self, input: &mut ParserContext<'_>) -> ParserResult {
        input.cached_leading_trivia_or(|input| Parser::leading_trivia(self, input))
    }

    fn trailing_trivia(&self, input: &mut ParserContext<'_>) -> ParserResult {
        Parser::trailing_trivia(self, input)
    }

    fn delimiters<LexCtx: IsLexicalContext>() -> &'static [(TerminalKind, TerminalKind)] {
        match LexCtx::value() {
            LexicalContext::Default => &[],
            LexicalContext::Tree => &[(TerminalKind::OpenBracket, TerminalKind::CloseBracket)],
        }
    }

    fn next_terminal<LexCtx: IsLexicalContext>(
        &self,
        input: &mut ParserContext<'_>,
    ) -> Option<ScannedTerminal> {
        let save = input.position();
        let mut furthest_position = input.position();
        let mut longest_terminal = None;

        macro_rules! longest_match {
                ($( { $kind:ident = $function:ident } )*) => {
                    $(
                        if self.$function(input) && input.position() > furthest_position {
                            furthest_position = input.position();

                            longest_terminal = Some(TerminalKind::$kind);
                        }
                        input.set_position(save);
                    )*
                };
            }

        match LexCtx::value() {
            LexicalContext::Default => {
                if let Some(kind) = match input.next() {
                    Some('!') => Some(TerminalKind::Bang),
                    Some('+') => Some(TerminalKind::Plus),
                    Some('.') => Some(TerminalKind::Period),
                    Some(_) => {
                        input.undo();
                        None
                    }
                    None => None,
                } {
                    furthest_position = input.position();
                    longest_terminal = Some(kind);
                }
                input.set_position(save);

                longest_match! {
                    { EndOfLine = end_of_line }
                    { Identifier = identifier }
                    { MultiLineComment = multi_line_comment }
                    { SingleLineComment = single_line_comment }
                    { StringLiteral = string_literal }
                    { Whitespace = whitespace }
                }
                // Make sure promotable identifiers are last so they don't grab other things
                longest_match! {}

                // We have an identifier; we need to check if it's a keyword
                if let Some(identifier) = longest_terminal.filter(|tok| [].contains(tok)) {
                    let kw_scan = KeywordScan::Absent;
                    let kw_scan = match kw_scan {
                        // Strict prefix; we need to match the whole identifier to promote
                        _ if input.position() < furthest_position => KeywordScan::Absent,
                        value => value,
                    };

                    input.set_position(furthest_position);
                    return Some(ScannedTerminal::IdentifierOrKeyword {
                        identifier,
                        kw: kw_scan,
                    });
                }
            }
            LexicalContext::Tree => {
                if let Some(kind) = match input.next() {
                    Some(';') => Some(TerminalKind::Semicolon),
                    Some('[') => Some(TerminalKind::OpenBracket),
                    Some(']') => Some(TerminalKind::CloseBracket),
                    Some(_) => {
                        input.undo();
                        None
                    }
                    None => None,
                } {
                    furthest_position = input.position();
                    longest_terminal = Some(kind);
                }
                input.set_position(save);

                longest_match! {
                    { DelimitedIdentifier = delimited_identifier }
                }
                // Make sure promotable identifiers are last so they don't grab other things
                longest_match! {
                    { Identifier = identifier }
                }

                // We have an identifier; we need to check if it's a keyword
                if let Some(identifier) =
                    longest_terminal.filter(|tok| [TerminalKind::Identifier].contains(tok))
                {
                    let kw_scan = if scan_chars!(input, 't', 'r', 'e', 'e') {
                        KeywordScan::Reserved(TerminalKind::TreeKeyword)
                    } else {
                        KeywordScan::Absent
                    };
                    let kw_scan = match kw_scan {
                        // Strict prefix; we need to match the whole identifier to promote
                        _ if input.position() < furthest_position => KeywordScan::Absent,
                        value => value,
                    };

                    input.set_position(furthest_position);
                    return Some(ScannedTerminal::IdentifierOrKeyword {
                        identifier,
                        kw: kw_scan,
                    });
                }
            }
        }

        match longest_terminal {
            Some(terminal) => {
                input.set_position(furthest_position);
                Some(ScannedTerminal::Single(terminal))
            }
            // Skip a character if possible and if we didn't recognize a terminal
            None if input.peek().is_some() => {
                let _ = input.next();
                Some(ScannedTerminal::Single(TerminalKind::UNRECOGNIZED))
            }
            None => None,
        }
    }
}
