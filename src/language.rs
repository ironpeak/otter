use std::collections::HashSet;

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub enum Language {
    CSharp,
    Go,
    JavaScript,
    Python,
    Rust,
}

pub(crate) fn get_languages() -> HashSet<Language> {
    let mut languages = HashSet::new();
    languages.insert(Language::CSharp);
    languages.insert(Language::Go);
    languages.insert(Language::JavaScript);
    languages.insert(Language::Python);
    languages.insert(Language::Rust);
    languages
}
