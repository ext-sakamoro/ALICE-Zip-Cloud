export default function LandingPage() {
  return (
    <div
      style={{
        minHeight: "100vh",
        background: "linear-gradient(135deg, #0a0a0a, #001a10)",
        color: "#fff",
        fontFamily: "system-ui, sans-serif",
      }}
    >
      <header
        style={{
          padding: "24px 48px",
          display: "flex",
          justifyContent: "space-between",
          alignItems: "center",
          borderBottom: "1px solid #ffffff10",
        }}
      >
        <h2 style={{ margin: 0, color: "#34d399" }}>ALICE Zip-Cloud</h2>
        <a
          href="/dashboard/console"
          style={{
            color: "#34d399",
            textDecoration: "none",
            padding: "8px 20px",
            border: "1px solid #34d399",
            borderRadius: 8,
            fontSize: 14,
          }}
        >
          Console →
        </a>
      </header>

      <main
        style={{
          maxWidth: 960,
          margin: "0 auto",
          padding: "80px 24px",
          textAlign: "center",
        }}
      >
        <div
          style={{
            display: "inline-block",
            background: "#34d39920",
            color: "#34d399",
            padding: "4px 16px",
            borderRadius: 20,
            fontSize: 13,
            marginBottom: 24,
          }}
        >
          Cloud Compression Service
        </div>
        <h1 style={{ fontSize: 52, marginBottom: 16, lineHeight: 1.1 }}>
          Compress Smarter,<br />Transfer Faster
        </h1>
        <p style={{ fontSize: 20, color: "#aaa", marginBottom: 48, maxWidth: 600, margin: "0 auto 48px" }}>
          Multi-algorithm cloud compression with intelligent algorithm selection, batch processing, and real-time throughput analytics.
        </p>

        <div
          style={{
            display: "grid",
            gridTemplateColumns: "repeat(3, 1fr)",
            gap: 24,
            textAlign: "left",
          }}
        >
          <div style={{ background: "#ffffff08", borderRadius: 12, padding: 28, border: "1px solid #ffffff10" }}>
            <div style={{ fontSize: 28, marginBottom: 12 }}>&#x1F4E6;</div>
            <h3 style={{ margin: "0 0 8px", color: "#34d399" }}>Multi-Algorithm</h3>
            <p style={{ color: "#aaa", margin: 0, lineHeight: 1.6 }}>
              Choose from zstd, lz4, brotli, gzip, and xz. Each optimized for different data types and latency targets.
            </p>
          </div>
          <div style={{ background: "#ffffff08", borderRadius: 12, padding: 28, border: "1px solid #ffffff10" }}>
            <div style={{ fontSize: 28, marginBottom: 12 }}>&#x1F4CA;</div>
            <h3 style={{ margin: "0 0 8px", color: "#34d399" }}>Smart Analysis</h3>
            <p style={{ color: "#aaa", margin: 0, lineHeight: 1.6 }}>
              Analyze data entropy and structure to automatically recommend the best algorithm and compression level.
            </p>
          </div>
          <div style={{ background: "#ffffff08", borderRadius: 12, padding: 28, border: "1px solid #ffffff10" }}>
            <div style={{ fontSize: 28, marginBottom: 12 }}>&#x26A1;</div>
            <h3 style={{ margin: "0 0 8px", color: "#34d399" }}>Batch Processing</h3>
            <p style={{ color: "#aaa", margin: 0, lineHeight: 1.6 }}>
              Compress thousands of files in parallel with per-item algorithm selection and fault-tolerant error handling.
            </p>
          </div>
        </div>

        <div style={{ marginTop: 64 }}>
          <a
            href="/dashboard/console"
            style={{
              display: "inline-block",
              background: "#34d399",
              color: "#000",
              padding: "14px 36px",
              borderRadius: 10,
              textDecoration: "none",
              fontWeight: "bold",
              fontSize: 16,
            }}
          >
            Open Console
          </a>
        </div>
      </main>
    </div>
  );
}
