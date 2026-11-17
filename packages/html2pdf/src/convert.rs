// Tien xu ly HTML hoa don dien tu + goi WeasyPrint CLI xuat PDF 1 trang A4.

use qrcode::render::svg;
use qrcode::{EcLevel, QrCode};
use regex::{Captures, Regex};
use std::env;
use std::fs;
use std::io::Read;
use std::path::{self, Path};
use std::process::Command;
use tempfile::Builder;

const FIT_ONE_PAGE_CSS: &str = r#"<style>
body { font-size: 10.8pt !important; }
.main-page { padding: 10px 14px 4px !important; line-height: 1.27 !important; }
.heading-content .main-title { font-size: 15.5pt !important; }
.heading-content p, .heading-content .code-ms { font-size: 10.8pt !important; }
.content-info .list-fill-out { margin-top: 2px !important; margin-bottom: 2px !important; }
.content-info .list-fill-out li { font-size: 10.8pt !important; }
.data-item { font-size: 10.8pt !important; margin-bottom: 2px !important; }
.data-item .di-label, .data-item .di-value { min-height: 18px !important; }
.res-tb { margin-top: 5px !important; margin-bottom: 5px !important; }
.res-tb tr td, .res-tb tr th { padding: 2px 4px !important; font-size: 10.3pt !important; }
.sign-box { margin-top: 4px !important; padding: 3px !important; }
.ft-sign p, .ft-sign h3 { margin: 2px 0 !important; }
.fd-end { padding-top: 2px !important; }
.vip-divide { margin: 2px 0 !important; }
#qrcodeTable svg { width: 80px; height: 80px; }
</style>"#;

