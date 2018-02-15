extern crate rand;

use rand::Rng;
use rand::thread_rng;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::iter::FromIterator;

const WALS_3: &'static str = include_str!("..\\..\\wals-3.csv");
const WALS_51: &'static str = include_str!("..\\..\\wals-51.csv");
const WALS_83: &'static str = include_str!("..\\..\\wals-83.csv");
const WALS_86: &'static str = include_str!("..\\..\\wals-86.csv");
const WALS_87: &'static str = include_str!("..\\..\\wals-87.csv");

macro_rules! enum_flags {
    ($out:expr, $test:expr, $($var:expr),*) => {
        $(
            if $test == $var {
                $out.push_str("1.0 ");
            } else {
                $out.push_str("0.0 ");
            }
        )*
    }
}

#[derive(Debug)]
struct WalsRecord<'a> {
    confidence: &'a str,
    description: &'a str,
    domainelement_pk: &'a str,
    frequency: &'a str,
    id: &'a str,
    jsondata: &'a str,
    markup_description: &'a str,
    name: &'a str,
    pk: &'a str,
    valueset_pk: &'a str,
}
impl<'a> FromIterator<&'a str> for WalsRecord<'a> {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = &'a str>,
    {
        let mut iter = iter.into_iter();
        WalsRecord {
            confidence: iter.next().unwrap(),
            description: iter.next().unwrap(),
            domainelement_pk: iter.next().unwrap(),
            frequency: iter.next().unwrap(),
            id: iter.next().unwrap(),
            jsondata: iter.next().unwrap(),
            markup_description: iter.next().unwrap(),
            name: iter.next().unwrap(),
            pk: iter.next().unwrap(),
            valueset_pk: iter.next().unwrap(),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum ConsonantVowelRatio {
    LowCVR,
    AverageCVR,
    HighCVR,
    UnknownCVR,
}
impl<'a> From<&'a str> for ConsonantVowelRatio {
    fn from(s: &'a str) -> ConsonantVowelRatio {
        match s {
            "9" | "10" => ConsonantVowelRatio::LowCVR,
            "11" => ConsonantVowelRatio::AverageCVR,
            "12" | "13" => ConsonantVowelRatio::HighCVR,
            _ => ConsonantVowelRatio::UnknownCVR,
        }
    }
}
impl ConsonantVowelRatio {
    fn to_input_str(&self) -> String {
        let mut buf = String::new();
        enum_flags!(
            buf,
            *self,
            ConsonantVowelRatio::LowCVR,
            ConsonantVowelRatio::AverageCVR,
            ConsonantVowelRatio::HighCVR,
            ConsonantVowelRatio::UnknownCVR
        );
        buf.pop();
        buf
    }

    fn to_output_str(&self) -> String {
        let mut buf = String::new();
        enum_flags!(
            buf,
            *self,
            ConsonantVowelRatio::LowCVR,
            ConsonantVowelRatio::AverageCVR,
            ConsonantVowelRatio::HighCVR
        );
        buf.pop();
        buf
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Case {
    PrefixCase,
    SuffixCase,
    NoCase,
    UnknownCase,
}
impl<'a> From<&'a str> for Case {
    fn from(s: &'a str) -> Case {
        match s {
            "254" | "259" => Case::SuffixCase,
            "255" | "260" => Case::PrefixCase,
            "262" => Case::NoCase,
            _ => Case::UnknownCase,
        }
    }
}
impl Case {
    fn to_input_str(&self) -> String {
        let mut buf = String::new();
        enum_flags!(
            buf,
            *self,
            Case::PrefixCase,
            Case::SuffixCase,
            Case::NoCase,
            Case::UnknownCase
        );
        buf.pop();
        buf
    }

    fn to_output_str(&self) -> String {
        let mut buf = String::new();
        enum_flags!(buf, *self, Case::PrefixCase, Case::SuffixCase, Case::NoCase);
        buf.pop();
        buf
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum OrderObjectVerb {
    ObjectVerbOOV,
    VerbObjectOOV,
    UnknownOOV,
}
impl<'a> From<&'a str> for OrderObjectVerb {
    fn from(s: &'a str) -> OrderObjectVerb {
        match s {
            "393" => OrderObjectVerb::ObjectVerbOOV,
            "394" => OrderObjectVerb::VerbObjectOOV,
            _ => OrderObjectVerb::UnknownOOV,
        }
    }
}
impl OrderObjectVerb {
    fn to_input_str(&self) -> String {
        let mut buf = String::new();
        enum_flags!(
            buf,
            *self,
            OrderObjectVerb::ObjectVerbOOV,
            OrderObjectVerb::VerbObjectOOV,
            OrderObjectVerb::UnknownOOV
        );
        buf.pop();
        buf
    }

    fn to_output_str(&self) -> String {
        let mut buf = String::new();
        enum_flags!(
            buf,
            *self,
            OrderObjectVerb::ObjectVerbOOV,
            OrderObjectVerb::VerbObjectOOV
        );
        buf.pop();
        buf
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum OrderGenitiveNoun {
    GenitiveNounOGN,
    NounGenitiveOGN,
    UnknownOGN,
}
impl<'a> From<&'a str> for OrderGenitiveNoun {
    fn from(s: &'a str) -> OrderGenitiveNoun {
        match s {
            "407" => OrderGenitiveNoun::GenitiveNounOGN,
            "408" => OrderGenitiveNoun::NounGenitiveOGN,
            _ => OrderGenitiveNoun::UnknownOGN,
        }
    }
}
impl OrderGenitiveNoun {
    fn to_input_str(&self) -> String {
        let mut buf = String::new();
        enum_flags!(
            buf,
            *self,
            OrderGenitiveNoun::GenitiveNounOGN,
            OrderGenitiveNoun::NounGenitiveOGN,
            OrderGenitiveNoun::UnknownOGN
        );
        buf.pop();
        buf
    }

    fn to_output_str(&self) -> String {
        let mut buf = String::new();
        enum_flags!(
            buf,
            *self,
            OrderGenitiveNoun::GenitiveNounOGN,
            OrderGenitiveNoun::NounGenitiveOGN
        );
        buf.pop();
        buf
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum OrderAdjectiveNoun {
    AdjectiveNounOAN,
    NounAdjectiveOAN,
    UnknownOAN,
}
impl<'a> From<&'a str> for OrderAdjectiveNoun {
    fn from(s: &'a str) -> OrderAdjectiveNoun {
        match s {
            "410" => OrderAdjectiveNoun::AdjectiveNounOAN,
            "411" => OrderAdjectiveNoun::NounAdjectiveOAN,
            _ => OrderAdjectiveNoun::UnknownOAN,
        }
    }
}
impl OrderAdjectiveNoun {
    fn to_input_str(&self) -> String {
        let mut buf = String::new();
        enum_flags!(
            buf,
            *self,
            OrderAdjectiveNoun::AdjectiveNounOAN,
            OrderAdjectiveNoun::NounAdjectiveOAN,
            OrderAdjectiveNoun::UnknownOAN
        );
        buf.pop();
        buf
    }

    fn to_output_str(&self) -> String {
        let mut buf = String::new();
        enum_flags!(
            buf,
            *self,
            OrderAdjectiveNoun::AdjectiveNounOAN,
            OrderAdjectiveNoun::NounAdjectiveOAN
        );
        buf.pop();
        buf
    }
}

#[derive(Default, Debug)]
struct LanguageBuilder<'a> {
    name: &'a str,
    consonant_vowel_ratio: Option<ConsonantVowelRatio>,
    case: Option<Case>,
    order_object_verb: Option<OrderObjectVerb>,
    order_genitive_noun: Option<OrderGenitiveNoun>,
    order_adjective_noun: Option<OrderAdjectiveNoun>,
}
impl<'a> LanguageBuilder<'a> {
    fn new(name: &'a str) -> LanguageBuilder<'a> {
        LanguageBuilder {
            name: name,
            ..Default::default()
        }
    }
    fn add_cvw_str(&mut self, from: &str) {
        self.consonant_vowel_ratio = Some(ConsonantVowelRatio::from(from));
    }
    fn cvw(&self) -> &Option<ConsonantVowelRatio> {
        &self.consonant_vowel_ratio
    }
    fn remove_cvw(&mut self) {
        self.consonant_vowel_ratio = None;
    }
    fn add_case_str(&mut self, from: &str) {
        self.case = Some(Case::from(from));
    }
    fn case(&self) -> &Option<Case> {
        &self.case
    }
    fn remove_case(&mut self) {
        self.case = None;
    }
    fn add_oov_str(&mut self, from: &str) {
        self.order_object_verb = Some(OrderObjectVerb::from(from));
    }
    fn oov(&self) -> &Option<OrderObjectVerb> {
        &self.order_object_verb
    }
    fn remove_oov(&mut self) {
        self.order_object_verb = None;
    }
    fn add_ogn_str(&mut self, from: &str) {
        self.order_genitive_noun = Some(OrderGenitiveNoun::from(from));
    }
    fn ogn(&self) -> &Option<OrderGenitiveNoun> {
        &self.order_genitive_noun
    }
    fn remove_ogn(&mut self) {
        self.order_genitive_noun = None;
    }
    fn add_oan_str(&mut self, from: &str) {
        self.order_adjective_noun = Some(OrderAdjectiveNoun::from(from));
    }
    fn oan(&self) -> &Option<OrderAdjectiveNoun> {
        &self.order_adjective_noun
    }
    fn remove_oan(&mut self) {
        self.order_adjective_noun = None;
    }
    fn build(&self) -> Option<Language> {
        Some(Language {
            name: self.name,
            consonant_vowel_ratio: self.consonant_vowel_ratio?,
            case: self.case?,
            order_object_verb: self.order_object_verb?,
            order_genitive_noun: self.order_genitive_noun?,
            order_adjective_noun: self.order_adjective_noun?,
        })
    }
}

#[derive(Debug, Copy, Clone)]
struct Language<'a> {
    name: &'a str,
    consonant_vowel_ratio: ConsonantVowelRatio,
    case: Case,
    order_object_verb: OrderObjectVerb,
    order_genitive_noun: OrderGenitiveNoun,
    order_adjective_noun: OrderAdjectiveNoun,
}

fn main() {
    let wals_3 = WALS_3
        .lines()
        .skip(1)
        .map(|line| line.split(',').collect::<WalsRecord>());
    let wals_51 = WALS_51
        .lines()
        .skip(1)
        .map(|line| line.split(',').collect::<WalsRecord>());
    let wals_83 = WALS_83
        .lines()
        .skip(1)
        .map(|line| line.split(',').collect::<WalsRecord>());
    let wals_86 = WALS_86
        .lines()
        .skip(1)
        .map(|line| line.split(',').collect::<WalsRecord>());
    let wals_87 = WALS_87
        .lines()
        .skip(1)
        .map(|line| line.split(',').collect::<WalsRecord>());

    let mut langmap: HashMap<&str, LanguageBuilder> = HashMap::new();
    for record in wals_3 {
        let name = &record.id[record.id.len() - 3..];
        let mut language = langmap.entry(name).or_insert(LanguageBuilder::new(name));
        language.add_cvw_str(record.domainelement_pk);
        if *language.cvw() == Some(ConsonantVowelRatio::UnknownCVR) {
            language.remove_cvw();
        }
    }
    for record in wals_51 {
        let name = &record.id[record.id.len() - 3..];
        let mut language = langmap.entry(name).or_insert(LanguageBuilder::new(name));
        language.add_case_str(record.domainelement_pk);
        if *language.case() == Some(Case::UnknownCase) {
            language.remove_case();
        }
    }
    for record in wals_83 {
        let name = &record.id[record.id.len() - 3..];
        let mut language = langmap.entry(name).or_insert(LanguageBuilder::new(name));
        language.add_oov_str(record.domainelement_pk);
        if *language.oov() == Some(OrderObjectVerb::UnknownOOV) {
            language.remove_oov();
        }
    }
    for record in wals_86 {
        let name = &record.id[record.id.len() - 3..];
        let mut language = langmap.entry(name).or_insert(LanguageBuilder::new(name));
        language.add_ogn_str(record.domainelement_pk);
        if *language.ogn() == Some(OrderGenitiveNoun::UnknownOGN) {
            language.remove_ogn();
        }
    }
    for record in wals_87 {
        let name = &record.id[record.id.len() - 3..];
        let mut language = langmap.entry(name).or_insert(LanguageBuilder::new(name));
        language.add_oan_str(record.domainelement_pk);
        if *language.oan() == Some(OrderAdjectiveNoun::UnknownOAN) {
            language.remove_oan();
        }
    }

    let mut langs: Vec<Language> = langmap
        .iter()
        .filter_map(|(_, value)| {
            let lang = value.build();
            if let Some(lang) = lang {
                Some(lang)
            } else {
                None
            }
        })
        .collect();

    let (train, valid, test) = (
        (langs.len() * 678) / 1000,
        (langs.len() * 81) / 1000,
        (langs.len() * 241) / 1000,
    );
    let test = test + (langs.len() - (train + valid + test));

    let mut trainset = Vec::new();
    let mut validset = Vec::new();
    let mut testset = Vec::new();
    let mut rng = thread_rng();

    rng.shuffle(&mut langs);

    for _ in 0..train {
        trainset.push(langs.pop().unwrap());
    }
    for _ in 0..valid {
        validset.push(langs.pop().unwrap());
    }
    for _ in 0..test {
        testset.push(langs.pop().unwrap());
    }

    let mut train_handle = File::create("usertrain.txt").unwrap();
    let mut valid_handle = File::create("uservalid.txt").unwrap();
    let mut unseen_handle = File::create("userunseen.txt").unwrap();
    write!(train_handle, "{}", set_string(&trainset));
    write!(valid_handle, "{}", set_string(&validset));
    write!(unseen_handle, "{}", set_string(&testset));
}

fn set_string(langs: &Vec<Language>) -> String {
    let mut obuf = String::new();

    // Input Count, Output Count
    obuf.push_str("17 12");
    // Input Format Specifier
    obuf.push_str(" %.2f");
    // Output Format Specifier
    obuf.push_str(" %.2f");
    // SSE Format Specifier
    obuf.push_str(" %.4f");
    obuf.push_str("\r\n");

    // Names
    obuf.push_str("LowCVR AverageCVR HighCVR UnknownCVR PrefixCase SuffixCase NoCase UnknownCase ObjectVerbOOV VerbObjectOOV UnknownOOV GenitiveNounOGN NounGenitiveOGN UnknownOGN AdjectiveNounOAN NounAdjectiveOAN UnknownOAN LowCVR AverageCVR HighCVR PrefixCase SuffixCase NoCase ObjectVerbOOV VerbObjectOOV GenitiveNounOGN NounGenitiveOGN AdjectiveNounOAN NounAdjectiveOAN\r\n");

    // Input Minimum Values
    obuf.push_str("0");
    for _ in 1..17 {
        obuf.push_str(" 0");
    }
    // Output Minimum Values
    for _ in 0..12 {
        obuf.push_str(" 0");
    }
    obuf.push_str("\r\n");

    // Input Maximum Values
    obuf.push_str("1");
    for _ in 1..17 {
        obuf.push_str(" 1");
    }
    // Output Maximum Values
    for _ in 0..12 {
        obuf.push_str(" 1");
    }
    obuf.push_str("\r\n");
    lang_set(langs, &mut obuf);
    obuf.pop();
    obuf.pop();
    obuf
}

fn lang_set(langs: &Vec<Language>, obuf: &mut String) {
    for lang in langs {
        let mut lang = lang.clone();
        dump_line(&lang, obuf);
        let consonant_vowel_ratio = lang.consonant_vowel_ratio;
        lang.consonant_vowel_ratio = ConsonantVowelRatio::UnknownCVR;
        dump_line(&lang, obuf);
        lang.consonant_vowel_ratio = consonant_vowel_ratio;
        let case = lang.case;
        lang.case = Case::UnknownCase;
        dump_line(&lang, obuf);
        lang.case = case;
        let order_object_verb = lang.order_object_verb;
        lang.order_object_verb = OrderObjectVerb::UnknownOOV;
        dump_line(&lang, obuf);
        lang.order_object_verb = order_object_verb;
        let order_genitive_noun = lang.order_genitive_noun;
        lang.order_genitive_noun = OrderGenitiveNoun::UnknownOGN;
        dump_line(&lang, obuf);
        lang.order_genitive_noun = order_genitive_noun;
        let order_adjective_noun = lang.order_adjective_noun;
        lang.order_adjective_noun = OrderAdjectiveNoun::UnknownOAN;
        dump_line(&lang, obuf);
    }
}

fn dump_line(lang: &Language, obuf: &mut String) {
        obuf.push_str(&lang.consonant_vowel_ratio.to_input_str());
        obuf.push_str(" ");
        obuf.push_str(&lang.case.to_input_str());
        obuf.push_str(" ");
        obuf.push_str(&lang.order_object_verb.to_input_str());
        obuf.push_str(" ");
        obuf.push_str(&lang.order_genitive_noun.to_input_str());
        obuf.push_str(" ");
        obuf.push_str(&lang.order_adjective_noun.to_input_str());
        obuf.push_str(" ");
        obuf.push_str(&lang.consonant_vowel_ratio.to_output_str());
        obuf.push_str(" ");
        obuf.push_str(&lang.case.to_output_str());
        obuf.push_str(" ");
        obuf.push_str(&lang.order_object_verb.to_output_str());
        obuf.push_str(" ");
        obuf.push_str(&lang.order_genitive_noun.to_output_str());
        obuf.push_str(" ");
        obuf.push_str(&lang.order_adjective_noun.to_output_str());
        obuf.push_str("\r\n");
}