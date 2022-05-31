use crate::{errors::ErrorReport, trim};

/// Parses a number provided a string, automatically eating the string through pointers.
///
/// It is not strict, meaning it will count as a match for `890 addfsdf`, but will stop parsing
/// at the invalid character.
pub fn parse_int(code: &mut String) -> Result<i64, ErrorReport> {
    // Stores the final number
    let mut num = 0i64;
    // Stores the number length
    let mut length = 0;

    // We parse the length of the number
    for (idx, char) in (code.clone()).chars().enumerate() {
        match char {
            ' ' => {
                // We remove the spaces, only if there has been no numbers detected yet
                if length == 0 {
                    trim(code);
                    continue;
                }
                break;
            }
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                // We add to the number length
                length += 1;
            }
            // If it's none of the above
            _ => {
                if length == 0 {
                    // We return none if the first character (except whitespaces) is unexpected
                    return Err(ErrorReport::new(format!("Expected a number but got {char}"), 0, ));
                }
                // Else we just start parsing the number (exiting the loop)
                break;
            }
        }
    }

    // We loop through the string
    loop {
        // If there's no more characters we exit the loop
        if length == 0 {
            break;
        }

        // Stores the current char
        let char = code.as_bytes()[0];

        // We increment `num` by the digit with the added zeros at the end
        num += i64::from(char - 48) * (10i64.pow(length - 1));

        // We remove the parsed character
        trim(code);
        length -= 1;
    }

    return Ok(num);
}