pub fn preprocess_invoice_html(input: &str) -> Result<String, String> {
    let mut html = input.to_string();

    // 1) QR: details.js lam viec nay trong browser — o day sinh SVG bang crate qrcode
    let input_re = Regex::new(r#"(?i)<input\b[^>]*\bid=["']qrcodeContent["'][^>]*>"#)
        .map_err(|e| e.to_string())?;
    let val_re = Regex::new(r#"(?i)\bvalue=(?:"([^"]*)"|'([^']*)')"#)
        .map_err(|e| e.to_string())?;

    if let Some(input_tag) = input_re.find(&html) {
        if let Some(caps) = val_re.captures(input_tag.as_str()) {
            let qr_value = caps
                .get(1)
                .or_else(|| caps.get(2))
                .map(|m| m.as_str())
                .unwrap_or("");
            if !qr_value.is_empty() {
                let code = QrCode::with_error_correction_level(qr_value.as_bytes(), EcLevel::L)
                    .map_err(|e| format!("loi tao ma QR: {e}"))?;
                let qr_svg = code
                    .render::<svg::Color>()
                    .quiet_zone(true)
                    .dark_color(svg::Color("#000000"))
                    .light_color(svg::Color("#ffffff"))
                    .build();

                let table_re = Regex::new(
                    r#"(?is)(<div\b[^>]*\bid=["']qrcodeTable["'][^>]*>)[\s\S]*?(</div>)"#,
                )
                .map_err(|e| e.to_string())?;
                html = table_re
                    .replace(&html, |caps: &Captures| {
                        format!("{}{}{}", &caps[1], qr_svg, &caps[2])
                    })
                    .into_owned();
            }
        }
    }

    // 2) Chu ky: chuoi DN -> chi giu CN
    let cks_re = Regex::new(r#"(?is)(<span\b[^>]*\bid=["']cks["'][^>]*>)([\s\S]*?)(</span>)"#)
        .map_err(|e| e.to_string())?;
    let cn_re = Regex::new(r#"(?i)CN=([^,]+)"#).map_err(|e| e.to_string())?;
    html = cks_re
        .replace(&html, |caps: &Captures| {
            let dn = &caps[2];
            let cn_or_dn = cn_re
                .captures(dn)
                .and_then(|c| c.get(1))
                .map(|m| m.as_str())
                .unwrap_or(dn);
            format!("{}{}{}", &caps[1], cn_or_dn, &caps[3])
        })
        .into_owned();

    // 3) Bo script + the an, chen CSS ep 1 trang
    let script_re = Regex::new(
        r#"(?is)<script\b[^>]*\bsrc=["'](?:\./)?details\.js["'][^>]*>\s*</script>"#,
    )
    .map_err(|e| e.to_string())?;
    loop {
        let next = script_re.replace_all(&html, "").into_owned();
        if next == html {
            break;
        }
        html = next;
    }

    html = input_re.replace_all(&html, "").into_owned();

    let head_end_re = Regex::new(r#"(?i)</head>"#).map_err(|e| e.to_string())?;
    if head_end_re.is_match(&html) {
        Ok(head_end_re
            .replace(&html, format!("{FIT_ONE_PAGE_CSS}</head>"))
            .into_owned())
    } else {
        Ok(format!("{FIT_ONE_PAGE_CSS}{html}"))
    }
}

pub fn convert_invoice_to_pdf(
    invoice_dir: &Path,
    out_pdf: &Path,
    weasyprint: &str,
) -> Result<(), String> {
    let raw = fs::read_to_string(invoice_dir.join("invoice.html"))
        .map_err(|e| format!("khong doc duoc invoice.html: {e}"))?;
    let html = preprocess_invoice_html(&raw)?;

    // Don thu muc tam work/ do ban cu de lai trong thu muc hoa don (best-effort)
    if let Ok(abs_out) = path::absolute(out_pdf) {
        if let Some(parent) = abs_out.parent() {
            let legacy_work = parent.join("work");
            let legacy_file = legacy_work.join("invoice-print.html");
            let _ = fs::remove_file(legacy_file);
            if let Ok(mut entries) = fs::read_dir(&legacy_work) {
                if entries.next().is_none() {
                    let _ = fs::remove_dir(legacy_work);
                }
            }
        }
    }

    let tmp_dir = Builder::new()
        .prefix("html2pdf-")
        .tempdir()
        .map_err(|e| format!("khong tao duoc thu muc tam: {e}"))?;
    let tmp_html = tmp_dir.path().join("invoice-print.html");
    fs::write(&tmp_html, html).map_err(|e| format!("khong ghi duoc file tam: {e}"))?;

    let abs_invoice_dir = path::absolute(invoice_dir).map_err(|e| e.to_string())?;
    let output = Command::new(weasyprint)
        .arg("--base-url")
        .arg(abs_invoice_dir)
        .arg("--encoding")
        .arg("utf-8")
        .arg(&tmp_html)
        .arg(out_pdf)
        .output()
        .map_err(|e| format!("khong chay duoc weasyprint: {e}"))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("weasyprint loi: {stderr}"));
    }

    let mut file =
        fs::File::open(out_pdf).map_err(|e| format!("khong mo duoc file PDF ra: {e}"))?;
    let mut head = [0u8; 4];
    file.read_exact(&mut head)
        .map_err(|_| "file ra khong phai PDF".to_string())?;
    if &head != b"%PDF" {
        return Err("file ra khong phai PDF".to_string());
    }

    Ok(())
}

pub fn sibling_weasyprint() -> Option<String> {
    let exe = env::current_exe().ok()?;
    let dir = exe.parent()?;
    for name in ["weasyprint.exe", "weasyprint"] {
        let candidate = dir.join(name);
        if candidate.exists() {
            return Some(candidate.to_string_lossy().into_owned());
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::preprocess_invoice_html;

    #[test]
    fn should_inject_qr_svg_strip_scripts_and_keep_cn() {
        let html = r#"<html><head></head><body>
            <input id="qrcodeContent" value="https://example.com/inv/123" />
            <div id="qrcodeTable">old</div>
            <span id="cks">CN=Cong Ty ABC, O=ABC, C=VN</span>
            <script src="./details.js"></script>
        </body></html>"#;

        let out = preprocess_invoice_html(html).expect("preprocess should succeed");
        assert!(out.contains("<svg"));
        assert!(!out.contains("qrcodeContent"));
        assert!(!out.contains("details.js"));
        assert!(out.contains("<span id=\"cks\">Cong Ty ABC</span>"));
        assert!(out.contains("#qrcodeTable svg { width: 80px; height: 80px; }"));
    }

    #[test]
    fn should_strip_nested_script_tags_in_loop() {
        let html = r#"<html><head></head><body><scr<script src="details.js"></script>ipt src="details.js"></script></body></html>"#;
        let out = preprocess_invoice_html(html).expect("preprocess should succeed");
        assert!(!out.contains("details.js"));
    }
}
