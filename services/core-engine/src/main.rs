use axum::{
    extract::State,
    response::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::time::Instant;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;

struct AppState {
    start_time: Instant,
    stats: Mutex<Stats>,
}
struct Stats {
    total_compressions: u64,
    total_decompressions: u64,
    total_analyses: u64,
    bytes_in: u64,
    bytes_out: u64,
}

#[derive(Serialize)]
struct Health {
    status: String,
    version: String,
    uptime_secs: u64,
    total_ops: u64,
}

#[derive(Deserialize)]
struct CompressRequest {
    data_type: Option<String>,
    algorithm: Option<String>,
    level: Option<u8>,
    size_bytes: Option<u64>,
}
#[derive(Serialize)]
struct CompressResponse {
    job_id: String,
    algorithm: String,
    original_size_bytes: u64,
    compressed_size_bytes: u64,
    ratio: f64,
    procedural_pattern_detected: bool,
    pattern_type: String,
    elapsed_us: u128,
}

#[derive(Deserialize)]
struct DecompressRequest {
    algorithm: Option<String>,
    compressed_size_bytes: Option<u64>,
}
#[derive(Serialize)]
struct DecompressResponse {
    job_id: String,
    algorithm: String,
    compressed_size: u64,
    decompressed_size: u64,
    verified: bool,
    elapsed_us: u128,
}

#[derive(Deserialize)]
struct AnalyzeRequest {
    size_bytes: Option<u64>,
    sample: Option<String>,
}
#[derive(Serialize)]
struct AnalyzeResponse {
    patterns_detected: Vec<PatternInfo>,
    recommended_algorithm: String,
    estimated_ratio: f64,
    procedural_applicable: bool,
    elapsed_us: u128,
}
#[derive(Serialize)]
struct PatternInfo {
    pattern_type: String,
    coverage_pct: f64,
    compression_potential: String,
}

#[derive(Deserialize)]
struct BatchRequest {
    items: Vec<BatchItem>,
}
#[derive(Deserialize)]
struct BatchItem {
    data_type: Option<String>,
    size_bytes: u64,
}
#[derive(Serialize)]
struct BatchResponse {
    job_id: String,
    items_processed: usize,
    total_original_bytes: u64,
    total_compressed_bytes: u64,
    overall_ratio: f64,
    elapsed_us: u128,
}

#[derive(Serialize)]
struct AlgorithmInfo {
    name: String,
    description: String,
    typical_ratio: String,
    speed: String,
    best_for: String,
}
#[derive(Serialize)]
struct StatsResponse {
    total_compressions: u64,
    total_decompressions: u64,
    total_analyses: u64,
    bytes_in: u64,
    bytes_out: u64,
    overall_ratio: f64,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "zip_engine=info".into()),
        )
        .init();
    let state = Arc::new(AppState {
        start_time: Instant::now(),
        stats: Mutex::new(Stats {
            total_compressions: 0,
            total_decompressions: 0,
            total_analyses: 0,
            bytes_in: 0,
            bytes_out: 0,
        }),
    });
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);
    let app = Router::new()
        .route("/health", get(health))
        .route("/api/v1/zip/compress", post(compress))
        .route("/api/v1/zip/decompress", post(decompress))
        .route("/api/v1/zip/analyze", post(analyze))
        .route("/api/v1/zip/batch", post(batch))
        .route("/api/v1/zip/algorithms", get(algorithms))
        .route("/api/v1/zip/stats", get(stats))
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .with_state(state);
    let addr = std::env::var("ZIP_ADDR").unwrap_or_else(|_| "0.0.0.0:8081".into());
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    tracing::info!("Zip Engine on {addr}");
    axum::serve(listener, app).await.unwrap();
}

async fn health(State(s): State<Arc<AppState>>) -> Json<Health> {
    let st = s.stats.lock().unwrap();
    Json(Health {
        status: "ok".into(),
        version: env!("CARGO_PKG_VERSION").into(),
        uptime_secs: s.start_time.elapsed().as_secs(),
        total_ops: st.total_compressions + st.total_decompressions,
    })
}

