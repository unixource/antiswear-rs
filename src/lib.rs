//! A library for recognizing obscene words with distortion protection,

use std::char;

pub mod utils;

/// Sets the method for recognizing words.
/// In the case of checking the `short` variable `Mode::Startswith` is replaced by `Mode::Equally` to avoid accidental triggers of similar words.
pub enum Mode {
    Contains,
    Startswith,
    Endswith,
    Equally,
}

impl Mode {
    fn is_contains(&self, value: &str, word: &str) -> bool {
        match self {
            Mode::Startswith => value.starts_with(word),
            Mode::Contains => value.contains(word),
            Mode::Endswith => value.ends_with(word),
            Mode::Equally => value == word,
        }
    }
}

/// The main structure that stores information about the language and has a function for checking.
pub struct Antiswear {
    pub bypasses: Vec<Replacement>,
    pub prefixes: Vec<String>,
    pub short: Vec<String>,
    pub alphabet: String,
    pub replacements: Vec<Replacement>,
    pub exceptions: Vec<String>,
    pub mode: Mode,
}

/// Builder for Antiswear with a simple API
///
/// A vector is created by dividing by spaces.
///
/// # Examples
///
/// ```
/// use antiswear_rs::{Builder, Mode};
///
/// Builder {
///     bypasses: "4-f 1-i", // ex. b1tch / "<from>-<into>"
///     prefixes_first: "",  // For `Mode::Startswith`, Builder will connect prefixes_first and prefixes_second
///     prefixes_second: "",
///     short: "fuck bitch",
///     alphabet: "abcdefghijklmnopqrstuvwxyz", // List of allowed characters, including characters from `bypasses`
///     replacements: "", // ex. хруст->hrust (from cyrillic to latin) / "<from>-<into>"
///     exceptions: "bitchin bitchy", // The list of ignored words
///     mode: Mode::Contains,
/// }.build(); // -> Antiswear
/// ```
pub struct Builder<'a> {
    pub bypasses: &'a str,
    pub prefixes_first: &'a str,
    pub prefixes_second: &'a str,
    pub short: &'a str,
    pub alphabet: &'a str,
    pub replacements: &'a str,
    pub exceptions: &'a str,
    pub mode: Mode,
}

pub struct Analyze {
    pub word: String,
    pub index: usize,
}

impl<'a> Builder<'a> {
    pub fn build(self) -> Antiswear {
        let mut alphabet = self.alphabet.to_string();
        let bypasses = Replacement::from_str(self.bypasses);

        for bypass in &bypasses {
            alphabet.push_str(&bypass.from)
        }
        alphabet.push(' ');

        Antiswear {
            bypasses: bypasses,
            prefixes: utils::add(
                        utils::split(self.prefixes_first),
                        utils::split(self.prefixes_second)
                      ),
            short: utils::split(self.short),
            alphabet: alphabet,
            replacements: Replacement::from_str(self.replacements),
            exceptions: utils::split(self.exceptions),
            mode: self.mode,
        }
    }
}

impl Antiswear {
    pub fn new() -> Self {
        Self {
            bypasses: Vec::new(),
            prefixes: Vec::new(),
            short: Vec::new(),
            alphabet: String::new(),
            replacements: Vec::new(),
            exceptions: Vec::new(),
            mode: Mode::Startswith,
        }
    }

    fn replace_bypasses(&self, word: &str) -> String {
        Replacement::replace(&self.bypasses, &self.replace_repeats(word))
    }

    fn replace_repeats(&self, word: &str) -> String {
        let mut out = String::new();
        let mut hold = false;
        let chars: Vec<char> = word.chars().collect();
        
        for (i, current) in word.chars().enumerate() {
            if i > 0 && chars[i-1] == current {
                hold = true;
            }
            if !hold {
                out.push(current);
            } else {
                hold = false;
            }
        }

        out
    }

    fn replace_replacements(&self, word: &str) -> String {
        Replacement::replace(&self.replacements, word)
    }

    fn is_swear(&self, word: &str) -> bool {
        let mut buffer = word.to_string();
        for char in word.chars() {
            if !self.alphabet.contains(char) {
                buffer = word.replace(char, "");
            }
        } 
        let word = &buffer;

        for except in &self.exceptions {
            if self.mode.is_contains(word, except) {
                return false
            }
        }

        for pref in &self.prefixes {
            if word.starts_with(pref) {
                return true
            }
        }

        for check in &self.short {
            match self.mode  {
                Mode::Startswith => if word == check { 
                    return true 
                },  
                _ => if self.mode.is_contains(word, check) {
                    return true;
                },
            } 
        }

        false
    }
    
