# ALICE-Zip-Cloud

Cloud compression SaaS with multi-algorithm support and batch processing.

## Architecture

```
Frontend (Next.js :3000)
        |
        v
API Gateway (:8081)
        |
   +----+----+----+
   |    |    |    |
Compress Decomp Analyze Batch
   |    |    |    |
   +----+----+----+
           |
    Algorithm Router
   /    |    |    \
zstd  lz4 brotli gzip  xz
           |
    Entropy Analyzer
    (Algorithm Advisor)
```

## Features

| Feature | Description |
|---------|-------------|
| Multi-Algorithm | zstd, lz4, brotli, gzip, xz with tunable levels |
| Smart Analysis | Entropy-based algorithm recommendation engine |
| Batch Processing | Parallel multi-file compression with error isolation |
| Checksum Support | Integrity verification on compress and decompress |
| Throughput Stats | Real-time compression ratio and speed metrics |

## API Endpoints

| Method | Path | Description |
|--------|------|-------------|
| GET | /health | Health check |
| POST | /api/v1/zip/compress | Compress a base64-encoded payload |
| POST | /api/v1/zip/decompress | Decompress a base64-encoded payload |
| POST | /api/v1/zip/analyze | Analyze data and recommend algorithm |
| POST | /api/v1/zip/batch | Batch compress/decompress multiple items |
| GET | /api/v1/zip/algorithms | List supported algorithms and capabilities |
| GET | /api/v1/zip/stats | Compression throughput and ratio statistics |

## Quick Start

```bash
docker compose up -d
# API:      http://localhost:8081
# Frontend: http://localhost:3000
```

## License

AGPL-3.0-or-later
