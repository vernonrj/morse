/// Morse Code Translations
/// 

use std::error::Error;
use std::time::Duration;

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum MorseBit {
    /// Dit, or Dot, or (.) is the short signal duration
    Dit,
    /// Dah, or Dash, or (-) is the long signal duration, equal to 3 dits
    Dah,
    /// letter space, equal to the duration of 3 dits
    LetterSpace,
    /// word space, equal to the duration of 7 dits
    WordSpace,
}

pub fn to_pretty(morse_tokens: impl IntoIterator<Item=MorseBit>) -> String {
    use MorseBit::*;
    let mut output = String::new();
    let mut last_tok = WordSpace;
    for tok in morse_tokens {
        match tok {
            Dit => output.push('.'),
            Dah => output.push('-'),
            LetterSpace if last_tok != LetterSpace && last_tok != WordSpace => output.push(' '),
            WordSpace if last_tok != LetterSpace && last_tok != WordSpace => output.push_str("  "),
            LetterSpace | WordSpace => (),
        }
        last_tok = tok;
    }
    output
}

pub fn to_durations(morse_tokens: impl IntoIterator<Item=MorseBit>) -> Vec<(bool, Duration)> {
    let mut durations: Vec<(bool, Duration)> = Vec::new();
    for token in morse_tokens {
        match token {
            MorseBit::Dit => {
                durations.push((true, Duration::new(1, 0)));
                durations.push((false, Duration::new(1, 0)));
            }
            MorseBit::Dah => {
                durations.push((true, Duration::new(3, 0)));
                durations.push((false, Duration::new(1, 0)));
            }
            MorseBit::LetterSpace => {
                durations.push((false, Duration::new(3, 0)));
            }
            MorseBit::WordSpace => {
                durations.push((false, Duration::new(7, 0)));
            }
        };
    }

    durations
}

pub fn from_pretty(text: &str) -> Result<Vec<MorseBit>, Box<dyn Error>> {
    let tokens: Result<Vec<MorseBit>, Box<dyn Error>> = text.chars()
        .map(|ch| match ch {
            '-' => Ok(MorseBit::Dah),
            '.' => Ok(MorseBit::Dit),
            ' ' => Ok(MorseBit::LetterSpace),
            _ => Err(format!("can't convert `{ch}`").into()),
        }).collect();
    tokens
}

pub fn encode(message: &str) -> Result<Vec<MorseBit>, Box<dyn Error>> {
    let mut output: Vec<MorseBit> = Vec::new();
    for chr in message.to_lowercase().chars() {
        use MorseBit::*;
        match chr {
            ' ' | '\t' | '\r' | '\n' => output.extend(&[WordSpace]),
            'a' => output.extend(&[Dit, Dah]),
            'b' => output.extend(&[Dah, Dit, Dit, Dit]),
            'c' => output.extend(&[Dah, Dit, Dah, Dit]),
            'd' => output.extend(&[Dah, Dit, Dit]),
            'e' => output.extend(&[Dit]),
            'f' => output.extend(&[Dit, Dit, Dah, Dit]),
            'g' => output.extend(&[Dah, Dah, Dit]),
            'h' => output.extend(&[Dit, Dit, Dit, Dit]),
            'i' => output.extend(&[Dit, Dit]),
            'j' => output.extend(&[Dit, Dah, Dah, Dah]),
            'k' => output.extend(&[Dah, Dit, Dah]),
            'l' => output.extend(&[Dit, Dah, Dit, Dit]),
            'm' => output.extend(&[Dah, Dah]),
            'n' => output.extend(&[Dah, Dit]),
            'o' => output.extend(&[Dah, Dah, Dah]),
            'p' => output.extend(&[Dit, Dah, Dah, Dit]),
            'q' => output.extend(&[Dah, Dah, Dit, Dah]),
            'r' => output.extend(&[Dit, Dah, Dit]),
            's' => output.extend(&[Dit, Dit, Dit]),
            't' => output.extend(&[Dah]),
            'u' => output.extend(&[Dit, Dit, Dah]),
            'v' => output.extend(&[Dit, Dit, Dit, Dah]),
            'w' => output.extend(&[Dit, Dah, Dah]),
            'x' => output.extend(&[Dah, Dit, Dit, Dah]),
            'y' => output.extend(&[Dah, Dit, Dah, Dah]),
            'z' => output.extend(&[Dah ,Dah, Dit, Dit]),
            '1' => output.extend(&[Dit, Dah, Dah, Dah, Dah]),
            '2' => output.extend(&[Dit, Dit, Dah, Dah, Dah]),
            '3' => output.extend(&[Dit, Dit, Dit, Dah, Dah]),
            '4' => output.extend(&[Dit, Dit, Dit, Dit, Dah]),
            '5' => output.extend(&[Dit, Dit, Dit, Dit, Dit]),
            '6' => output.extend(&[Dah, Dit, Dit, Dit, Dit]),
            '7' => output.extend(&[Dah, Dah, Dit, Dit, Dit]),
            '8' => output.extend(&[Dah, Dah, Dah, Dit, Dit]),
            '9' => output.extend(&[Dah, Dah, Dah, Dah, Dit]),
            '0' => output.extend(&[Dah, Dah, Dah, Dah, Dah]),
            _ => return Err(format!("can't morsify character `{chr}`").into()),
        };
        output.push(LetterSpace);
    }
    Ok(output)
}

