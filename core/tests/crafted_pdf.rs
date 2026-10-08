//! RUSTSEC-2026-0187: a deeply nested PDF used to overflow the stack in lopdf. A stack overflow
//! aborts the whole process instead of panicking, so the extractor's `catch_unwind` cannot stop it.

use blume_finder_core::extract::extract_text;
use std::fs;

fn pdf_with_nested_array(depth: usize) -> Vec<u8> {
    let nested = format!("{}{}", "[".repeat(depth), "]".repeat(depth));
    let objects = [
        format!("<< /Type /Catalog /Pages 2 0 R /X {nested} >>"),
        "<< /Type /Pages /Kids [] /Count 0 >>".to_string(),
    ];

    let mut pdf = b"%PDF-1.4\n".to_vec();
    let mut offsets = Vec::new();
    for (number, body) in (1..).zip(&objects) {
        offsets.push(pdf.len());
        pdf.extend_from_slice(format!("{number} 0 obj\n{body}\nendobj\n").as_bytes());
    }

    let xref = pdf.len();
    let size = objects.len() + 1;
    let mut tail = format!("xref\n0 {size}\n0000000000 65535 f \n");
    for offset in offsets {
        tail += &format!("{offset:010} 00000 n \n");
    }
    tail += &format!("trailer\n<< /Size {size} /Root 1 0 R >>\nstartxref\n{xref}\n%%EOF\n");
    pdf.extend_from_slice(tail.as_bytes());
    pdf
}

#[test]
fn deeply_nested_pdf_does_not_abort_the_process() {
    let path = std::env::temp_dir().join(format!("blume-nested-{}.pdf", std::process::id()));
    fs::write(&path, pdf_with_nested_array(100_000)).unwrap();
    let text = extract_text(&path, "pdf");
    let _ = fs::remove_file(&path);
    assert!(text.unwrap_or_default().trim().is_empty());
}
