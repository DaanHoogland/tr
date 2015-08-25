//!TR(1)                                                       TR(1)
//!
//!NAME
//!
//!    tr - translate characters
//!
//!SYNOPSIS
//!    tr [ -cds ] [ string1 [ string2 ] ]
//!
//!DESCRIPTION
//!    Tr copies the standard input to the standard output with
//!    substitution or deletion of selected characters.  Input
//!    characters found in string1 are mapped into the correspond-
//!    ing characters of string2. When string2 is short it is pad-
//!    ded to the length of string1 by duplicating its last charac-
//!    ter.  Any combination of the options -cds may be used:
//!
//!    -c   complement the set of characters in string1 with
//!         respect to the universe of characters whose ASCII codes
//!         are 01 through 0377
//!
//!    -d   delete all input characters in string1
//!
//!    -s   squeeze all strings of repeated output characters that
//!         are in string2 to single characters
//!
//!    In either string the notation a-b means a range of charac-
//!    ters from a to b in increasing ASCII order.  The character
//!    `\' followed by 1, 2 or 3 octal digits stands for the char-
//!    acter whose ASCII code is given by those digits.  A `\' fol-
//!    lowed by any other character stands for that character.
//!
//!    The following example creates a list of all the words in
//!    `file1' one per line in `file2', where a word is taken to be
//!    a maximal string of alphabetics.  The second string is
//!    quoted to protect `\' from the Shell.  012 is the ASCII code
//!    for newline.
//!
//!        tr -cs A-Za-z '\012' <file1 >file2
//!
//!SEE ALSO
//!    ed(1), ascii(7)
//!
//!BUGS
//!    loads.

#[macro_use]
extern crate clap;
use clap::{Arg, App};

struct Translation {
	complement: bool,
	delete: bool,
	squeeze: bool,
	search: String,
	replace: String
}

fn print_opts(tr: Translation)
{
    println!("flags\ncompliment {}\ndelete: {}\nsqueeze {}", tr.complement, tr.delete, tr.squeeze);
    println!("search: {}", tr.search);
    println!("replace: {}", tr.replace);
}


fn append_replace(mut tr: Translation) -> Translation {
	let mut newreplace = String::new();
	{// extra scope to guard the argument
        let mut search = tr.search.chars();
        let mut replacechars = tr.replace.chars();

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
    tr.replace = newreplace;
    return tr;
}


fn get_opts(mut tr: Translation) -> Translation {
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

    tr.search = matches.value_of("search").unwrap().to_string();
    tr.replace = matches.value_of("replace").unwrap_or("").to_string();
    tr.complement = matches.is_present("complement");
    tr.delete = matches.is_present("delete");
    tr.squeeze = matches.is_present("squeeze");
    return tr;
}

#[cfg(not(test))]
fn main() {
	let tr = Translation { complement: false, delete: false, squeeze: false, search: String::new(), replace: String::new()}; 
    let tr = get_opts(tr);
	// actually put them somewhere for retrieval by the other parts of the program instead of print
    let tr = append_replace(tr);
    print_opts(tr);
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
    use super::{append_replace, Translation};

    #[test]
    fn append_replace_when_it_is_short() {
    	let tr = Translation {search: "abcde".to_string() , replace: "xyz".to_string(), complement: false, delete: false, squeeze: false};
        let tr = append_replace(tr);
        assert_eq!("xyzzz", tr.replace);
    }
    #[test]
    fn append_replace_when_it_is_long() {
    	let tr = Translation {search: "a".to_string() , replace: "xyz".to_string(), complement: false, delete: false, squeeze: false};
        let tr = append_replace(tr);
        assert_eq!("x", tr.replace);
    }
    #[test]
    fn append_replace_when_it_is_exact_in_length() {
    	let tr = Translation {search: "abc".to_string() , replace: "xyz".to_string(), complement: false, delete: false, squeeze: false};
        let tr = append_replace(tr);
        assert_eq!("xyz", tr.replace);
    }
}

