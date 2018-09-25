use std::borrow::Cow::{self, Borrowed, Owned};

use rustyline::completion::Completer;
use rustyline::error::ReadlineError;
use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::Helper;

use commands_processor::CommandProcessor;

pub struct CommandHelper<'a> {
    processor: &'a CommandProcessor,
}

impl<'a> CommandHelper<'a> {
    pub fn new(processor: &'a CommandProcessor) -> Self {
        CommandHelper { processor }
    }
}

impl<'a> Completer for CommandHelper<'a> {
    type Candidate = String;

    fn complete(&self, _line: &str, pos: usize) -> Result<(usize, Vec<String>), ReadlineError> {
        Ok((pos, vec![]))
    }
    //     fn complete(&self, line: &str, pos: usize) -> Result<(usize, Vec<Pair>)> {
    //         let (start, path, esc_char, break_chars, quote) =
    //             if let Some((idx, quote)) = find_unclosed_quote(&line[..pos]) {
    //                 let start = idx + 1;
    //                 if quote == Quote::Double {
    //                     (
    //                         start,
    //                         unescape(&line[start..pos], DOUBLE_QUOTES_ESCAPE_CHAR),
    //                         DOUBLE_QUOTES_ESCAPE_CHAR,
    //                         &self.double_quotes_special_chars,
    //                         quote,
    //                     )
    //                 } else {
    //                     (
    //                         start,
    //                         Borrowed(&line[start..pos]),
    //                         None,
    //                         &self.break_chars,
    //                         quote,
    //                     )
    //                 }
    //             } else {
    //                 let (start, path) = extract_word(line, pos, ESCAPE_CHAR, &self.break_chars);
    //                 let path = unescape(path, ESCAPE_CHAR);
    //                 (start, path, ESCAPE_CHAR, &self.break_chars, Quote::None)
    //             };
    //         let matches = try!(filename_complete(&path, esc_char, break_chars, quote));
    //         Ok((start, matches))
    //     }
    // }
}

impl<'a> Hinter for CommandHelper<'a> {
    fn hint(&self, line: &str, _pos: usize) -> Option<String> {
        if line == "hello" {
            Some(" World".to_owned())
        } else {
            None
        }
    }
}

impl<'a> Highlighter for CommandHelper<'a> {
    fn highlight_prompt<'p>(&self, prompt: &'p str) -> Cow<'p, str> {
        Borrowed(prompt)
    }

    fn highlight_hint<'h>(&self, hint: &'h str) -> Cow<'h, str> {
        Owned("\x1b[1m".to_owned() + hint + "\x1b[m")
    }
}

impl<'a> Helper for CommandHelper<'a> {}
