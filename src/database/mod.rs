use rusqlite::{Connection, Result};
use crate::core::BatteryInfo;
use std::path::PathBuf;

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new() -> Result<Self> {
        let db_path = Self::get_db_path();
        let conn = Connection::open(db_path)?;
        
        conn.execute(
            "CREATE TABLE IF NOT EXISTS battery_history (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                timestamp INTEGER NOT NULL,
                percent INTEGER NOT NULL,
                health INTEGER,
                status TEXT,
                cycle_count INTEGER,
                voltage_now REAL,
                current_now REAL,
                power_now REAL,
                technology TEXT,
                manufacturer TEXT,
                model TEXT,
                serial_number TEXT,
                capacity_full INTEGER,
                capacity_design INTEGER,
                time_to_empty INTEGER,
                time_to_full INTEGER
            )",
            [],
        )?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_timestamp ON battery_history(timestamp)",
            [],
        )?;

        let db = Database { conn };
        let _ = db.cleanup_old_records(30);
        Ok(db)
    }

    fn get_db_path() -> PathBuf {
        let mut path = dirs::data_local_dir()
            .unwrap_or_else(|| PathBuf::from("."));
        path.push("rusttery");
        std::fs::create_dir_all(&path).ok();
        path.push("battery_history.db");
        path
    }

    pub fn save_battery_info(&self, info: &BatteryInfo) -> Result<()> {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        self.conn.execute(
            "INSERT INTO battery_history (
                timestamp, percent, health, status, cycle_count, 
                voltage_now, current_now, power_now, technology,
                manufacturer, model, serial_number, capacity_full,
                capacity_design, time_to_empty, time_to_full
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16)",
            rusqlite::params![
                timestamp,
                info.percent,
                info.health,
                info.status,
                info.cycle_count,
                info.voltage_now,
                info.current_now,
                info.power_now,
                info.technology,
                info.manufacturer,
                info.model,
                info.serial_number,
                info.capacity_full,
                info.capacity_design,
                info.time_to_empty,
                info.time_to_full,
            ],
        )?;

        Ok(())
    }

    pub fn get_history(&self, hours: i64) -> Result<Vec<HistoryRecord>> {
        let since = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64 - (hours * 3600);

        let mut stmt = self.conn.prepare(
            "SELECT timestamp, percent, health, status, cycle_count,
                    voltage_now, current_now, power_now, technology,
                    manufacturer, model, serial_number, capacity_full,
                    capacity_design, time_to_empty, time_to_full
             FROM battery_history 
             WHERE timestamp >= ?1 
             ORDER BY timestamp ASC"
        )?;

        let records = stmt.query_map([since], |row| {
            Ok(HistoryRecord {
                timestamp: row.get(0)?,
                percent: row.get(1)?,
                health: row.get(2)?,
                status: row.get(3)?,
                cycle_count: row.get(4)?,
                voltage_now: row.get(5)?,
                current_now: row.get(6)?,
                power_now: row.get(7)?,
                technology: row.get(8)?,
                manufacturer: row.get(9)?,
                model: row.get(10)?,
                serial_number: row.get(11)?,
                capacity_full: row.get(12)?,
                capacity_design: row.get(13)?,
                time_to_empty: row.get(14)?,
                time_to_full: row.get(15)?,
            })
        })?;

        records.collect()
    }

    pub fn get_statistics(&self, hours: i64) -> Result<Statistics> {
        let since = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64 - (hours * 3600);

        let mut stmt = self.conn.prepare(
            "SELECT 
                AVG(percent) as avg_percent,
                MIN(percent) as min_percent,
                MAX(percent) as max_percent,
                AVG(power_now) as avg_power,
                COUNT(*) as total_records
             FROM battery_history 
             WHERE timestamp >= ?1"
        )?;

        let stats = stmt.query_row([since], |row| {
            Ok(Statistics {
                avg_percent: row.get(0).unwrap_or(0.0),
                min_percent: row.get(1).unwrap_or(0),
                max_percent: row.get(2).unwrap_or(0),
                avg_power: row.get(3).unwrap_or(0.0),
                total_records: row.get(4).unwrap_or(0),
            })
        })?;

        Ok(stats)
    }

    pub fn cleanup_old_records(&self, days: i64) -> Result<usize> {
        let cutoff = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64 - (days * 86400);

        self.conn.execute(
            "DELETE FROM battery_history WHERE timestamp < ?1",
            [cutoff],
        )
    }
}

#[derive(Debug, serde::Serialize)]
pub struct HistoryRecord {
    pub timestamp: i64,
    pub percent: i32,
    pub health: Option<i32>,
    pub status: Option<String>,
    pub cycle_count: Option<i32>,
    pub voltage_now: Option<f32>,
    pub current_now: Option<f32>,
    pub power_now: Option<f32>,
    pub technology: Option<String>,
    pub manufacturer: Option<String>,
    pub model: Option<String>,
    pub serial_number: Option<String>,
    pub capacity_full: Option<i32>,
    pub capacity_design: Option<i32>,
    pub time_to_empty: Option<i32>,
    pub time_to_full: Option<i32>,
}

#[derive(Debug, serde::Serialize)]
pub struct Statistics {
    pub avg_percent: f64,
    pub min_percent: i32,
    pub max_percent: i32,
    pub avg_power: f64,
    pub total_records: i64,
}
