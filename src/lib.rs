//! Tools to help studying the greek new testament.
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

/// The function gives the core text of a greek new testament critical edition.  
/// It might be useful for comparing greek new testament critical editions by gettig their "core" differences/concordances.
///
/// In concrete terms, it remove diacritic signs, remove any character that is not in the greek alphabet, puts all greek letters in lowercase, and change all sigmas to lunar sigma.
///
/// So this function :
/// - does not replace nomina sacras (e.g., κϲ) by their non-abreviated form (resp. κυριοϲ), nor words (e.g., κύριος) by their nomina sacras form (when a nomina sacra form exists) (resp. κϲ).
/// - is made to delete any character used to encode nomina sacras (e.g., '|', or '(' and ')'), hence |κς| will give κϲ.
/// - does delete all 'ˉ' characters (so παραβολὴˉ becomes παραβολη, not παραβολην)
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

/*
pub fn core_char(c: &char) -> Option<char> {
   
    c.nfd().reduce(|core_c, i| 
                  
                  if ('α'..='ω').contains(&i) 
                  || ('Α'..='Ω').contains(&i) 
                  {
                      match i {
                          'σ' | 'ς' | 'Σ' => 'ϲ',
                          _ => i.to_lowercase().nth(0).unwrap()
                      }
                  }
                  else {
                      core_c
                  }
            )
}

// Would it be better to return an Option<String> ?
pub fn core_text2(s: &str) -> String {
    s.chars().filter_map(|c| core_char(&c)).collect()
}
*/

pub fn core_text(mut s: String) -> String {
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

fn replace(mut s: String) -> String {
    // We remplace any "invisible nu" by a "true one". ------------------
    // EDIT : not anymore (because it's not easy to manage), but I hope later.
    // s = s.replace("ˉ", "ν");

    // We change any uppercase letter to lowercase. ---------------------
    s = s.to_lowercase();

    // We replace every sigmas to the lunar sigma. ----------------------
    s.replace(&['σ', 'ς'], "ϲ")
}

fn greedy_format(s: &str) -> String {
    #[allow(non_snake_case)]
    let S = String::from(s);

    // We remove any character that is not a greek character.
    S.chars()
        .filter(
            |c| {
                *c >= '\u{03B1}' && *c <= '\u{03C9}' // lowercases
                        || *c == '\u{03F2}'
            }, // the lunar sigma
        )
        .collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

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
   
    /*
    #[test]
    fn test_core_char() {
        let s = "16 Εἶπεν δὲ παραβολὴν πρὸς αὐτοὺς λέγων·
            ἀνθρώπου τινὸς πλουσίου εὐφόρησεν ἡ χώρα. 17 
            καὶ διελογίζετο ἐν ἑαυτῷ λέγων· τί ποιήσω, ὅτι 
            οὐκ ἔχω ποῦ συνάξω τοὺς καρπούς μου; ";

        let s2 = "ειπενδεπαραβοληνπροϲαυτουϲλεγωνανθρωπουτ\
            ινοϲπλουϲιουευφορηϲενηχωρακαιδιελογιζετοενεαυτω\
            λεγωντιποιηϲωοτιουκεχωπουϲυναξωτουϲκαρπουϲμου";

        assert_eq!(core_text2(s).as_str(), s2);
    }
    */

    /*
       https://stackoverflow.com/questions/517923/what-is-the-best-way-to-remove-accents-normalize-in-a-python-unicode-string

       https://crates.io/crates/hebrew_unicode_utils

       Tester fonctionnalite :
        - (1) qu'il n'y ait plus d'accentuations
        - (2) qu'il n'y ait plus de majuscules
        - (3) qu'il n'y ait plus de sigma autre que le lunaire
        - (4) qu'il n'y ait pas d'autre caractere que les
          caracteres alphabetique grec.
       Tester une combinaison de deux :
        - (1) et (2)
        - (1) et (3)
        - (1) et (4)
        - (2) et (3)
        - (2) et (4)
        - (3) et (4)
       Tester une combinaison de trois :
        - (1) et (2) et (3)
        - (1) et (2) et (4)
        - (1) et (3) et (4)
        - (2) et (3) et (4)
      Tester toutes les fonctionnalite ensemble :
        - (1) et (2) et (3) et (4)

      Tester sur quelle donnee ?
      (1) :
          - (1.1) que sur des accents
          - (1.2) que sur des caracteres accentues - un unicode
          - (1.3) que sur des caractere accentues - deux unicodes
          - (1.3) que sur des caracteres non accentues
          - (1.4) que sur une chaine ayant des caracteres accentues et non accentues

    */
}
