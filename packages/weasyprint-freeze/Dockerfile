# syntax=docker/dockerfile:1
FROM python:3.12-slim AS builder

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
COPY entry.py ./

RUN python3 -m pip install --no-cache-dir --upgrade pip weasyprint pyinstaller \
    && pyinstaller --noconfirm --onefile --name weasyprint \
       --collect-all weasyprint --collect-all pydyf --collect-all fontTools \
       --collect-all pyphen --collect-all PIL \
       entry.py \
    && cp dist/weasyprint /tmp/weasyprint-x86_64-unknown-linux-gnu

FROM scratch AS export
COPY --from=builder /tmp/weasyprint-x86_64-unknown-linux-gnu /weasyprint-x86_64-unknown-linux-gnu
