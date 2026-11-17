# syntax=docker/dockerfile:1
# Build & export cac binary Linux x64 (html2pdf + weasyprint) ra thu muc host.
# Cach dung:
#   docker build --platform=linux/amd64 --target export --output type=local,dest=binaries .

FROM rust:1-slim AS html2pdf-builder
WORKDIR /app
COPY packages/html2pdf/Cargo.toml packages/html2pdf/Cargo.lock ./
COPY packages/html2pdf/src ./src
RUN cargo build --release --locked \
    && cp target/release/html2pdf /tmp/html2pdf-x86_64-unknown-linux-gnu \
    && cd /tmp && sha256sum html2pdf-x86_64-unknown-linux-gnu > html2pdf-x86_64-unknown-linux-gnu.sha256

FROM python:3.12-slim AS weasyprint-builder
RUN apt-get update && apt-get install -y --no-install-recommends \
    binutils \
    libpango-1.0-0 \
    libpangocairo-1.0-0 \
    libpangoft2-1.0-0 \
    libcairo2 \
    libgdk-pixbuf-2.0-0 \
    libffi8 \
    shared-mime-info \
    fonts-dejavu-core \
    && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY packages/weasyprint-freeze/entry.py ./
RUN python3 -m pip install --no-cache-dir --upgrade pip weasyprint pyinstaller \
    && pyinstaller --noconfirm --onefile --name weasyprint \
       --collect-all weasyprint --collect-all pydyf --collect-all fontTools \
       --collect-all pyphen --collect-all PIL \
       entry.py \
    && cp dist/weasyprint /tmp/weasyprint-x86_64-unknown-linux-gnu \
    && cd /tmp && sha256sum weasyprint-x86_64-unknown-linux-gnu > weasyprint-x86_64-unknown-linux-gnu.sha256

FROM scratch AS export
COPY --from=html2pdf-builder /tmp/html2pdf-x86_64-unknown-linux-gnu /html2pdf-x86_64-unknown-linux-gnu
COPY --from=html2pdf-builder /tmp/html2pdf-x86_64-unknown-linux-gnu.sha256 /html2pdf-x86_64-unknown-linux-gnu.sha256
COPY --from=weasyprint-builder /tmp/weasyprint-x86_64-unknown-linux-gnu /weasyprint-x86_64-unknown-linux-gnu
COPY --from=weasyprint-builder /tmp/weasyprint-x86_64-unknown-linux-gnu.sha256 /weasyprint-x86_64-unknown-linux-gnu.sha256
