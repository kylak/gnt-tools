//! This crate has been created in order to be a tool box for studying the greek new testament.
//! The crate is provided AS-IS.
//! # Examples
//! 
//! ```
//! use gnt_tools::core_text;
//!
//! let s = "16 Εἶπεν δὲ παραβολὴν πρὸς αὐτοὺς λέγων·
//!          ἀνθρώπου τινὸς πλουσίου εὐφόρησεν ἡ χώρα. 17 
//!          καὶ διελογίζετο ἐν ἑαυτῷ λέγων· τί ποιήσω, ὅτι 
//!          οὐκ ἔχω ποῦ συνάξω τοὺς καρπούς μου; ";
//!
//! let s2 = "ειπενδεπαραβοληνπροϲαυτουϲλεγωνανθρωπουτ\
//!           ινοϲπλουϲιουευφορηϲενηχωρακαιδιελογιζετοενεαυτω\
//!           λεγωντιποιηϲωοτιουκεχωπουϲυναξωτουϲκαρπουϲμου";
//!
//! assert_eq!(core_text(String::from(s)), String::from(s2));
//! ```

use unicode_normalization::UnicodeNormalization;

/*
   If one has developped a greedy function (cf. below),
   this format function would still may be useful to detect 
   a uncommon character in the greek text (by comparing its 
   result with the greedy result).
*/
/// The function gives the core text of a greek new testament critical edition.  
///It might be useful for comparing greek new testament critical editions by gettig their "core" differences/concordances.
///
/// Note on this function : 
/// - it does not replace nomina sacras (e.g., κϲ) by their non-abreviated form (resp. κυριοϲ), nor words (e.g., κύριος) by their nomina sacras form (when a nomina sacra form exists) (resp. κϲ).
/// - it is made to delete any character used to encode nomina sacras (e.g., '|', or '(' and ')'), hence |κς| will give κϲ.
/// # Example : 
/// ```
/// use gnt_tools::core_text;
///
/// let s = "16 Εἶπεν δὲ παραβολὴν πρὸς αὐτοὺς λέγων·
///          ἀνθρώπου τινὸς πλουσίου εὐφόρησεν ἡ χώρα. 17 
///          καὶ διελογίζετο ἐν ἑαυτῷ λέγων· τί ποιήσω, ὅτι 
///          οὐκ ἔχω ποῦ συνάξω τοὺς καρπούς μου; ";
///
/// let s2 = "ειπενδεπαραβοληνπροϲαυτουϲλεγωνανθρωπουτ\
///           ινοϲπλουϲιουευφορηϲενηχωρακαιδιελογιζετοενεαυτω\
///           λεγωντιποιηϲωοτιουκεχωπουϲυναξωτουϲκαρπουϲμου";
///
/// assert_eq!(core_text(String::from(s)), String::from(s2));
/// ```
pub fn core_text(mut s : String) -> String {

    /* We remove punctuation and others signs. ------------------------
    Warning: some softwares may use '|', '(' and ')' to manage nomina sacras */
    s = s.replace(&['¶', '⋄', '?', '!', '–', ':', 
    ';' /* greek question mark u+037e */, 
    ';' /* semicolon u+003b */,
    ',', '.', '·', '“', '”', '‘', '’', '᾽',
    'ʼ', '*', '[', ']', '…', '⟦', '⟧', '|', '(', ')'], "");

    // We remove any spaces. --------------------------------------------
    s = s.chars().filter(|c| !c.is_whitespace()).collect();

    // We remove diacritics signs. --------------------------------------
    const LEN: usize = '\u{036f}' as usize - '\u{0300}' as usize;
    let mut arr = ['\0'; LEN];
    for (item, ch) in std::iter::zip(&mut arr, '\u{0300}'..='\u{036f}') {
        *item = ch;
    }
    s = s.nfd().to_string().replace(arr, "");

    // We remove any digit.
    s = s.chars().filter(|c| 
               !(*c >= '\u{0030}' && *c <= '\u{0039}') // digit
            ).collect();

    replace(s)
}

fn replace (mut s : String) -> String {
    
    // We remplace any "invisible nu" by a "true one". ------------------
    s = s.replace("ˉ", "ν");
    
    // We change any uppercase letter to lowercase. ---------------------
    s = s.to_lowercase();
    
    // We replace every sigmas to the lunar sigma. ----------------------
    s.replace(&['σ', 'ς'], "ϲ")
}

/* I didn't delete this function in order to show that 
   a greedy formating that says "keep only the greek text
   and remove anything that is not a greek character" is
   actually not that obvious.
*/
#[allow(dead_code)]
fn greedy_format(s : &str) -> String {

    #[allow(non_snake_case)]
    let mut S : String = String::from(s);
    S = replace(S);
    
    // We remove any character that is not a greek character.
    S.chars().filter(|c| 
                           *c >= '\u{03B1}' && *c <= '\u{03C9}'     // lowercases
                        || *c >= '\u{0391}' && *c <= '\u{03A9}'     // upercases
                        || *c >= '\u{10140}' && *c <= '\u{1018E}'   // digits
                    )
             .collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_greedy_format() {
        assert_eq!(greedy_format("Hi Πέτρος!"), String::from("πετροϲ"));
    }
    
    #[test]
    fn test_core_text() {
   
        let s = "16 Εἶπεν δὲ παραβολὴν πρὸς αὐτοὺς λέγων·
            ἀνθρώπου τινὸς πλουσίου εὐφόρησεν ἡ χώρα. 17 
            καὶ διελογίζετο ἐν ἑαυτῷ λέγων· τί ποιήσω, ὅτι 
            οὐκ ἔχω ποῦ συνάξω τοὺς καρπούς μου; ";

        let s2 = "ειπενδεπαραβοληνπροϲαυτουϲλεγωνανθρωπουτ\
            ινοϲπλουϲιουευφορηϲενηχωρακαιδιελογιζετοενεαυτω\
            λεγωντιποιηϲωοτιουκεχωπουϲυναξωτουϲκαρπουϲμου";

        assert_eq!(core_text(String::from(s)), String::from(s2));
    }
}
