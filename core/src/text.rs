//! Découpage en mots, racinisation française et extraits.
//! Tout se passe en mémoire, sans réseau.

use rust_stemmers::{Algorithm, Stemmer};
use std::{collections::HashSet, sync::OnceLock};

/// Marqueurs autour des mots trouvés dans un extrait (caractères de contrôle,
/// faciles à remplacer par du gras, du surlignage, etc.).
pub const HL_START: char = '\u{1}';
pub const HL_END: char = '\u{2}';

const STOP: &[&str] = &[
    "le", "la", "les", "un", "une", "des", "du", "de", "et", "ou", "en", "au", "aux", "ce", "ces",
    "cet", "cette", "mon", "ma", "mes", "ton", "ta", "tes", "son", "sa", "ses", "notre", "nos",
    "votre", "vos", "leur", "leurs", "je", "tu", "il", "elle", "on", "nous", "vous", "ils",
    "elles", "qui", "que", "quoi", "dont", "où", "ne", "pas", "plus", "sur", "sous", "dans", "par",
    "pour", "avec", "sans", "chez", "est", "sont", "été", "être", "avoir", "ai", "as", "avons",
    "ont", "se", "ça", "cela", "ceci", "mais", "donc", "car", "ni", "si", "the", "of", "and", "to",
    "in", "is", "for", "on", "with",
];

/// Mots ignorés seulement dans une requête (pas à l'indexation).
const QUERY_STOP: &[&str] = &[
    "fichier",
    "fichiers",
    "dossier",
    "dossiers",
    "document",
    "documents",
    "cherche",
    "chercher",
    "trouve",
    "trouver",
    "retrouve",
    "retrouver",
    "moi",
    "veux",
    "besoin",
];

fn stop_set() -> &'static HashSet<&'static str> {
    static S: OnceLock<HashSet<&'static str>> = OnceLock::new();
    S.get_or_init(|| STOP.iter().copied().collect())
}

fn query_stop_set() -> &'static HashSet<&'static str> {
    static S: OnceLock<HashSet<&'static str>> = OnceLock::new();
    S.get_or_init(|| QUERY_STOP.iter().copied().collect())
}

pub fn is_stop(w: &str) -> bool {
    w.chars().count() < 2 || stop_set().contains(w)
}

pub struct Lang {
    stemmer: Stemmer,
}

impl Lang {
    pub fn new() -> Self {
        Lang {
            stemmer: Stemmer::create(Algorithm::French),
        }
    }

    /// Le pluriel est retiré avant la racinisation : le stemmer français donne
    /// sinon des racines différentes (loyer -> loi, loyers -> loyer).
    pub fn stem(&self, lower_word: &str) -> String {
        let w = if lower_word.chars().count() > 3 {
            lower_word
                .strip_suffix('s')
                .or_else(|| lower_word.strip_suffix('x'))
                .unwrap_or(lower_word)
        } else {
            lower_word
        };
        self.stemmer.stem(w).into_owned()
    }
}

impl Default for Lang {
    fn default() -> Self {
        Self::new()
    }
}

/// Parcourt les mots d'un texte. La fonction reçoit (début, fin) en octets et
/// renvoie `false` pour arrêter le parcours.
pub fn for_each_word(text: &str, mut f: impl FnMut(usize, usize) -> bool) {
    let mut start: Option<usize> = None;
    for (i, c) in text.char_indices() {
        if c.is_alphanumeric() {
            if start.is_none() {
                start = Some(i);
            }
        } else if let Some(s) = start.take() {
            if !f(s, i) {
                return;
            }
        }
    }
    if let Some(s) = start {
        f(s, text.len());
    }
}

/// Texte réduit à ses racines, séparées par des espaces, sans mots vides.
pub fn stem_text(lang: &Lang, text: &str) -> String {
    let mut out = String::with_capacity(text.len() / 2);
    for_each_word(text, |s, e| {
        let w = text[s..e].to_lowercase();
        if is_stop(&w) || w.chars().count() > 40 {
            return true;
        }
        if !out.is_empty() {
            out.push(' ');
        }
        out.push_str(&lang.stem(&w));
        true
    });
    out
}

/// Nom de fichier sans extension, réduit à ses mots en minuscules.
pub fn name_words(name: &str) -> String {
    let stem = match name.rfind('.') {
        Some(i) if i > 0 => &name[..i],
        _ => name,
    };
    let mut out = String::new();
    for_each_word(stem, |s, e| {
        if !out.is_empty() {
            out.push(' ');
        }
        out.push_str(&stem[s..e].to_lowercase());
        true
    });
    out
}

/// Retire les accents courants pour comparer sans en tenir compte.
pub fn fold(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            'à' | 'â' | 'ä' | 'á' | 'ã' => 'a',
            'é' | 'è' | 'ê' | 'ë' => 'e',
            'î' | 'ï' | 'í' => 'i',
            'ô' | 'ö' | 'ó' | 'õ' => 'o',
            'ù' | 'û' | 'ü' | 'ú' => 'u',
            'ç' => 'c',
            'ñ' => 'n',
            'ÿ' => 'y',
            _ => c,
        })
        .collect()
}

