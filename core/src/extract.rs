//! Text extraction from files, locally, without network.
//! Formats: plain text, Markdown, CSV, HTML, PDF (with text), Word, PowerPoint,
//! Excel and OpenDocument. No OCR in this version: scans and images are found
//! by their name only.

use std::{fs::File, io::Read, panic, path::Path};
use zip::ZipArchive;

/// Maximum text kept per file.
const MAX_TEXT: usize = 3_000_000;
/// Beyond this size, the content is not read (the name stays indexed).
pub const MAX_FILE_BYTES: u64 = 150 * 1024 * 1024;

pub fn is_readable(ext: &str) -> bool {
    matches!(
        ext,
        "txt"
            | "md"
            | "markdown"
            | "csv"
            | "tsv"
            | "log"
            | "rst"
            | "html"
            | "htm"
            | "pdf"
            | "docx"
            | "pptx"
            | "xlsx"
            | "odt"
            | "ods"
            | "odp"
    )
}

/// Returns None if the file is unreadable or corrupt.
pub fn extract_text(path: &Path, ext: &str) -> Option<String> {
    let result = panic::catch_unwind(|| extract_inner(path, ext));
    let mut text = result.ok().flatten()?;
    truncate(&mut text, MAX_TEXT);
    Some(text)
}

fn extract_inner(path: &Path, ext: &str) -> Option<String> {
    match ext {
        "txt" | "md" | "markdown" | "csv" | "tsv" | "log" | "rst" => read_limited(path),
        "html" | "htm" => read_limited(path).map(|s| strip_html(&s)),
        "pdf" => pdf_extract::extract_text(path).ok(),
        "docx" => zip_text(path, |n| n == "word/document.xml"),
        "pptx" => zip_text(path, |n| {
            n.starts_with("ppt/slides/slide") && n.ends_with(".xml")
        }),
        "xlsx" => zip_text(path, |n| n == "xl/sharedStrings.xml"),
        "odt" | "ods" | "odp" => zip_text(path, |n| n == "content.xml"),
        _ => None,
    }
}

fn read_limited(path: &Path) -> Option<String> {
    let mut buf = Vec::new();
    File::open(path)
        .ok()?
        .take((MAX_TEXT * 2) as u64)
        .read_to_end(&mut buf)
        .ok()?;
    Some(String::from_utf8_lossy(&buf).into_owned())
}

fn zip_text(path: &Path, wanted: impl Fn(&str) -> bool) -> Option<String> {
    let mut zip = ZipArchive::new(File::open(path).ok()?).ok()?;
    let mut names: Vec<String> = (0..zip.len())
        .filter_map(|i| zip.by_index(i).ok().map(|e| e.name().to_string()))
        .filter(|n| wanted(n))
        .collect();
    names.sort();
    let mut out = String::new();
    for name in names {
        let mut raw = Vec::new();
        zip.by_name(&name)
            .ok()?
            .take((MAX_TEXT * 4) as u64)
            .read_to_end(&mut raw)
            .ok()?;
        out.push_str(&xml_to_text(&String::from_utf8_lossy(&raw)));
        out.push('\n');
        if out.len() > MAX_TEXT {
            break;
        }
    }
    Some(out)
}

fn tag_kind(tag: &str) -> u8 {
    let first = tag
        .split_whitespace()
        .next()
        .unwrap_or("")
        .trim_end_matches('/');
    match first {
        "/w:p" | "/a:p" | "/si" | "/text:p" | "/text:h" | "w:br" | "/table:table-row" => 1,
        "w:tab" | "/w:tc" | "/table:table-cell" => 2,
        _ => 0,
    }
}

fn xml_to_text(xml: &str) -> String {
    let mut out = String::with_capacity(xml.len() / 3);
    let mut rest = xml;
    while let Some(i) = rest.find('<') {
        out.push_str(&rest[..i]);
        let after = &rest[i + 1..];
        match after.find('>') {
            Some(j) => {
                match tag_kind(&after[..j]) {
                    1 => out.push('\n'),
                    2 => out.push(' '),
                    _ => {}
                }
                rest = &after[j + 1..];
            }
            None => {
                rest = "";
                break;
            }
        }
    }
    out.push_str(rest);
    decode_entities(&out)
}

fn strip_html(html: &str) -> String {
    let lower = html.to_ascii_lowercase(); // same length in bytes as the original
    let mut out = String::with_capacity(html.len() / 3);
    let mut pos = 0;
    while let Some(rel) = html[pos..].find('<') {
        let i = pos + rel;
        out.push_str(&html[pos..i]);
        let Some(rel_end) = html[i..].find('>') else {
            pos = html.len();
            break;
        };
        let end = i + rel_end;
        let tag = lower[i + 1..end]
            .split_whitespace()
            .next()
            .unwrap_or("")
            .trim_end_matches('/');
        match tag {
            "script" | "style" => {
                let close = format!("</{tag}");
                pos = match lower[end..].find(&close) {
                    Some(c) => end + c,
                    None => html.len(),
                };
                continue;
            }
            "p" | "/p" | "div" | "/div" | "br" | "li" | "/li" | "tr" | "/tr" | "h1" | "h2"
            | "h3" | "h4" | "h5" | "h6" | "/h1" | "/h2" | "/h3" | "/h4" | "/h5" | "/h6" => {
                out.push('\n')
            }
            _ => {}
        }
        pos = end + 1;
    }
    if pos < html.len() {
        out.push_str(&html[pos..]);
    }
    decode_entities(&out)
}

fn decode_entities(s: &str) -> String {
    if !s.contains('&') {
        return s.to_string();
    }
    let mut out = String::with_capacity(s.len());
    let mut rest = s;
    while let Some(i) = rest.find('&') {
        out.push_str(&rest[..i]);
        let after = &rest[i + 1..];
        match after.find(';').filter(|&j| j <= 10) {
            Some(j) => {
                let ent = &after[..j];
                let decoded = match ent {
                    "amp" => Some('&'),
                    "lt" => Some('<'),
                    "gt" => Some('>'),
                    "quot" => Some('"'),
                    "apos" => Some('\''),
                    "nbsp" => Some(' '),
                    _ if ent.starts_with("#x") || ent.starts_with("#X") => {
                        u32::from_str_radix(&ent[2..], 16)
                            .ok()
                            .and_then(char::from_u32)
                    }
                    _ if ent.starts_with('#') => {
                        ent[1..].parse::<u32>().ok().and_then(char::from_u32)
                    }
                    _ => None,
                };
                match decoded {
                    Some(c) => {
                        out.push(c);
                        rest = &after[j + 1..];
                    }
                    None => {
                        out.push('&');
                        rest = after;
                    }
                }
            }
            None => {
                out.push('&');
                rest = after;
            }
        }
    }
    out.push_str(rest);
    out
}

fn truncate(s: &mut String, max: usize) {
    if s.len() <= max {
        return;
    }
    let mut i = max;
    while !s.is_char_boundary(i) {
        i -= 1;
    }
    s.truncate(i);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn xml_paragraphs_and_entities() {
        let xml = "<w:p><w:r><w:t>Loyer &amp; charges</w:t></w:r></w:p><w:p><w:r><w:t>Bail</w:t></w:r></w:p>";
        let t = xml_to_text(xml);
        assert!(t.contains("Loyer & charges\nBail"));
    }

    #[test]
    fn html_skips_scripts() {
        let t = strip_html(
            "<html><script>var x=1;</script><body><p>Bonjour &eacute;</p></body></html>",
        );
        assert!(t.contains("Bonjour"));
        assert!(!t.contains("var x"));
    }
}
