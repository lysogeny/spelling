use encoding::{all::ISO_8859_1, EncoderTrap, Encoding};
use hunspell_rs::Hunspell;
use hunspell_sys::{Hunspell_create, Hunspell_get_dic_encoding, Hunspell_spell};

const AFFIX_PATH: &str = "lang/de_DE.aff";
const DIC_PATH: &str = "lang/de_DE.dic";

unsafe fn check_with_encoding(word: &str) -> bool {
    let affix_path = std::ffi::CString::new(AFFIX_PATH).unwrap();
    let dic_path = std::ffi::CString::new(DIC_PATH).unwrap();
    let hs = Hunspell_create(affix_path.as_ptr(), dic_path.as_ptr());
    let word_enc: &[u8] = &ISO_8859_1.encode(word, EncoderTrap::Strict).unwrap();
    let word_c = std::ffi::CString::new(word_enc).unwrap();
    Hunspell_spell(hs, word_c.as_ptr()) > 0
}

fn check_binding(word: &str) -> bool {
    let spell = Hunspell::new(AFFIX_PATH, DIC_PATH);
    spell.check(word) == hunspell_rs::CheckResult::FoundInDictionary
}

fn main() {
    let words = vec!["Öl", "Baum", "Käse", "Motorrad", "Tippfheler"];
    println!("{:>10} {:<8} {:<8}", "word", "binding", "with_encoding");
    for word in words {
        let result_binding = check_binding(word);
        let result_encoded = unsafe { check_with_encoding(word) };
        println!("{word:>10} {result_binding:<8} {result_encoded:<8}")
    }
}
