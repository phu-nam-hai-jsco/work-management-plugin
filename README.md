# work-management-plugin

Bộ công cụ chuyển HTML hóa đơn điện tử sang PDF (1 trang A4) cho ứng dụng **Quản Lý Công Việc** (`work-management`):

- **`html2pdf.exe`** (đổi tên từ `invoice-pdf.exe`): tiền xử lý `invoice.html` (sinh mã QR SVG nội tuyến, rút gọn chữ ký số `CN=...`, ép khổ 1 trang A4) và gọi `weasyprint`.
- **`weasyprint.exe`**: bản PyInstaller onefile kèm sẵn toàn bộ thư viện GTK / Pango / Cairo cho Windows x64.

## Tải tệp thực thi

Tải trực tiếp tại mục **Releases** (`v1.0.0`):
- `html2pdf.exe` / `html2pdf-x86_64-pc-windows-msvc.exe`
- `weasyprint.exe` / `weasyprint-x86_64-pc-windows-msvc.exe`
- Kèm các file `.sha256` để kiểm tra tính toàn vẹn.

## Checksums (SHA-256)

| Tệp | Kích thước | SHA-256 |
| --- | --- | --- |
| `html2pdf.exe` (`html2pdf-x86_64-pc-windows-msvc.exe`) | 115,404,288 bytes | `3d497ac3da7fb5ff4cf9a7740cc5eb74e283719c3b6cd673c47da2f1a3379949` |
| `weasyprint.exe` (`weasyprint-x86_64-pc-windows-msvc.exe`) | 40,335,113 bytes | `4c05e4efc531f942bb5cfe33196d7cf1330621d7b38295bb9d2e283a77ff49c1` |

## Bun (Windows x64)

Bản phát hành Bun được đính kèm sẵn vào release dưới dạng file `.exe` để app desktop kéo về:

| Mục trong `manifest.json` | Tệp | Nguồn |
| --- | --- | --- |
| `bun` | `bun.exe` / `bun-x86_64-pc-windows-msvc.exe` | `bun-windows-x64.zip` |
| `bun-baseline` | `bun-baseline.exe` / `bun-baseline-x86_64-pc-windows-msvc.exe` | `bun-windows-x64-baseline.zip` (CPU không có AVX2) |

Cập nhật: chạy workflow **Đính kèm Bun (Windows x64)** (`.github/workflows/attach-bun.yml`, *Run workflow*), chọn `bun-version` (`latest` hoặc vd `1.4.4`). Workflow tải zip từ cùng nguồn với action `setup-bun` của repo `bun`, đối chiếu `SHASUMS256.txt`, chạy thử `bun.exe --version`, đính file vào release (`releaseTag` trong `manifest.json`) rồi commit `version` / `sha256` / `size` vào `manifest.json` và `checksums/`.