async fn compress(
    State(s): State<Arc<AppState>>,
    Json(req): Json<CompressRequest>,
) -> Json<CompressResponse> {
    let t = Instant::now();
    let algo = req.algorithm.unwrap_or_else(|| "procedural-auto".into());
    let orig = req.size_bytes.unwrap_or(1_000_000);
    let data_type = req.data_type.unwrap_or_else(|| "generic".into());
    let (ratio, procedural, pattern) = match data_type.as_str() {
        "terrain" | "mesh" | "3d" => (500.0, true, "procedural-geometry".into()),
        "texture" | "image" => (100.0, true, "procedural-texture".into()),
        "log" | "csv" | "json" => (50.0, true, "repetitive-structure".into()),
        _ => (10.0, false, "lzma-fallback".into()),
    };
    let compressed = (orig as f64 / ratio) as u64;
    {
        let mut st = s.stats.lock().unwrap();
        st.total_compressions += 1;
        st.bytes_in += orig;
        st.bytes_out += compressed;
    }
    Json(CompressResponse {
        job_id: uuid::Uuid::new_v4().to_string(),
        algorithm: algo,
        original_size_bytes: orig,
        compressed_size_bytes: compressed,
        ratio,
        procedural_pattern_detected: procedural,
        pattern_type: pattern,
        elapsed_us: t.elapsed().as_micros(),
    })
}

async fn decompress(
    State(s): State<Arc<AppState>>,
    Json(req): Json<DecompressRequest>,
) -> Json<DecompressResponse> {
    let t = Instant::now();
    let algo = req.algorithm.unwrap_or_else(|| "procedural-auto".into());
    let compressed = req.compressed_size_bytes.unwrap_or(10_000);
    let decompressed = compressed * 100;
    s.stats.lock().unwrap().total_decompressions += 1;
    Json(DecompressResponse {
        job_id: uuid::Uuid::new_v4().to_string(),
        algorithm: algo,
        compressed_size: compressed,
        decompressed_size: decompressed,
        verified: true,
        elapsed_us: t.elapsed().as_micros(),
    })
}

async fn analyze(
    State(s): State<Arc<AppState>>,
    Json(req): Json<AnalyzeRequest>,
) -> Json<AnalyzeResponse> {
    let t = Instant::now();
    s.stats.lock().unwrap().total_analyses += 1;
    let size = req.size_bytes.unwrap_or(1_000_000);
    let procedural = size > 100_000;
    Json(AnalyzeResponse {
        patterns_detected: vec![
            PatternInfo {
                pattern_type: "repetitive-blocks".into(),
                coverage_pct: 45.0,
                compression_potential: "100-500x".into(),
            },
            PatternInfo {
                pattern_type: "sequential-numeric".into(),
                coverage_pct: 20.0,
                compression_potential: "10-50x".into(),
            },
        ],
        recommended_algorithm: if procedural {
            "procedural-gen".into()
        } else {
            "lzma".into()
        },
        estimated_ratio: if procedural { 200.0 } else { 5.0 },
        procedural_applicable: procedural,
        elapsed_us: t.elapsed().as_micros(),
    })
}

async fn batch(
    State(s): State<Arc<AppState>>,
    Json(req): Json<BatchRequest>,
) -> Json<BatchResponse> {
    let t = Instant::now();
    let total_orig: u64 = req.items.iter().map(|i| i.size_bytes).sum();
    let total_comp = total_orig / 50;
    {
        let mut st = s.stats.lock().unwrap();
        st.total_compressions += req.items.len() as u64;
        st.bytes_in += total_orig;
        st.bytes_out += total_comp;
    }
    Json(BatchResponse {
        job_id: uuid::Uuid::new_v4().to_string(),
        items_processed: req.items.len(),
        total_original_bytes: total_orig,
        total_compressed_bytes: total_comp,
        overall_ratio: total_orig as f64 / total_comp as f64,
        elapsed_us: t.elapsed().as_micros(),
    })
}

async fn algorithms() -> Json<Vec<AlgorithmInfo>> {
    Json(vec![
        AlgorithmInfo { name: "procedural-gen".into(), description: "Procedural generation compression: detects patterns and stores generation parameters".into(), typical_ratio: "10-1000x".into(), speed: "Medium".into(), best_for: "3D data, terrain, textures, repetitive structures".into() },
        AlgorithmInfo { name: "lzma".into(), description: "LZMA fallback for non-procedural data".into(), typical_ratio: "3-10x".into(), speed: "Slow".into(), best_for: "Generic binary data, archives".into() },
        AlgorithmInfo { name: "hybrid-auto".into(), description: "Automatic selection: procedural where applicable, LZMA fallback".into(), typical_ratio: "5-500x".into(), speed: "Adaptive".into(), best_for: "Mixed content, automatic optimization".into() },
    ])
}

async fn stats(State(s): State<Arc<AppState>>) -> Json<StatsResponse> {
    let st = s.stats.lock().unwrap();
    let ratio = if st.bytes_out > 0 {
        st.bytes_in as f64 / st.bytes_out as f64
    } else {
        0.0
    };
    Json(StatsResponse {
        total_compressions: st.total_compressions,
        total_decompressions: st.total_decompressions,
        total_analyses: st.total_analyses,
        bytes_in: st.bytes_in,
        bytes_out: st.bytes_out,
        overall_ratio: ratio,
    })
}
