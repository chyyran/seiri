
fn remaining(chars: &mut MultiPeek<Chars>) -> usize {
    let mut index: usize = 0;
    while let Some(_) = chars.peek().cloned() {
        index += 1;
    }
    chars.reset_peek();
    index
}
/// Counts the amount of mismatched braces current.
fn brace_counter(tokens: &[Token]) -> usize {
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


fn match_logical_after_closing(
    c: &char,
    characters: &mut MultiPeek<Chars>,
    tokens: &[Token],
    argument: &mut String,
) -> Option<Result<Option<(Token, LexerMode)>>> {
    // Need to see if the next non space character is a bang.
    // If so, take next_characters.0 - unmatched - 1 braces, then
    // and immediately to edge mode.
    match next_non_match_character(|&c| c == ' ', characters) {
        Ok(next_character) => {
            match next_character.0 {
                '!' => {
//                    println!("{:?}", next_character);
                    match match_bang_in_argument(characters, tokens) {
                        Ok(is_bang) if is_bang => {
                            characters.reset_peek();
                            let braces = brace_counter(tokens);
                            // The operator was found after
                            // n - 1 peeks (n = `next_character.1`)
                            // In other words, there are n braces between
                            // This '}' and the operator, with m braces,
                            // where m =``braces`.
                            // We will consume n - m braces.

                            if let Some(braces) = next_character.1.checked_sub(braces) {
                                for _ in 0..braces {
                                    if let Some(c) = characters.next() {
                                        characters.reset_peek();
                                        argument.push(c);
                                    } else {
                                        return Some(Err(Error::LexerUnexpectedEndOfInput));
                                    }
                                }
                                // Return the token with the proper amount of closing braces.
                                return Some(Ok(Some((
                                    Token::Argument(argument.to_owned()),
                                    LexerMode::ArgumentEdge,
                                ))));
                            } else {
                                return Some(Ok(Some((
                                    Token::Argument(argument.to_owned()),
                                    LexerMode::ArgumentEdge,
                                ))));
                            }
                        }
                        Ok(_) => {
                            // Consume this '}'
                            characters.next();
                            characters.reset_peek();
                            argument.push(*c);
                        }
                        Err(err) => return Some(Err(err)),
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
    None
}


fn match_argument_end(
    c: &char,
    characters: &mut MultiPeek<Chars>,
    tokens: &[Token],
    argument: &mut String,
) -> Option<Result<Option<(Token, LexerMode)>>> {
    // We need to determine if this bracket is a closing.
    if let Ok(bracket_after) = next_non_match_character(|&c| c == ' ', characters) {
        match bracket_after.0 {
            '|' | '&' => {
                // Need to see if the next non space character is a bang.
                // If so, drop this '}' and switch immediately to edge
                // mode.
                match next_non_match_character(|&c| c == ' ', characters) {
                    Ok(next_character) => {
                        match next_character.0 {
                            '!' => {
                                // We found a bang!
                                match match_bang_in_argument(characters, tokens) {
                                    Ok(is_bang) if is_bang => {
                                        characters.reset_peek();
                                        return Some(Ok(Some((
                                            Token::Argument(argument.to_owned()),
                                            LexerMode::ArgumentEdge,
                                        ))));
                                    }
                                    Ok(_) => {
                                        // Consume this '}'
                                        characters.next();
                                        characters.reset_peek();
                                        argument.push(*c);
                                    }
                                    Err(err) => return Some(Err(err)),
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
            '}' => {
                // Need to see if the next non '}' character is a logical operator.
                // If so, count braces and switch to edge detection once braces match.
                // Otherwise, consume this '}'
                match next_non_match_character(|&c| c == '}', characters) {
                    Ok(next_character) => {
                        match next_character.0 {
                            '|' | '&' => {
                                match match_logical_after_closing(c, characters, tokens, argument) {
                                    Some(result) => return Some(result),
                                    None => (),
                                }
                            }
                            ' ' => {
                                match next_non_match_character(|&c| c == ' ', characters) {
                                    Ok(next_character)
                                        if next_character.0 == '&' || next_character.0 == '|' =>
                                    {
                                        match match_logical_after_closing(
                                            c,
                                            characters,
                                            tokens,
                                            argument,
                                        ) {
                                            Some(result) => return Some(result),
                                            None => (),
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
