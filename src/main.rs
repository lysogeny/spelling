use hunspell_rs::Hunspell;
use hunspell_sys::{
    Hunspell_create,
    Hunspell_spell,
};
use encoding::{all::ISO_8859_1, Encoding, EncoderTrap};

const affix_path: &str = "lang/de_DE.aff";
const dic_path: &str = "lang/de_DE.dic";

unsafe fn check_with_encoding(word: &str) -> bool {
    let hs = Hunspell_create(
        std::ffi::CString::new(affix_path).unwrap().as_ptr(),
        std::ffi::CString::new(dic_path).unwrap().as_ptr(),
    );
    let word_enc: &[u8] = &ISO_8859_1.encode(word, EncoderTrap::Strict).unwrap();
    Hunspell_spell(hs, std::ffi::CString::new(word_enc).unwrap().as_ptr()) > 0
}

fn check_binding(word: &str) -> bool {
    let spell = Hunspell::new(affix_path, dic_path);
    spell.check(word) == hunspell_rs::CheckResult::FoundInDictionary
}

fn main() {
    let words = vec![
        "Öl", 
        "Baum",
    ];
        println!("{:>8} {:<8} {:<8}", "word", "binding", "with_encoding");
    for word in words {
        let result_binding = check_binding(word);
        let result_encoded = unsafe {check_with_encoding(word)};
        println!("{word:>8} {result_binding:<8} {result_encoded:<8}")
    }
}
