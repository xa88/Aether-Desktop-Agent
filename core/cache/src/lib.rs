//! ada-cache: Repository, execution, error, and UI state caching.

use rusqlite::{params, Connection};
use std::path::Path;
use chrono::{Utc, Duration};
use serde::{Serialize, Deserialize};
use sha2::{Sha256, Digest};
use hex;

pub mod metrics;

#[derive(Debug, Serialize, Deserialize)]
pub struct RepoFingerprint {
    pub git_head: String,
    pub lockfile_hash: String,
    pub toolchain: String,
}

impl RepoFingerprint {
    pub fn compute_hash(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(&self.git_head);
        hasher.update(&self.lockfile_hash);
        hasher.update(&self.toolchain);
        hex::encode(hasher.finalize())
    }
}

pub struct CacheManager {
    conn: Connection,
}

impl CacheManager {
    pub fn new<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        let conn = Connection::open(path)?;
        Self::init_db(&conn)?;
        Ok(Self { conn })
    }

    fn init_db(conn: &Connection) -> anyhow::Result<()> {
        // exec_cache: generic command/step results
        conn.execute(
            "CREATE TABLE IF NOT EXISTS exec_cache (
                key TEXT PRIMARY KEY,
                value TEXT,
                expires_at DATETIME,
                ts DATETIME DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        )?;

        // repo_cache: setup/install states
        conn.execute(
            "CREATE TABLE IF NOT EXISTS repo_cache (
                repo_id TEXT PRIMARY KEY,
                fingerprint_hash TEXT,
                state TEXT,
                ts DATETIME DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        )?;

        // error_sig_cache: playbook mapping
        conn.execute(
            "CREATE TABLE IF NOT EXISTS error_sig_cache (
                signature TEXT PRIMARY KEY,
                playbook_id TEXT,
                ts DATETIME DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        )?;

        Ok(())
    }

    pub fn set_exec(&self, key: &str, value: &str, ttl_days: i64) -> anyhow::Result<()> {
        let expires_at = Utc::now() + Duration::days(ttl_days);
        self.conn.execute(
            "INSERT OR REPLACE INTO exec_cache (key, value, expires_at) VALUES (?1, ?2, ?3)",
            params![key, value, expires_at.to_rfc3339()],
        )?;
        Ok(())
    }

    pub fn get_exec(&self, key: &str) -> anyhow::Result<Option<String>> {
        let mut stmt = self.conn.prepare("SELECT value, expires_at FROM exec_cache WHERE key = ?1")?;
        let mut rows = stmt.query(params![key])?;
        if let Some(row) = rows.next()? {
            let val: String = row.get(0)?;
            let exp_str: String = row.get(1)?;
            let exp = chrono::DateTime::parse_from_rfc3339(&exp_str)?;
            if exp < Utc::now() {
                return Ok(None);
            }
            Ok(Some(val))
        } else {
            Ok(None)
        }
    }

    pub fn set_repo_state(&self, repo_id: &str, hash: &str, state: &str) -> anyhow::Result<()> {
        self.conn.execute(
            "INSERT OR REPLACE INTO repo_cache (repo_id, fingerprint_hash, state) VALUES (?1, ?2, ?3)",
            params![repo_id, hash, state],
        )?;
        Ok(())
    }

    pub fn get_repo_state(&self, repo_id: &str, current_hash: &str) -> anyhow::Result<Option<String>> {
        let mut stmt = self.conn.prepare("SELECT state FROM repo_cache WHERE repo_id = ?1 AND fingerprint_hash = ?2")?;
        let mut rows = stmt.query(params![repo_id, current_hash])?;
        if let Some(row) = rows.next()? {
            Ok(Some(row.get(0)?))
        } else {
            Ok(None)
        }
    }

    /// Normalize error messages (remove paths, ports, numbers) to create a signature
    pub fn normalize_error(msg: &str) -> String {
        let mut result = msg.to_lowercase();
        // Simple regex-like logic for MVP: strip numbers and common noise
        result = result.chars().filter(|c| !c.is_numeric()).collect();
        result
    }

    pub fn get_similar_plans(&self, goal_keyword: &str) -> anyhow::Result<Vec<String>> {
        let mut stmt = self.conn.prepare("SELECT value FROM exec_cache WHERE key LIKE ?1 LIMIT 5")?;
        let query = format!("%{}%", goal_keyword);
        let mut rows = stmt.query(params![query])?;
        let mut results = Vec::new();
        while let Some(row) = rows.next()? {
            results.push(row.get(0)?);
        }
        Ok(results)
    }
}
