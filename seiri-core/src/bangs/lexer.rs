use error::{Error, Result};
use itertools::Itertools;
use itertools::multipeek;
use itertools::MultiPeek;
use std::str::Chars;

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum Token {
    BangBegin(char),
    BangIdentifier(String),
    ArgumentBegin,
    ArgumentEnd,
    Argument(String),
    LogicalOperator(char),
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum LexerMode {
    Bang,
    BangIdentifier,
    ArgumentEdge,
    Argument,
}

fn match_bang(c: &char, characters: &mut MultiPeek<Chars>) -> Result<Option<(Token, LexerMode)>> {
    if !c.is_whitespace() {
        return match c {
            &'!' => {
                characters.next();
                Ok(Some((Token::BangBegin(*c), LexerMode::BangIdentifier)))
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
            _ => return Err(Error::LexerUnexpectedCharacter(*c, LexerMode::ArgumentEdge)),
        };
        characters.next();
        Ok(token)
    }
}

/// Searches a multipeek for the next character not equal to the specified character.
/// Advances the peek cursor.
fn next_non_character(ignore: char, chars: &mut MultiPeek<Chars>) -> Result<(char, usize)> {
    let mut index: usize = 0;
    while let Some(c) = chars.peek().cloned() {
        match c {
            _ if c == ignore => index += 1,
            _ => return Ok((c, index)),
        }
    }
    Err(Error::LexerUnexpectedEndOfInput)
}

fn remaining(chars: &mut MultiPeek<Chars>) -> usize {
    let mut index: usize = 0;
    while let Some(c) = chars.peek().cloned() {
        index += 1;
    }
    chars.reset_peek();
    index
}
/// Counts the amount of mismatched braces current.
fn brace_counter(tokens: &Vec<Token>) -> usize {
    let mut counter: usize = 0;

    for token in tokens {
        match token {
            &Token::ArgumentBegin => counter += 1,
            &Token::ArgumentEnd => counter -= 1,
            _ => continue,
        };
    }
    counter
}

fn match_argument_end(
    c: &char,
    characters: &mut MultiPeek<Chars>,
    tokens: &Vec<Token>,
    argument: &mut String,
) -> Option<Result<Option<(Token, LexerMode)>>> {
    // We need to determine if this bracket is a closing.
    if let Ok(bracket_after) = next_non_character(' ', characters) {
        match bracket_after.0 {
            '|' | '&' => {
                // Need to see if the next non space character is a bang.
                // If so, drop this '}' and switch immediately to edge
                // mode.
                match next_non_character(' ', characters) {
                    Ok(next_character) => {
                        match next_character.0 {
                            '!' => {
                                // We found a bang!
                                characters.reset_peek();
                                return Some(Ok(Some((
                                    Token::Argument(argument.to_owned()),
                                    LexerMode::ArgumentEdge,
                                ))));
                            }
                            _ => {
                                // Consume this '}'
                                characters.next();
                                characters.reset_peek();
                                argument.push(*c);
                            }
                        }
                    }
                    Err(err) => return Some(Err(err)),
                };
            }
            '}' => {
                // Need to see if the next non '}' character is a logical operator.
                // If so, count braces and switch to edge detection once braces match.
                // Otherwise, consume this '}'
                match next_non_character('}', characters) {
                    Ok(next_character) => {
                        match next_character.0 {
                            '|' | '&' => {
                                // Need to see if the next non space character is a bang.
                                // If so, take next_characters.0 - unmatched - 1 braces, then
                                // and immediately to edge mode.
                                match next_non_character(' ', characters) {
                                    Ok(next_character) => {
                                        match next_character.0 {
                                            '!' => {
                                                // We found a bang!
                                                characters.reset_peek();
                                                let braces = brace_counter(tokens);

                                                // The operator was found after
                                                // n - 1 peeks (n = `next_character.1`)
                                                // In other words, there are n braces between
                                                // This '}' and the operator, with m braces,
                                                // where m =``braces`.
                                                // We will consume n - m braces.

                                                if let Some(braces) =
                                                    next_character.1.checked_sub(braces)
                                                {
                                                    for _ in 0..braces {
                                                        if let Some(c) = characters.next() {
                                                            characters.reset_peek();
                                                            argument.push(c);
                                                        } else {
                                                            return Some(Err(
                                                                Error::LexerUnexpectedEndOfInput,
                                                            ));
                                                        }
                                                    }
                                                    // Return the token with the proper amount of closing braces.
                                                    return Some(Ok(Some((
                                                        Token::Argument(argument.to_owned()),
                                                        LexerMode::ArgumentEdge,
                                                    ))));
                                                } else {
                                                    return Some(Err(
                                                        Error::LexerUnexpectedEndOfInput,
                                                    ));
                                                }
                                            }
                                            _ => {
                                                // Consume this '}'
                                                characters.next();
                                                characters.reset_peek();
                                                argument.push(*c);
                                            }
                                        }
                                    }
                                    Err(err) => return Some(Err(err)),
                                };
                            }
                            _ => {
                                // Consume this '}'
                                characters.next();
                                characters.reset_peek();
                                argument.push(*c);
                            }
                        }
                    }
                    Err(_) => {
                        // We matched the end of the string,
                        // and have to do brace counting now.

                        characters.reset_peek(); // Include this '}'
                                                 // Matching the end of string here means that
                                                 // All the characters from here to the end are braces.
                        let length_remaining = remaining(characters);

                        // Braces to keep.
                        let braces = brace_counter(tokens);

                        if let Some(braces) = length_remaining.checked_sub(braces) {
                            for _ in 0..braces {
                                if let Some(c) = characters.next() {
                                    characters.reset_peek();
                                    argument.push(c);
                                } else {
                                    return Some(Err(Error::LexerUnexpectedEndOfInput));
                                }
                            }
                            return Some(Ok(Some((
                                Token::Argument(argument.to_owned()),
                                LexerMode::ArgumentEdge,
                            ))));
                        } else {
                            return Some(Err(Error::LexerUnexpectedEndOfInput));
                        }
                    }
                };
            }
            _ => {
                // Consume this '}'
                characters.next(); // We consume this character
                characters.reset_peek(); // Reset the peek to the next character.
                argument.push(*c);
            }
        }
    } else {
        // This is a closing bracket at the end of string,
        // and we are done parsing this argument.
        // Do not consume this '}'
        characters.reset_peek(); // Reset this peek.
        return Some(Ok(Some((
            Token::Argument(argument.to_owned()),
            LexerMode::ArgumentEdge,
        ))));
    };
    None
}

fn match_argument(
    c: &char,
    characters: &mut MultiPeek<Chars>,
    tokens: &Vec<Token>,
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
            '}' => match match_argument_end(&c, characters, tokens, &mut argument) {
                Some(result) => return result,
                None => (),
            },
            // Support escapes as well.
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

pub fn lex_query(query: &str) -> Result<Vec<Token>> {
    let mut tokens = Vec::<Token>::new();
    let mut mode = LexerMode::Bang;

    let query = query.to_owned();
    let mut characters = multipeek(query.chars());

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
                    tokens.push(token.0)
                }
                None => (),
            },
            Err(err) => return Err(err),
        }
    }

    Ok(tokens)
}