    fn get(&self, word: &str) -> [String; 4] {
        let word = &word.to_lowercase();
        let replace_replacements = self.replace_replacements(&word);
        let replace_bypasses = self.replace_bypasses(&word);
        [
            replace_replacements.clone(),
            replace_bypasses.clone(),
            self.replace_replacements(&replace_bypasses),
            self.replace_bypasses(&replace_replacements),
        ]
    }
    
    /// # Example
    ///
    /// ```
    /// use antiswear_rs::Antiswear;
    /// let antiswear = Antiswear::en();
    ///
    /// assert_eq!(antiswear.check("what the fuck").is_some(), true);
    /// ```
    pub fn check<'a>(&self, text: &'a str) -> Option<Analyze>{
        let mut slice = text.trim().replace(" ", "");
        slice = {
            #[allow(unused_assignments)]
            let mut end = 0;

            let length = slice.chars().count();
            if length > 10 {
                end = 10;
            } else {
                end = length;
            }

            if let Some(r) = utils::utf8_slice(&slice, 0, end) {
                r.to_string()
            } else {
                String::new()
            }
        };
        
        for bypass in self.get(&slice) {
           if self.is_swear(&bypass) {
                return Some(
                        Analyze {
                            word: slice,
                            index: 0,
                        }
                )
           } 
        }

        for (i, word) in text.split_whitespace().enumerate() {
            for bypass in self.get(word) {
                if self.is_swear(&bypass) {
                    return Some(
                        Analyze {
                            word: word.to_string(),
                            index: i,
                        }
                    )
                }
            }
        }

        None
    }

    pub fn ru() -> Self {
        Builder {
            bypasses: "ia-я yo-е ё-е 6-б 3-з 0-о c-с p-р /\\-л",
            prefixes_first: "у ни а о вы до попере нев невъ за из изъ ис на недо надъ не о об объ от отъ по долба долбо под подъ пере пре пред предъ при про раз рас разъ съ со су через черес чрез черезъ вз взъ довы без бес долбо",
            prefixes_second: "хуе шлюх хуи хуй хую хуя пизд пезд блят бляд сук пидар пидор еб бзд пидр педр хул залуп спизд спизж пизж",
            short: "бля бл нах манда сучка мозгоеб мозгоебина",
            alphabet: "абвгдеёжзийклмнопрстуфхцчшщъыьэюяabcdefghijklmnopqrstuvwxyz ",
            replacements: "a-а b-б v-в g-г d-д e-е zh-ж z-з i-и k-к l-л m-м n-н o-о p-п r-р s-с t-т u-у f-ф h-х c-ц ch-ч sh-ш yu-ю ya-я",
            exceptions: "",
            mode: Mode::Startswith,
        }.build()
    }

    pub fn en() -> Self {
        Builder {
            bypasses: "1-i 4-f",
            prefixes_first: "",
            prefixes_second: "",
            short: "fuck bitch",
            alphabet: "abcdefghijklmnopqrstuvwxyz ",
            replacements: "",
            exceptions: "bitchin bitchy",
            mode: Mode::Contains,
        }.build()
     }
}

/// A structure for accounting for several languages at once
/// 
/// # Example
///
/// ```
/// use antiswear_rs::{AntiswearGroup, Antiswear};
///
/// let antiswear = AntiswearGroup {
///     elems: vec![
///         Antiswear::en(),
///         Antiswear::ru(),
///     ] 
/// };
///
/// assert_eq!(antiswear.check("what the fuck").is_some(), true);
/// assert_eq!(antiswear.check("блять...").is_some(), true);
/// ```
pub struct AntiswearGroup {
    pub elems: Vec<Antiswear>,
}

impl AntiswearGroup {
    pub fn check(&self, text: &str) -> Option<Analyze> {
        for antiswear in &self.elems {
            let check = antiswear.check(text);
            if check.is_some() {
                return check
            }
        }
        
        None
    }
}

pub struct Replacement {
    pub from: String,
    pub into: String,
}

impl Replacement {
    pub fn from_str(value: &str) -> Vec<Self> {
        if value == "" {
            return vec![
                Self {
                    from: String::new(),
                    into: String::new(),
                }
            ]
        }

        let mut conv: Vec<Self> = value.split(" ").map(|elem| {
            let r = utils::convert_vec(elem.split("-"));
            Self {
                from: r[0].clone(),
                into: r[1].clone(),
            }
        }).collect();

        conv.sort_by_key(|s| s.from.len());
        conv.reverse();
        conv
    }

    fn replace(vec: &Vec<Replacement>, word: &str) -> String {
        let mut out = word.to_string();

        for rep in vec {
            out = out.replace(&rep.from, &rep.into);
        }

        out
    }
}
