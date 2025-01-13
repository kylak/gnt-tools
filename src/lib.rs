//! Tools to help studying the greek new testament.
//! The crate is provided AS-IS.
//! # Examples
//!
//! ```
//! use gnt_tools::core_char;
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
//! let core_text : String = s.chars()
//!                           .filter_map(|c| core_char(&c))
//!                           .collect();
//!
//! assert_eq!(core_text.as_str(), s2);
//! ```

use unicode_normalization::UnicodeNormalization;

// TODO : doc de la fonction a re-ecrire.
/// The function helps giving the core text of a greek new testament critical edition.  
/// This might be useful for comparing greek new testament critical editions by gettig their "core" differences/concordances.
///
/// In concrete terms, core_char remove any character that is not in the greek alphabet, puts all greek letters in lowercase, and change all sigmas to lunar sigma.
///
/// So this function :
/// - does not replace nomina sacras (e.g., κϲ) by their non-abreviated form (resp. κυριοϲ), nor words (e.g., κύριος) by their nomina sacras form (when a nomina sacra form exists) (resp. κϲ).
/// - is made to delete any character used to encode nomina sacras (e.g., '|', or '(' and ')'), hence |κς| will give κϲ.
/// - does delete all 'ˉ' characters (so παραβολὴˉ becomes παραβολη, not παραβολην)
/// TODO : expliquer pourquoi on ne garde pas le point median.
/// # Example :
/// ```
/// use gnt_tools::core_char;
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
/// let core_text : String = s.chars()
///                           .filter_map(|c| core_char(&c))
///                           .collect();
///
/// assert_eq!(core_text.as_str(), s2);
/// ```
// - gerer les invisibles nu? [ca demandera peut-etre un changement du type de retour]
// - todo : faire une demande a l'unicode foundation pour ajouter les caracteres grecs onciales.
// - expliquer pourquoi on ne garde pas le point milieu
#[inline]
pub fn core_char(c: &char) -> Option<char> {
   
    c.nfd().fold(None, |core_c, i| 
                
            // TODO : on fait quoi avec les symboles numeriques?

              /* It could be interesting to check if 
               * `('α'..='ω').contains(&i)` is faster
               * than 'α' <= i && i <= 'ω'. */
              
              if 'α' <= i && i <= 'ω' 
              || 'Α' <= i && i <= 'Ω' 
              {
                  match i {
                      'σ' | 'ς' | 'Σ' => Some('ϲ'),
                      _ => i.to_lowercase().nth(0)
                  }
              }
              else if i == ';'
                   || 'Ͱ' > i || i > 'Ͽ'
                   || i == '·'
              {
                  core_c
              }
              else {
                  panic!("Greek unicode character '{i}' is \
                  not handled. If you think it would be \
                  relevant to handle this character, please \
                  open an issue on our GitHub repository : \
                  https://github.com/kylak/gnt-tools/issues.");
              }
            )
}

// fonction qui donne les caracteres supprimes par core_char
// pub fn new_char(s1, s2)
// ca permet de pouvoir etre sur des caracteres qu'on a supprime

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_core_text() {
        let s = "16 Εἶπεν δὲ παραβολὴν πρὸς αὐτοὺς λέγων·
            ἀνθρώπου τινὸς πλουσίου εὐφόρησεν ἡ Ͷχώρα. 17 
            καὶ διελογίζετο ἐν ἑαυτῷ λέγων· τί ποιήσω, ὅτι 
            οὐκ ἔχω ποῦ συνάξω τοὺς καρπούς μου; ";

        let s2 = "ειπενδεπαραβοληνπροϲαυτουϲλεγωνανθρωπουτ\
            ινοϲπλουϲιουευφορηϲενηχωρακαιδιελογιζετοενεαυτω\
            λεγωντιποιηϲωοτιουκεχωπουϲυναξωτουϲκαρπουϲμου";

        let core_text : String = s.chars()
                                  .filter_map(|c| core_char(&c))
                                  .collect();

         assert_eq!(core_text.as_str(), s2);
    }

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
