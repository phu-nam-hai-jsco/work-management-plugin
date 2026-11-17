// Entry cua sidecar exe (Rust binary doc lap).
// Giao dien cho Tauri:
//   html2pdf <thu-muc-hoa-don> <file-pdf-ra> [--weasyprint <duong-dan>]
// Thanh cong: in 1 dong JSON ra stdout {"ok":true,"pdf":"...","bytes":N,"ms":M}, exit 0
// Loi:       in {"ok":false,"error":"..."} ra stderr, exit 1

mod convert;

use serde::Serialize;
use std::env;
use std::fs;
use std::path;
use std::process;
use std::time::Instant;

#[derive(Serialize)]
struct PingResponse<'a> {
    ok: bool,
    app: &'a str,
    version: &'a str,
}

#[derive(Serialize)]
struct SuccessResponse {
    ok: bool,
    pdf: String,
    bytes: u64,
    ms: u128,
}

#[derive(Serialize)]
struct ErrorResponse {
    ok: bool,
    error: String,
}

fn parse_args(args: &[String]) -> (Option<String>, Option<String>, Option<String>) {
    let mut positional: Vec<String> = Vec::new();
    let mut weasyprint: Option<String> = None;
    let mut i = 0;

    while i < args.len() {
        if args[i] == "--weasyprint" {
            if i + 1 < args.len() {
                weasyprint = Some(args[i + 1].clone());
                i += 2;
                continue;
            }
        } else {
            positional.push(args[i].clone());
        }
        i += 1;
    }

    let invoice_dir = positional.first().cloned();
    let out_pdf = positional.get(1).cloned();
    (invoice_dir, out_pdf, weasyprint)
}

fn run() -> Result<SuccessResponse, String> {
    let t0 = Instant::now();
    let args: Vec<String> = env::args().skip(1).collect();

    if args.first().map(String::as_str) == Some("--ping") {
        let ping = PingResponse {
            ok: true,
            app: "html2pdf",
            version: env!("CARGO_PKG_VERSION"),
        };
        println!(
            "{}",
            serde_json::to_string(&ping).map_err(|e| e.to_string())?
        );
        process::exit(0);
    }

    let (invoice_dir, out_pdf, weasyprint_flag) = parse_args(&args);
    let invoice_dir = invoice_dir.filter(|s| !s.is_empty());
    let out_pdf = out_pdf.filter(|s| !s.is_empty());

    let (Some(invoice_dir), Some(out_pdf)) = (invoice_dir, out_pdf) else {
        return Err(
            "thieu tham so: html2pdf <thu-muc-hoa-don> <file-pdf-ra> [--weasyprint <path>]"
                .to_string(),
        );
    };

    let weasyprint = weasyprint_flag
        .filter(|s| !s.is_empty())
        .or_else(|| env::var("WEASYPRINT_PATH").ok().filter(|s| !s.is_empty()))
        .or_else(convert::sibling_weasyprint)
        .unwrap_or_else(|| "weasyprint".to_string());

    convert::convert_invoice_to_pdf(
        path::Path::new(&invoice_dir),
        path::Path::new(&out_pdf),
        &weasyprint,
    )?;

    let abs = path::absolute(&out_pdf).map_err(|e| e.to_string())?;
    let bytes = fs::metadata(&abs).map_err(|e| e.to_string())?.len();

    Ok(SuccessResponse {
        ok: true,
        pdf: abs.to_string_lossy().into_owned(),
        bytes,
        ms: t0.elapsed().as_millis(),
    })
}

fn main() {
    match run() {
        Ok(res) => {
            if let Ok(json) = serde_json::to_string(&res) {
                println!("{json}");
            }
        }
        Err(err) => {
            let payload = ErrorResponse {
                ok: false,
                error: err,
            };
            if let Ok(json) = serde_json::to_string(&payload) {
                eprintln!("{json}");
            } else {
                eprintln!("{{\"ok\":false,\"error\":\"unknown error\"}}");
            }
            process::exit(1);
        }
    }
}
