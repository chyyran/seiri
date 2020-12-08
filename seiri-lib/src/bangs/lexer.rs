use crate::error::{Error, Result};
use itertools::Itertools;
use itertools::multipeek;
use itertools::MultiPeek;
use std::str::Chars;

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum Token {
    /// MatchAll is produced by lexing an empty string.
    /// It can be followed only by InputEnd.
    MatchAll,

    /// BangPrefix is the '!' prepended before starting
    /// A Bang. A token stream always starts with either
    /// MatchAll, or BangPrefix, and BangPrefix is
    /// followed only by BangIdentifier.
    BangPrefix(char),

    /// BangIdentifier is the name of the bang
    /// It is always preceeded by BangPrefix,
    /// and is followed by ArgumentBegin.
    BangIdentifier(String),

    /// ArgumentBegin is the opening brace of the bang argument.
    /// It is always preceeded by BangIdentifier,
    /// and is followed by Argument.
    ArgumentBegin,

    /// ArgumentEnd is the closing brace of the bang argument.
    /// It is always preceeded by Argument.
    /// It can be followed by either ArgumentEnd,
    /// LogicalOperator, or InputEnd.
    ArgumentEnd,

    /// Argument is the string content of the bang argument.
    /// It is always between an ArgumentBegin token, and an
    /// ArgumentEnd token.
    Argument(String),

    /// LogicalOperator represents a binary operator on two bangs.
    /// Hence it is always preceeded by an ArgumentEnd token,
    /// and followed by a BangPrefix bang token.
    LogicalOperator(char),

    /// InputEnd represents the end of a the query, and is
    /// automatically appended once there are no more characters to
    /// lex. It is always preceeded by either an ArgumentEnd token, or
    /// a MatchAll token, where it forms the magic sequence
    /// [MatchAll, InputEnd] matching all tracks.
    InputEnd,

    /// Used to expand a macro into a valid series of tokens during
    /// the lexing step.
    PreprocessTokenExpand(Vec<Token>),
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum LexerMode {
    Bang,
    BangIdentifier,
    ArgumentEdge,
    Argument,
}

trait LexerProperties {
    fn is_valid_bang_identifier(&self) -> bool;
    fn is_argument_start_identifier(&self) -> bool;
}

impl LexerProperties for char {
    fn is_valid_bang_identifier(&self) -> bool {
        self.is_alphanumeric() || self == &'!'
    }
    fn is_argument_start_identifier(&self) -> bool {
        self == &'{' || self == &'`'
    }
}

fn match_bang(c: &char, characters: &mut MultiPeek<Chars>) -> Result<Option<(Token, LexerMode)>> {
    if !c.is_whitespace() {
        return match c {
            &'!' => {
                characters.next();
                Ok(Some((Token::BangPrefix(*c), LexerMode::BangIdentifier)))
            }
            _ => Err(Error::LexerUnexpectedCharacter(*c, LexerMode::Bang)),
        };
    }
    characters.next();
    Ok(None)
}

fn match_bang_identifier(
    c: &char,
    characters: &mut MultiPeek<Chars>,
) -> Result<Option<(Token, LexerMode)>> {
    if c.is_alphanumeric() {
        let token = Token::BangIdentifier(
            characters
                .take_while_ref(|&c| c.is_alphanumeric())
                .collect(),
        );
        Ok(Some((token, LexerMode::ArgumentEdge)))
    } else if c == &'!' {
        characters.next();
        Ok(Some((
            Token::BangIdentifier(String::from("!")),
            LexerMode::ArgumentEdge,
        )))
    } else {
        return Err(Error::LexerUnexpectedCharacter(
            *c,
            LexerMode::BangIdentifier,
        ));
    }
}

fn match_argument_edge(
    c: &char,
    characters: &mut MultiPeek<Chars>,
) -> Result<Option<(Token, LexerMode)>> {
    if c.is_whitespace() {
        characters.next();
        Ok(None)
    } else {
        let token = match c {
            &'|' => Some((Token::LogicalOperator('|'), LexerMode::Bang)),
            &'&' => Some((Token::LogicalOperator('&'), LexerMode::Bang)),
            &'{' => Some((Token::ArgumentBegin, LexerMode::Argument)),
            &'}' => Some((Token::ArgumentEnd, LexerMode::ArgumentEdge)),
            &'`' => Some((
                Token::PreprocessTokenExpand(vec![
                    Token::ArgumentBegin,
                    Token::Argument("true".to_owned()),
                    Token::ArgumentEnd,
                ]),
                LexerMode::Argument,
            )),
            _ => return Err(Error::LexerUnexpectedCharacter(*c, LexerMode::ArgumentEdge)),
        };
        characters.next();
        Ok(token)
    }
}

/// Searches a multipeek for the next character not equal to the specified character.
/// Advances the peek cursor.
fn next_non_match_character<F>(f: F, chars: &mut MultiPeek<Chars>) -> Result<(char, usize)>
where
    F: Fn(&char) -> bool,
{
    let mut index: usize = 0;
    while let Some(c) = chars.peek().cloned() {
        match c {
            _ if f(&c) => index += 1,
            _ => return Ok((c, index)),
        }
    }
    Err(Error::LexerUnexpectedEndOfInput)
}

fn confirm_bang_sequence(characters: &mut MultiPeek<Chars>) -> Result<bool> {
    // We found a bang!, we have to make triple sure it's a legit bang.
    // This is assuming that the current peek position is at the bang position.
    match characters.peek().cloned() {
        // We're going to see if the next token is a bang identifier followed by
        // An argument opener.
        Some(bang_ident) if bang_ident.is_valid_bang_identifier() => {
            match next_non_match_character(|&c| c.is_valid_bang_identifier(), characters) {
                Ok(next_character) if next_character.0.is_argument_start_identifier() => Ok(true),
                Ok(_) => Ok(false),
                Err(err) => Err(err),
            }
        }
        _ => Ok(false),
    }
}

fn match_argument(
    c: &char,
    characters: &mut MultiPeek<Chars>,
    tokens: &[Token],
) -> Result<Option<(Token, LexerMode)>> {
    if let &Some(ref token) = &tokens.iter().rev().nth(1) {
        match token {
            &&Token::BangIdentifier(ref token) => match token.as_ref() {
                "!" => return match_bang(c, characters),
                _ => (),
            },
            _ => (),
        }
    };

    let mut argument = String::new();
    characters.reset_peek(); // Reset the peek to right before entering this fn.
                             // Note that 'c' in this scope now is now invalid.

    while let Some(c) = characters.peek().cloned() {
        match c {
            '}' => {
                // This is a closing bracket at the end of string,
                // and we are done parsing this argument.
                // Do not consume this '}'
                characters.reset_peek(); // Reset this peek.
                return Ok(Some((
                    Token::Argument(argument.to_owned()),
                    LexerMode::ArgumentEdge,
                )));
            }
            //Support escapes as well.
            '\\' => {
                characters.next(); // Consume this '\' without adding it to the buffer.
                if let Some(escape_after) = characters.next() {
                    argument.push(escape_after);
                    characters.reset_peek();
                } else {
                    // If we try an escape at the end of the striing
                    return Err(Error::LexerUnexpectedEscapeCharacter(LexerMode::Argument));
                };
            }
            _ => {
                characters.next(); // We consume this character
                characters.reset_peek(); // Reset the peek to the next character.
                argument.push(c);
            }
        }
    }

    Ok(Some((Token::Argument(argument), LexerMode::ArgumentEdge)))
}

fn match_title(query: &str, characters: &mut MultiPeek<Chars>) -> Option<Token> {
    // We want the lexer to consider non bang openers as title peeks.
    match next_non_match_character(|&c| c == ' ', characters) {
        Ok(character) if character.0 == '!' => {
            match confirm_bang_sequence(characters) {
                Ok(not_bang) if !not_bang => {
                    // No bang found, return the title.
                    characters.reset_peek();
                    return Some(Token::PreprocessTokenExpand(vec![
                        Token::BangPrefix('!'),
                        Token::BangIdentifier("q".to_owned()),
                        Token::ArgumentBegin,
                        Token::Argument(query.to_owned()),
                        Token::ArgumentEnd,
                        Token::InputEnd,
                    ]));
                }
                // There is a valid bang here, or some other weird shit. Just continue with regular parsing.
                _ => None,
            }
        }
        // No bang found, so this is a title.
        _ => {
            characters.reset_peek();
            Some(Token::PreprocessTokenExpand(vec![
                Token::BangPrefix('!'),
                Token::BangIdentifier("q".to_owned()),
                Token::ArgumentBegin,
                Token::Argument(query.to_owned()),
                Token::ArgumentEnd,
                Token::InputEnd,
            ]))
        }
    }
}

fn ensure_arguments_balanced(tokens: &Vec<Token>) -> bool {
    let mut argument_begin = 0;
    let mut argument_end = 0;
    let mut bang_prefix = 0;
    let mut bang_ident = 0;
    for token in tokens {
        match token {
            &Token::ArgumentBegin => argument_begin += 1,
            &Token::ArgumentEnd => argument_end += 1,
            &Token::BangPrefix(_) => bang_prefix += 1,
            &Token::BangIdentifier(_) => bang_ident += 1,
            _ => (),
        }
    }

    (argument_begin == argument_end) && (bang_prefix == bang_ident)
        && (bang_prefix == argument_begin)
}
/// Lexes the given query string, and
/// returns an ordered vector of tokens.
///
/// The lexer is guaranteed to either error or
/// return a valid token stream.
///
/// A valid token stream is either [MatchAll, InputEnd],
/// or starts with [BangPrefix, BangIdentifier, ArgumentBegin, ...],
/// and ends with [..., ArgumentEnd, InputEnd].
///
/// The lexer also handles desugaring of bang-less title searches
/// and the true tick sugar ` -> {true}
pub fn lex_query(query: &str) -> Result<Vec<Token>> {
    let mut tokens = Vec::<Token>::new();
    let mut mode = LexerMode::Bang;

    // The empty query matches all tracks
    if query.chars().count() == 0 {
        tokens.push(Token::MatchAll);
        tokens.push(Token::InputEnd);
        return Ok(tokens);
    };

    let query = query.to_owned();
    let mut characters = multipeek(query.chars());

    match match_title(&query, &mut characters) {
        Some(Token::PreprocessTokenExpand(title)) => {
            tokens.extend(title.into_iter());
            return Ok(tokens);
        }
        _ => (),
    }

    characters.reset_peek();
    while let Some(c) = characters.peek().cloned() {
        let result = match mode {
            LexerMode::Bang => match_bang(&c, &mut characters),
            LexerMode::BangIdentifier => match_bang_identifier(&c, &mut characters),
            LexerMode::ArgumentEdge => match_argument_edge(&c, &mut characters),
            LexerMode::Argument => match_argument(&c, &mut characters, &tokens),
        };
        match result {
            Ok(some) => match some {
                Some(token) => {
                    mode = token.1;
                    match token.0 {
                        Token::PreprocessTokenExpand(expansion) => {
                            tokens.extend(expansion.into_iter())
                        }
                        _ => tokens.push(token.0),
                    }
                }
                None => (),
            },
            Err(err) => return Err(err),
        }
        characters.reset_peek();
    }

    if ensure_arguments_balanced(&tokens) {
        tokens.push(Token::InputEnd);
        Ok(tokens)
    } else {
        Err(Error::LexerUnexpectedEndOfInput)
    }
    //Ok(tokens)
}
