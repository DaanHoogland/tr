#[macro_use]
extern crate clap;
use clap::{Arg, App};
use std::process::exit;

const MAN_PAGE: &'static str = /* @MANSTART{tr} */ r#"NAME
    tr - translate characters

SYNOPSIS
    tr [ -cds ] [ string1 [ string2 ] ]

DESCRIPTION
    Tr copies the standard input to the standard output with
    substitution or deletion of selected characters.  Input
    characters found in string1 are mapped into the correspond-
    ing characters of string2. When string2 is short it is pad-
    ded to the length of string1 by duplicating its last charac-
    ter.  Any combination of the options -cds may be used:

    -c   complement the set of characters in string1 with
         respect to the universe of characters whose ASCII codes
         are 01 through 0377

    -d   delete all input characters in string1

    -s   squeeze all strings of repeated output characters that
         are in string2 to single characters

    In either string the notation a-b means a range of charac-
    ters from a to b in increasing ASCII order.  The character
    `\' followed by 1, 2 or 3 octal digits stands for the char-
    acter whose ASCII code is given by those digits.  A `\' fol-
    lowed by any other character stands for that character.

    The following example creates a list of all the words in
    `file1' one per line in `file2', where a word is taken to be
    a maximal string of alphabetics.  The second string is
    quoted to protect `\' from the Shell.  012 is the ASCII code
    for newline.

        tr -cs A-Za-z '\012' <file1 >file2

SEE ALSO
    ed(1), ascii(7)

BUGS
    loads.
"#; /* @MANEND */


struct Translation {
    complement:  bool,
    delete:      bool,
    squeeze:     bool,
    search:      String,
    replace:     String
}

impl Translation {
    fn print_opts(&self)
    {
        println!("flags\ncompliment {}\ndelete: {}\nsqueeze {}", self.complement, self.delete, self.squeeze);
        println!("search: {}", self.search);
        println!("replace: {}", self.replace);
    }

    fn append_replace(&mut self) -> &mut Translation {
	      let mut newreplace = String::new();
	      {// extra scope to guard the argument
            let mut search = self.search.chars();
            let mut replacechars = self.replace.chars();

            let mut kar = replacechars.next();
            let mut nextkar : Option<char> = kar;
            while ! search.next().is_none() {
                if ! nextkar.is_none() {
                    kar = nextkar;
                }
                newreplace.push(kar.unwrap());
                nextkar = replacechars.next();
            }
        }
        self.replace = newreplace;
        self
    }

    fn get_opts(&mut self) -> &mut Translation {
        let matches = App::new("tr")
            .author("Daan Hoogland <daan@gmail.com>")
            .version(&crate_version!()[..])
            .about("translate")
            .arg(Arg::with_name("complement")
                 .short("c")
                 .help("Use the complement the set of characters in string1 with respect to the universe of characters whose ASCII codes are 01 through 0377"))
            .arg(Arg::with_name("squeeze")
                 .help("squeeze all strings of repeated output characters that are in string2 to single characters")
                 .short("s"))
            .arg(Arg::with_name("delete")
                 .short("d")
                 .conflicts_with("replace")
                 .help("delete all input characters in string1"))
            .arg(Arg::with_name("search")
                 .help("character string to match")
                 .required(true)
            )
            .arg(Arg::with_name("replace")
                 .help("character string to substitude")
                 .conflicts_with("delete")
            )
            .get_matches();
        self.search = matches.value_of("search").unwrap().to_string();
        self.replace = matches.value_of("replace").unwrap_or("").to_string();
        self.complement = matches.is_present("complement");
        self.delete = matches.is_present("delete");
        self.squeeze = matches.is_present("squeeze");
        self
    }
    fn check_opts(&mut self) -> &mut Translation {
        if !self.delete && !self.squeeze && self.replace.len() == 0 {
            // big issue
            println!("replace string can not be empty when neither -s nor -d is specified.");
            println!("{}", MAN_PAGE);
            exit(1);
        }

        self
    }
}
#[cfg(not(test))]
fn main() {
	let mut tr = Translation { complement: false, delete: false, squeeze: false, search: String::new(), replace: String::new()};
    tr.get_opts().check_opts();
	// actually put them somewhere for retrieval by the other parts of the program instead of print
    tr.append_replace();
    tr.print_opts();
// if complement is turned on recreate 'search' to contain the complement of search
// open std input
// open std ouput
// read a char
// if not in search => pass through
// decide what to do
// switching over:
// case either 'delete' switched on or find matching char in 'replace'
// case 'squeeze' switched on
// 
}

#[cfg(test)]
mod tests {
    use super::Translation;

    #[test]
    fn append_replace_when_it_is_short() {
        let mut tr = Translation {search: "abcde".to_string() , replace: "xyz".to_string(), complement: false, delete: false, squeeze: false};
        tr.append_replace();
        assert_eq!("xyzzz", tr.replace);
    }
    #[test]
    fn append_replace_when_it_is_long() {
        let mut tr = Translation {search: "a".to_string() , replace: "xyz".to_string(), complement: false, delete: false, squeeze: false};
        tr.append_replace();
        assert_eq!("x", tr.replace);
    }
    #[test]
    fn append_replace_when_it_is_exact_in_length() {
        let mut tr = Translation {search: "abc".to_string() , replace: "xyz".to_string(), complement: false, delete: false, squeeze: false};
        tr.append_replace();
        assert_eq!("xyz", tr.replace);
    }
}