pub fn decode(mut stream: &[MorseBit]) -> Result<String, Box<dyn Error>> {
    let mut output: String = String::new();
    while ! stream.is_empty() {
        // println!("stream = {stream:?}");
        let (next_chunk, new_stream) = get_next_morse(stream);
        stream = new_stream;
        // println!("next chunk = {next_chunk:?}");
        use MorseBit::*;
        let ch = match next_chunk {
            &[WordSpace] => ' ',
            &[Dit, Dah] => 'a',
            &[Dah, Dit, Dit, Dit] => 'b',
            &[Dah, Dit, Dah, Dit] => 'c',
            &[Dah, Dit, Dit] => 'd',
            &[Dit] => 'e',
            &[Dit, Dit, Dah, Dit] => 'f',
            &[Dah, Dah, Dit] => 'g',
            &[Dit, Dit, Dit, Dit] => 'h',
            &[Dit, Dit] => 'i',
            &[Dit, Dah, Dah, Dah] => 'j',
            &[Dah, Dit, Dah] => 'k',
            &[Dit, Dah, Dit, Dit] => 'l',
            &[Dah, Dah] => 'm',
            &[Dah, Dit] => 'n',
            &[Dah, Dah, Dah] => 'o',
            &[Dit, Dah, Dah, Dit] => 'p',
            &[Dah, Dah, Dit, Dah] => 'q',
            &[Dit, Dah, Dit] => 'r',
            &[Dit, Dit, Dit] => 's',
            &[Dah] => 't',
            &[Dit, Dit, Dah] => 'u',
            &[Dit, Dit, Dit, Dah] => 'v',
            &[Dit, Dah, Dah] => 'w',
            &[Dah, Dit, Dit, Dah] => 'x',
            &[Dah, Dit, Dah, Dah] => 'y',
            &[Dah, Dah, Dit, Dit] => 'z',
            &[Dit, Dah, Dah, Dah, Dah] => '1',
            &[Dit, Dit, Dah, Dah, Dah] => '2',
            &[Dit, Dit, Dit, Dah, Dah] => '3',
            &[Dit, Dit, Dit, Dit, Dah] => '4',
            &[Dit, Dit, Dit, Dit, Dit] => '5',
            &[Dah, Dit, Dit, Dit, Dit] => '6',
            &[Dah, Dah, Dit, Dit, Dit] => '7',
            &[Dah, Dah, Dah, Dit, Dit] => '8',
            &[Dah, Dah, Dah, Dah, Dit] => '9',
            &[Dah, Dah, Dah, Dah, Dah] => '0',
            e => return Err(format!("Failed to parse token {e:?}").into()),
            // e => panic!("Failed to parse token {e:?}"),
        };
        output.push(ch);
    }
    Ok(output)
}

