// Entry cua sidecar exe (Bun compile thanh 1 file chay doc lap).
// Giao dien cho Tauri:
//   html2pdf <thu-muc-hoa-don> <file-pdf-ra> [--weasyprint <duong-dan>]
// Thanh cong: in 1 dong JSON ra stdout {"ok":true,"pdf":"...","bytes":N}, exit 0
// Loi:       in {"ok":false,"error":"..."} ra stderr, exit 1
import path from 'node:path';
import { convertInvoiceToPdf, siblingWeasyprint } from './convert.ts';

const args = Bun.argv.slice(2);
if (args[0] === '--ping') {
  console.log(JSON.stringify({ ok: true, app: 'html2pdf', version: '1.0.0' }));
  process.exit(0);
}

const t0 = Date.now();
const [invoiceDir, outPdf] = args;
const wi = args.indexOf('--weasyprint');
const weasyprint =
  (wi >= 0 && args[wi + 1]) || Bun.env.WEASYPRINT_PATH || siblingWeasyprint() || 'weasyprint';

try {
  if (!invoiceDir || !outPdf) {
    throw new Error('thieu tham so: html2pdf <thu-muc-hoa-don> <file-pdf-ra> [--weasyprint <path>]');
  }
  await convertInvoiceToPdf(invoiceDir, outPdf, weasyprint);
  const abs = path.resolve(outPdf);
  const stat = await Bun.file(abs).stat();
  console.log(
    JSON.stringify({ ok: true, pdf: abs, bytes: stat.size, ms: Date.now() - t0 }),
  );
} catch (error) {
  const message = error instanceof Error ? error.message : String(error);
  console.error(JSON.stringify({ ok: false, error: message }));
  process.exit(1);
}
