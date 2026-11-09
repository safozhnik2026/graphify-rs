//! Persistence of query results to the local memory directory.

use std::path::{Path, PathBuf};

/// Save a query result (question + answer) to the memory directory.
///
/// Used by the `save-result` CLI command to persist answers for later reference.
pub fn save_query_result(
    question: &str,
    answer: &str,
    memory_dir: &Path,
    query_type: &str,
    source_nodes: Option<&[String]>,
) -> std::io::Result<PathBuf> {
    std::fs::create_dir_all(memory_dir)?;

    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    let filename = format!("{query_type}_{timestamp}.md");
    let path = memory_dir.join(&filename);

    let nodes_str = source_nodes.map(|n| n.join(", ")).unwrap_or_default();

    let content = format!(
        "---\ntype: {query_type}\ntimestamp: {timestamp}\nnodes: [{nodes_str}]\n---\n\n## Question\n\n{question}\n\n## Answer\n\n{answer}\n"
    );
    std::fs::write(&path, content)?;

    tracing::info!("Saved query result: {} -> {}", query_type, path.display());
    Ok(path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_save_query_result() {
        let tmp = tempfile::tempdir().unwrap();
        let path = save_query_result(
            "What is Rust?",
            "A systems programming language.",
            tmp.path(),
            "query",
            Some(&["node1".to_string(), "node2".to_string()]),
        )
        .unwrap();

        assert!(path.exists());
        let content = std::fs::read_to_string(&path).unwrap();
        assert!(content.contains("What is Rust?"));
        assert!(content.contains("systems programming language"));
        assert!(content.contains("node1, node2"));
        assert!(content.contains("type: query"));
    }

    #[test]
    fn test_save_query_result_no_nodes() {
        let tmp = tempfile::tempdir().unwrap();
        let path = save_query_result("question", "answer", tmp.path(), "chat", None).unwrap();

        let content = std::fs::read_to_string(&path).unwrap();
        assert!(content.contains("nodes: []"));
    }
}