pub struct Query {
    pub terms: Vec<String>,
    pub stems: Vec<String>,
    folded: Vec<String>,
}

pub fn parse_query(lang: &Lang, q: &str) -> Query {
    let mut all: Vec<String> = Vec::new();
    for_each_word(q, |s, e| {
        all.push(q[s..e].to_lowercase());
        true
    });
    let mut terms: Vec<String> = all
        .iter()
        .filter(|w| !is_stop(w) && !query_stop_set().contains(w.as_str()))
        .cloned()
        .collect();
    if terms.is_empty() {
        terms = all
            .iter()
            .filter(|w| w.chars().count() >= 2)
            .cloned()
            .collect();
    }
    let mut seen = HashSet::new();
    terms.retain(|t| seen.insert(t.clone()));
    let stems: Vec<String> = terms.iter().map(|t| lang.stem(t)).collect();
    let folded = stems.iter().map(|s| fold(s)).collect();
    Query {
        terms,
        stems,
        folded,
    }
}

/// Expression FTS5 : chaque mot doit apparaître (ou l'un d'eux avec "OR"),
/// dans le nom du fichier ou dans son contenu, en tolérant le pluriel,
/// les accents et la saisie partielle.
pub fn fts_expr(q: &Query, op: &str) -> String {
    q.terms
        .iter()
        .zip(q.stems.iter())
        .map(|(t, s)| format!("(name:\"{t}\"* OR stems:\"{s}\"*)"))
        .collect::<Vec<_>>()
        .join(&format!(" {op} "))
}

fn word_matches(lang: &Lang, w: &str, q: &Query) -> bool {
    let st = fold(&lang.stem(&w.to_lowercase()));
    q.folded.iter().any(|qs| st.starts_with(qs.as_str()))
}

fn floor_boundary(s: &str, mut i: usize) -> usize {
    i = i.min(s.len());
    while !s.is_char_boundary(i) {
        i -= 1;
    }
    i
}

fn ceil_boundary(s: &str, mut i: usize) -> usize {
    i = i.min(s.len());
    while !s.is_char_boundary(i) {
        i += 1;
    }
    i
}

/// Extrait du texte autour du premier mot trouvé, avec les mots trouvés
/// encadrés par HL_START / HL_END.
pub fn make_snippet(lang: &Lang, body: &str, q: &Query) -> Option<String> {
    let body = &body[..floor_boundary(body, 3_000_000)];
    let mut found: Option<(usize, usize)> = None;
    for_each_word(body, |s, e| {
        if word_matches(lang, &body[s..e], q) {
            found = Some((s, e));
            false
        } else {
            true
        }
    });
    let (ms, me) = found?;
    let from = floor_boundary(body, ms.saturating_sub(90));
    let to = ceil_boundary(body, me + 220);
    let mut win = String::new();
    let mut last_space = true;
    for c in body[from..to].chars() {
        if c.is_whitespace() {
            if !last_space {
                win.push(' ');
                last_space = true;
            }
        } else {
            win.push(c);
            last_space = false;
        }
    }
    let mut out = String::new();
    let mut pos = 0;
    for_each_word(&win, |s, e| {
        if word_matches(lang, &win[s..e], q) {
            out.push_str(&win[pos..s]);
            out.push(HL_START);
            out.push_str(&win[s..e]);
            out.push(HL_END);
            pos = e;
        }
        true
    });
    out.push_str(&win[pos..]);
    let mut out = out.trim().to_string();
    if from > 0 {
        out.insert(0, '…');
    }
    if to < body.len() {
        out.push('…');
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stems_plural_and_case() {
        let l = Lang::new();
        assert_eq!(
            stem_text(&l, "Les CONTRATS de location"),
            stem_text(&l, "contrat location")
        );
    }

    #[test]
    fn query_drops_filler_words() {
        let l = Lang::new();
        let q = parse_query(&l, "mon contrat de location de l'an dernier");
        assert!(q.terms.contains(&"contrat".to_string()));
        assert!(q.terms.contains(&"location".to_string()));
        assert!(!q.terms.contains(&"de".to_string()));
        assert!(!q.terms.contains(&"mon".to_string()));
    }

    #[test]
    fn snippet_is_found_without_accents() {
        let l = Lang::new();
        let q = parse_query(&l, "eleve");
        let s = make_snippet(&l, "Le dossier de l'élève Martin est complet.", &q).unwrap();
        assert!(s.contains(HL_START));
        assert!(s.contains("élève"));
    }

    #[test]
    fn plural_and_singular_share_a_stem() {
        let l = Lang::new();
        for (a, b) in [
            ("loyer", "loyers"),
            ("contrat", "contrats"),
            ("crêpe", "crêpes"),
        ] {
            assert_eq!(l.stem(a), l.stem(b), "{a} / {b}");
        }
    }

    #[test]
    fn name_words_split() {
        assert_eq!(name_words("Facture_Darty-2023.pdf"), "facture darty 2023");
    }
}