fn get_next_morse(mut stream: &[MorseBit]) -> (&[MorseBit], &[MorseBit]) {
    if stream.first() == Some(&MorseBit::WordSpace) {
        return stream.split_at(1);
    }
    while stream.starts_with(&[MorseBit::LetterSpace]) {
        stream = stream.strip_prefix(&[MorseBit::LetterSpace]).unwrap_or_default();
    }
    match stream.split(|m| m == &MorseBit::LetterSpace).next() {
        Some(chs) => {
            let rest = chs.len() + 1;
            if rest >= stream.len() {
                (chs, &[])
            } else {
                (chs, &stream[rest..])
            }
        },
        None => (stream, &[]),
    }
}

#[test]
fn test_encode_empty() {
    assert_eq!(encode("").unwrap(), vec![]);
}

#[test]
fn test_encode_letter() {
    use MorseBit::*;
    assert_eq!(encode(" ").unwrap(), vec![WordSpace, LetterSpace]);
    assert_eq!(encode("a").unwrap(), vec![Dit, Dah, LetterSpace]);
}

#[test]
fn test_encode_hello() {
    use MorseBit::*;
    assert_eq!(encode("hello").unwrap(), vec![
        Dit, Dit, Dit, Dit, LetterSpace, /* h */
        Dit, LetterSpace, /* e */
        Dit, Dah, Dit, Dit, LetterSpace, /* l */
        Dit, Dah, Dit, Dit, LetterSpace, /* l */
        Dah, Dah, Dah, LetterSpace, /* o */
    ]);
}

#[test]
fn test_encode_jatorocket() {
    use MorseBit::*;
    assert_eq!(encode("jato rocket").unwrap(), vec![
        Dit, Dah, Dah, Dah, LetterSpace, /* j */
        Dit, Dah, LetterSpace, /* a */
        Dah, LetterSpace, /* t */
        Dah, Dah, Dah, LetterSpace, /* o */
        WordSpace, LetterSpace, /* space */
        Dit, Dah, Dit, LetterSpace, /* r */
        Dah, Dah, Dah, LetterSpace, /* o */
        Dah, Dit, Dah, Dit, LetterSpace, /* c */
        Dah, Dit, Dah, LetterSpace, /* k */
        Dit, LetterSpace, /* e */
        Dah, LetterSpace, /* t */
    ]);
}

#[test]
fn test_decode_empty() {
    assert_eq!(decode(&[]).unwrap(), "");
    assert_eq!(decode(&[]).unwrap(), "");
}

#[test]
fn test_decode_letter() {
    use MorseBit::*;
    assert_eq!(decode(&[Dit]).unwrap(), "e");
    assert_eq!(decode(&[Dit, LetterSpace]).unwrap(), "e");
    assert_eq!(decode(&[Dit, Dah]).unwrap(), "a");
    assert_eq!(decode(&[Dit, Dah, LetterSpace]).unwrap(), "a");
    assert_eq!(decode(&[Dit, Dah, Dah, Dah]).unwrap(), "j");
}

#[test]
fn test_decode_hello() {
    use MorseBit::*;
    assert_eq!(decode(&[Dit, Dit, Dit, Dit, LetterSpace, /* H */
                        Dit, LetterSpace, /* e */
                        Dit, Dah, Dit, Dit, LetterSpace, /* l */
                        Dit, Dah, Dit, Dit, LetterSpace, /* l */
                        Dah, Dah, Dah] /* o */
                    ).unwrap(),
               "hello");
}

#[test]
fn test_decode_jatorocket() {
    use MorseBit::*;
    assert_eq!(decode(&[Dit, Dah, Dah, Dah, LetterSpace, /* j */
                        Dit, Dah, LetterSpace, /* a */
                        Dah, LetterSpace, /* t */
                        Dah, Dah, Dah, LetterSpace, /* o */
                        WordSpace,
                        Dit, Dah, Dit, LetterSpace, /* r */
                        Dah, Dah, Dah, LetterSpace, /* o */
                        Dah, Dit, Dah, Dit, LetterSpace, /* c */
                        Dah, Dit, Dah, LetterSpace, /* k */
                        Dit, LetterSpace, /* e */
                        Dah, LetterSpace, /* t */
                        ]
                    ).unwrap(),
               "jato rocket");
}