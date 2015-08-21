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



fn main() {
	parse_cl();

    // more porgram logic goes here...
}


fn parse_cl() {
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

    let search = matches.value_of("search").unwrap();
    let replace = matches.value_of("replace").unwrap_or("");
    let compliment = matches.is_present("compliment");
    let delete = matches.is_present("delete");
    let squeeze = matches.is_present("squeeze");

	// actually put them somewhere for retrieval by the other parts of the program instead of print
    println!("flags\ncompliment {}\ndelete: {}\nsqueeze {}", compliment, delete, squeeze);
    println!("search: {}", search);
    println!("replace: {}", replace);
}
