//! Context compressor for token-efficient LLM interactions.
//!
//! Port from Python `analysis/context_compressor.py`.
//! Truncation-based compression to stay within token limits.

/// Rough estimate: 1 token ≈ 4 chars for English code.
const CHARS_PER_TOKEN: usize = 4;

/// Compresses file content and analysis context to fit within token budgets.
pub struct ContextCompressor {
    max_tokens: usize,
    max_chars: usize,
}

impl ContextCompressor {
    pub fn new(max_tokens: usize) -> Self {
        Self {
            max_tokens,
            max_chars: max_tokens * CHARS_PER_TOKEN,
        }
    }

    /// Compress a map of {path: content} to fit within budget.
    pub fn compress_files(
        &self,
        files: &[(&str, &str)],
        max_per_file_tokens: usize,
    ) -> Vec<(String, String)> {
        let max_per_file_chars = max_per_file_tokens * CHARS_PER_TOKEN;
        let total_budget = self.max_chars;
        let mut compressed = Vec::new();
        let mut total_chars = 0;

        // Sort by size — process smallest first to keep more files
        let mut sorted: Vec<_> = files.to_vec();
        sorted.sort_by_key(|(_, content)| content.len());

        for (path, content) in sorted {
            let remaining = total_budget.saturating_sub(total_chars);
            if remaining == 0 {
                break;
            }

            let per_file_limit = max_per_file_chars.min(remaining);
            let result = if content.len() <= per_file_limit {
                content.to_string()
            } else {
                Self::truncate_middle(content, per_file_limit)
            };

            total_chars += result.len();
            compressed.push((path.to_string(), result));
        }

        compressed
    }

    /// Compress arbitrary text to fit within token budget.
    pub fn compress_text(&self, text: &str, max_tokens: Option<usize>) -> String {
        let limit = (max_tokens.unwrap_or(self.max_tokens)) * CHARS_PER_TOKEN;
        if text.len() <= limit {
            return text.to_string();
        }
        Self::truncate_middle(text, limit)
    }

    /// Keep first and last portions, replace middle with marker.
    fn truncate_middle(text: &str, max_chars: usize) -> String {
        if text.len() <= max_chars {
            return text.to_string();
        }
        // 60% head, 40% tail
        let head_size = (max_chars as f64 * 0.6) as usize;
        let marker_size = 60;
        let tail_size = max_chars.saturating_sub(head_size + marker_size);

        if tail_size == 0 {
            return text.chars().take(max_chars).collect();
        }

        let omitted = text.len() - head_size - tail_size;
        let head: String = text.chars().take(head_size).collect();
        let tail: String = text.chars().skip(text.len() - tail_size).collect();

        format!(
            "{}\n\n... ({} chars / ~{} tokens omitted) ...\n\n{}",
            head,
            omitted,
            omitted / CHARS_PER_TOKEN,
            tail
        )
    }

    /// Compact finding summary for prompt injection.
    pub fn summarize_findings_compact(findings: &[crate::core::models::Finding]) -> String {
        if findings.is_empty() {
            return "No issues.".to_string();
        }
        let mut parts: Vec<String> = Vec::new();
        for f in findings.iter().take(10) {
            parts.push(format!("[{}] {} ({})", f.severity, f.title, f.file_path));
        }
        if findings.len() > 10 {
            parts.push(format!(" +{} more", findings.len() - 10));
        }
        parts.join("\n")
    }
}

impl Default for ContextCompressor {
    fn default() -> Self {
        Self::new(30_000)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compress_text_small() {
        let c = ContextCompressor::new(100);
        let text = "hello world";
        assert_eq!(c.compress_text(text, None), text);
    }

    #[test]
    fn test_compress_text_large() {
        let c = ContextCompressor::new(50); // 200 chars max
        let text = "a".repeat(1000);
        let result = c.compress_text(&text, None);
        // Result should be significantly smaller than original
        assert!(result.len() < 500, "Expected < 500, got {}", result.len());
        assert!(result.contains("omitted"));
    }

    #[test]
    fn test_compress_files_fits() {
        let c = ContextCompressor::new(10000);
        let files = vec![("a.py", "print('hello')"), ("b.py", "x = 1")];
        let result = c.compress_files(&files, 1000);
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_truncate_middle() {
        let text = "a".repeat(500);
        let result = ContextCompressor::truncate_middle(&text, 200);
        // head=120 + tail=20 + marker ≈ reasonable size
        assert!(result.contains("omitted"), "Should contain omitted marker");
        // Total content (head+tail) should be less than original
        assert!(result.len() < 500, "Expected < 500, got {}", result.len());
    }
}
