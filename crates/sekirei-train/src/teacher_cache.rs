use std::collections::HashMap;
use std::fs;
use std::io::{BufWriter, Write};
use std::path::Path;

/// Load teacher scores from a JSONL cache file.
/// Each line: `{"sfen":"...","label_depth":N,"score_cp":N}`.
pub fn load(path: &Path) -> HashMap<String, i32> {
    let content = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("teacher cache: cannot read {:?}: {e}", path);
            return HashMap::new();
        }
    };
    let mut map = HashMap::new();
    let mut skipped = 0usize;
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let Ok(val) = serde_json::from_str::<serde_json::Value>(line) else {
            skipped += 1;
            continue;
        };
        let Some(sfen) = val.get("sfen").and_then(|v| v.as_str()) else {
            skipped += 1;
            continue;
        };
        let Some(cp) = val.get("score_cp").and_then(|v| v.as_i64()) else {
            skipped += 1;
            continue;
        };
        map.insert(sfen.to_string(), cp as i32);
    }
    if skipped > 0 {
        eprintln!("teacher cache: {skipped} lines skipped");
    }
    eprintln!(
        "teacher cache: {} entries loaded from {:?}",
        map.len(),
        path
    );
    map
}

/// Write teacher cache to a JSONL file.
/// `entries`: sfen → score_cp mapping; `label_depth` is recorded for documentation.
pub fn write(path: &Path, entries: &HashMap<String, i32>, label_depth: u32) -> std::io::Result<()> {
    let f = fs::File::create(path)?;
    let mut w = BufWriter::new(f);
    for (sfen, &cp) in entries {
        writeln!(
            w,
            r#"{{"sfen":{},"label_depth":{},"score_cp":{}}}"#,
            json_string(sfen),
            label_depth,
            cp
        )?;
    }
    Ok(())
}

fn json_string(s: &str) -> String {
    format!("\"{}\"", s.replace('\\', "\\\\").replace('"', "\\\""))
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    const SFEN_A: &str = "lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL b - 1";
    const SFEN_B: &str = "lnsgkgsnl/1r5b1/ppppppppp/9/9/2P6/PP1PPPPPP/1B5R1/LNSGKGSNL w - 2";

    #[test]
    fn roundtrip() {
        let f = NamedTempFile::new().unwrap();
        let mut expected = HashMap::new();
        expected.insert(SFEN_A.to_string(), 48i32);
        expected.insert(SFEN_B.to_string(), -120i32);
        write(f.path(), &expected, 4).unwrap();
        let loaded = load(f.path());
        assert_eq!(loaded, expected);
    }

    #[test]
    fn broken_lines_skipped() {
        let mut f = NamedTempFile::new().unwrap();
        writeln!(f, "not json").unwrap();
        writeln!(f, r#"{{"sfen":"{SFEN_A}","label_depth":4,"score_cp":100}}"#).unwrap();
        let loaded = load(f.path());
        assert_eq!(loaded.len(), 1);
        assert_eq!(loaded[SFEN_A], 100);
    }

    #[test]
    fn missing_score_cp_skipped() {
        let mut f = NamedTempFile::new().unwrap();
        writeln!(f, r#"{{"sfen":"{SFEN_A}","label_depth":4}}"#).unwrap();
        let loaded = load(f.path());
        assert!(loaded.is_empty());
    }
}
