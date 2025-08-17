# syntax=docker/dockerfile:1

FROM node:22.18.0-trixie-slim AS frontend-builder

WORKDIR /app

COPY webpage/ ./webpage/

WORKDIR /app/webpage

RUN npm install
RUN npm run build

FROM python:3.10-slim AS backend-builder

WORKDIR /app

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    curl \
    build-essential \
    && rm -rf /var/lib/apt/lists/*

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

COPY webserver/ ./webserver/
COPY grape/ ./grape/

WORKDIR /app/webserver

RUN cargo build --release

FROM python:3.10-slim

WORKDIR /app

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY requirements.txt .
RUN pip install --no-cache-dir -r requirements.txt

COPY grape/ ./grape/

COPY --from=backend-builder /app/webserver/target/release/grape-webserver /usr/local/bin/

COPY --from=frontend-builder /app/webpage/dist/ ./webpage/dist/

EXPOSE 12358

CMD ["grape-webserver", "/app/", "0.0.0.0:12358", "2"]