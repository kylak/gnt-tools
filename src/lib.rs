//! Tools to help studying the greek new testament.
//! The crate is provided AS-IS.
//! # Examples
//! 
//! ```
//! use gnt_tools::core_text;
//!
//! let s = "16 Εἶπεν δὲ παραβολὴˉ πρὸς αὐτοὺς λέγων·
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

/// The function gives the core text of a greek new testament critical edition.  
/// It might be useful for comparing greek new testament critical editions by gettig their "core" differences/concordances.
///
/// In concrete terms, it remove diacritic signs, change nu bars to nu letter, remove any character that is not in the greek alphabet, change all sigmas to lunar sigma, and puts all greek letters in lowercase.
///
/// Also, this function : 
/// - does not replace nomina sacras (e.g., κϲ) by their non-abreviated form (resp. κυριοϲ), nor words (e.g., κύριος) by their nomina sacras form (when a nomina sacra form exists) (resp. κϲ).
/// - is made to delete any character used to encode nomina sacras (e.g., '|', or '(' and ')'), hence |κς| will give κϲ.
/// # Example : 
/// ```
/// use gnt_tools::core_text;
///
/// let s = "16 Εἶπεν δὲ παραβολὴˉ πρὸς αὐτοὺς λέγων·
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

    // We remove diacritics signs. Doing it now avoids the greedy_format 
    // function to remove some greek letters (the accentued ones).
    const LEN: usize = '\u{036f}' as usize - '\u{0300}' as usize;
    let mut arr = ['\0'; LEN];
    for (item, ch) in std::iter::zip(&mut arr, '\u{0300}'..='\u{036f}') {
        *item = ch;
    }
    s = s.nfd().to_string().replace(arr, "");

    s = replace(s);
    greedy_format(s.as_str())
}

fn replace (mut s : String) -> String {
    
    // We remplace any "invisible nu" by a "true one". ------------------
    s = s.replace("ˉ", "ν");
    
    // We change any uppercase letter to lowercase. ---------------------
    s = s.to_lowercase();
    
    // We replace every sigmas to the lunar sigma. ----------------------
    s.replace(&['σ', 'ς'], "ϲ")
}

fn greedy_format(s : &str) -> String {

    #[allow(non_snake_case)]
    let S = String::from(s);
    
    // We remove any character that is not a greek character.
    S.chars().filter(|c| 
                           *c >= '\u{03B1}' && *c <= '\u{03C9}' // lowercases
                        || *c == '\u{03F2}'                     // the lunar sigma
                    )
             .collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_core_text() {
   
        let s = "16 Εἶπεν δὲ παραβολὴˉ πρὸς αὐτοὺς λέγων·
            ἀνθρώπου τινὸς πλουσίου εὐφόρησεν ἡ χώρα. 17 
            καὶ διελογίζετο ἐν ἑαυτῷ λέγων· τί ποιήσω, ὅτι 
            οὐκ ἔχω ποῦ συνάξω τοὺς καρπούς μου; ";

        let s2 = "ειπενδεπαραβοληνπροϲαυτουϲλεγωνανθρωπουτ\
            ινοϲπλουϲιουευφορηϲενηχωρακαιδιελογιζετοενεαυτω\
            λεγωντιποιηϲωοτιουκεχωπουϲυναξωτουϲκαρπουϲμου";

        assert_eq!(core_text(String::from(s)), String::from(s2));
    }
}
