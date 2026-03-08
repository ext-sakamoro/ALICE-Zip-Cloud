"use client";
import { useState } from "react";

type Tab = "compress" | "decompress" | "analyze" | "batch" | "algorithms" | "stats";

const DEFAULTS: Record<Tab, string> = {
  compress: JSON.stringify({
    data: "SGVsbG8sIFdvcmxkISBUaGlzIGlzIGEgdGVzdCBwYXlsb2FkLg==",
    algorithm: "zstd",
    level: 3,
    options: { checksum: true, dict_id: null }
  }, null, 2),
  decompress: JSON.stringify({
    data: "KLUv/QBYWQAASGVsbG8sIFdvcmxkIQ==",
    algorithm: "zstd",
    expected_size: 1024
  }, null, 2),
  analyze: JSON.stringify({
    data: "SGVsbG8sIFdvcmxkISBUaGlzIGlzIGEgdGVzdCBwYXlsb2FkLg==",
    suggest_algorithm: true,
    sample_size: 4096
  }, null, 2),
  batch: JSON.stringify({
    items: [
      { id: "file-001", data: "SGVsbG8gV29ybGQ=", algorithm: "lz4" },
      { id: "file-002", data: "VGhpcyBpcyBhIHRlc3Q=", algorithm: "zstd" },
      { id: "file-003", data: "QW5vdGhlciBmaWxl", algorithm: "brotli" }
    ],
    parallel: true,
    on_error: "skip"
  }, null, 2),
  algorithms: "",
  stats: "",
};

const TAB_LABELS: Record<Tab, string> = {
  compress: "POST /compress",
  decompress: "POST /decompress",
  analyze: "POST /analyze",
  batch: "POST /batch",
  algorithms: "GET /algorithms",
  stats: "GET /stats",
};

const GET_TABS: Tab[] = ["algorithms", "stats"];

const ROUTES: Record<Tab, string> = {
  compress: "/api/v1/zip/compress",
  decompress: "/api/v1/zip/decompress",
  analyze: "/api/v1/zip/analyze",
  batch: "/api/v1/zip/batch",
  algorithms: "/api/v1/zip/algorithms",
  stats: "/api/v1/zip/stats",
};

export default function ConsolePage() {
  const [activeTab, setActiveTab] = useState<Tab>("compress");
  const [input, setInput] = useState(DEFAULTS["compress"]);
  const [response, setResponse] = useState("");
  const [loading, setLoading] = useState(false);

  const API = "http://localhost:8081";

  const handleTabChange = (tab: Tab) => {
    setActiveTab(tab);
    setInput(DEFAULTS[tab]);
    setResponse("");
  };

  const send = async () => {
    setLoading(true);
    try {
      const isGet = GET_TABS.includes(activeTab);
      const res = await fetch(`${API}${ROUTES[activeTab]}`, {
        method: isGet ? "GET" : "POST",
        headers: isGet ? {} : { "Content-Type": "application/json" },
        body: isGet ? undefined : input,
      });
      setResponse(JSON.stringify(await res.json(), null, 2));
    } catch (e: unknown) {
      setResponse(`Error: ${e instanceof Error ? e.message : String(e)}`);
    }
    setLoading(false);
  };

  return (
    <div style={{ padding: 24, fontFamily: "monospace", background: "#0a0a0a", minHeight: "100vh", color: "#fff" }}>
      <h1 style={{ marginBottom: 4 }}>ALICE Zip-Cloud — Console</h1>
      <p style={{ color: "#666", marginBottom: 24, fontSize: 14 }}>Cloud compression API tester (zstd, lz4, brotli, gzip, xz)</p>

      <div style={{ display: "flex", gap: 8, marginBottom: 16, flexWrap: "wrap" }}>
        {(Object.keys(TAB_LABELS) as Tab[]).map((tab) => (
          <button
            key={tab}
            onClick={() => handleTabChange(tab)}
            style={{
              padding: "6px 14px",
              borderRadius: 6,
              border: "1px solid",
              borderColor: activeTab === tab ? "#34d399" : "#333",
              background: activeTab === tab ? "#34d39920" : "#111",
              color: activeTab === tab ? "#34d399" : "#888",
              cursor: "pointer",
              fontSize: 13,
            }}
          >
            {TAB_LABELS[tab]}
          </button>
        ))}
      </div>

      {!GET_TABS.includes(activeTab) && (
        <textarea
          value={input}
          onChange={(e) => setInput(e.target.value)}
          rows={14}
          style={{
            width: "100%",
            fontFamily: "monospace",
            fontSize: 13,
            background: "#111",
            color: "#e0e0e0",
            border: "1px solid #333",
            borderRadius: 8,
            padding: 12,
            boxSizing: "border-box",
          }}
        />
      )}

      {GET_TABS.includes(activeTab) && (
        <div style={{ color: "#666", fontSize: 13, padding: "12px 0" }}>
          No request body required for GET requests.
        </div>
      )}

      <button
        onClick={send}
        disabled={loading}
        style={{
          marginTop: 12,
          padding: "10px 28px",
          background: loading ? "#333" : "#34d399",
          color: loading ? "#666" : "#000",
          border: "none",
          borderRadius: 8,
          cursor: loading ? "not-allowed" : "pointer",
          fontWeight: "bold",
          fontSize: 14,
        }}
      >
        {loading ? "Sending..." : "Send"}
      </button>

      <pre
        style={{
          background: "#111",
          color: "#0f0",
          padding: 16,
          marginTop: 16,
          minHeight: 200,
          overflow: "auto",
          borderRadius: 8,
          border: "1px solid #222",
          fontSize: 13,
        }}
      >
        {response || "// Response will appear here"}
      </pre>
    </div>
  );
}
