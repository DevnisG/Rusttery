use serde::{Deserialize, Serialize};

#[cfg(windows)]
use windows::Devices::Power::Battery;

#[cfg(target_os = "linux")]
use std::fs;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BatteryInfo {
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

#[cfg(windows)]
pub fn get_battery_info() -> Option<BatteryInfo> {
    let battery = Battery::AggregateBattery().ok()?;
    let report = battery.GetReport().ok()?;

    let full_ref = report.FullChargeCapacityInMilliwattHours().ok()?;
    let full = full_ref.Value().ok()? as f32;

    let remaining_ref = report.RemainingCapacityInMilliwattHours().ok()?;
    let remaining = remaining_ref.Value().ok()? as f32;

    if full <= 0.0 {
        return None;
    }

    let percent = ((remaining / full) * 100.0).round() as i32;
    
    let (health, capacity_design) = if let Ok(design_ref) = report.DesignCapacityInMilliwattHours() {
        if let Ok(design) = design_ref.Value() {
            let design = design as f32;
            if design > 0.0 {
                (Some(((full / design) * 100.0).round() as i32), Some((design / 1000.0) as i32))
            } else {
                (None, None)
            }
        } else {
            (None, None)
        }
    } else {
        (None, None)
    };

    let status = if let Ok(status_ref) = report.Status() {
        match status_ref.0 {
            1 => Some("Discharging".to_string()),
            2 => Some("Idle".to_string()),
            3 => Some("Charging".to_string()),
            _ => Some("Unknown".to_string()),
        }
    } else {
        None
    };

    let charge_rate = report.ChargeRateInMilliwatts()
        .ok()
        .and_then(|r| r.Value().ok())
        .map(|v| v as f32 / 1000.0);

    let (time_to_empty, time_to_full) = if let Some(rate) = charge_rate {
        if rate > 0.0 {
            let time_full = ((full - remaining) / rate * 60.0) as i32;
            (None, Some(time_full))
        } else if rate < 0.0 {
            let time_empty = (remaining / rate.abs() * 60.0) as i32;
            (Some(time_empty), None)
        } else {
            (None, None)
        }
    } else {
        (None, None)
    };

    Some(BatteryInfo {
        percent: percent.clamp(0, 100),
        health: health.map(|h| h.clamp(0, 100)),
        status,
        cycle_count: None,
        voltage_now: None,
        current_now: None,
        power_now: charge_rate,
        technology: None,
        manufacturer: None,
        model: None,
        serial_number: None,
        capacity_full: Some((full / 1000.0) as i32),
        capacity_design,
        time_to_empty,
        time_to_full,
    })
}

#[cfg(target_os = "linux")]
pub fn get_battery_info() -> Option<BatteryInfo> {
    let power_supply_path = "/sys/class/power_supply";
    
    let entries = fs::read_dir(power_supply_path).ok()?;
    
    for entry in entries.flatten() {
        let path = entry.path();
    
        let type_path = path.join("type");
        if let Ok(device_type) = fs::read_to_string(&type_path) {
            if device_type.trim() != "Battery" {
                continue;
            }
        } else {
            continue;
        }
        
        let capacity_path = path.join("capacity");
        let percent = if let Ok(capacity_str) = fs::read_to_string(&capacity_path) {
            if let Ok(capacity) = capacity_str.trim().parse::<i32>() {
                capacity.clamp(0, 100)
            } else {
                continue;
            }
        } else {
            continue;
        };
        
        let status = fs::read_to_string(path.join("status"))
            .ok()
            .map(|s| s.trim().to_string());

        let cycle_count = fs::read_to_string(path.join("cycle_count"))
            .ok()
            .and_then(|s| s.trim().parse::<i32>().ok());

        let voltage_now = fs::read_to_string(path.join("voltage_now"))
            .ok()
            .and_then(|s| s.trim().parse::<f32>().ok())
            .map(|v| v / 1_000_000.0);

        let current_now = fs::read_to_string(path.join("current_now"))
            .ok()
            .and_then(|s| s.trim().parse::<f32>().ok())
            .map(|c| c / 1_000_000.0);

        let power_now = fs::read_to_string(path.join("power_now"))
            .ok()
            .and_then(|s| s.trim().parse::<f32>().ok())
            .map(|p| p / 1_000_000.0);

        let technology = fs::read_to_string(path.join("technology"))
            .ok()
            .map(|s| s.trim().to_string());

        let manufacturer = fs::read_to_string(path.join("manufacturer"))
            .ok()
            .map(|s| s.trim().to_string());

        let model = fs::read_to_string(path.join("model_name"))
            .ok()
            .map(|s| s.trim().to_string());

        let serial_number = fs::read_to_string(path.join("serial_number"))
            .ok()
            .map(|s| s.trim().to_string());

        let (health, capacity_full, capacity_design) = {
            let energy_full = fs::read_to_string(path.join("energy_full"))
                .ok()
                .and_then(|s| s.trim().parse::<f32>().ok());
            
            let energy_full_design = fs::read_to_string(path.join("energy_full_design"))
                .ok()
                .and_then(|s| s.trim().parse::<f32>().ok());

            if let (Some(full), Some(design)) = (energy_full, energy_full_design) {
                let health_val = if design > 0.0 {
                    Some(((full / design) * 100.0).round() as i32)
                } else {
                    None
                };
                (health_val, Some((full / 1_000_000.0) as i32), Some((design / 1_000_000.0) as i32))
            } else {
                let charge_full = fs::read_to_string(path.join("charge_full"))
                    .ok()
                    .and_then(|s| s.trim().parse::<f32>().ok());
                
                let charge_full_design = fs::read_to_string(path.join("charge_full_design"))
                    .ok()
                    .and_then(|s| s.trim().parse::<f32>().ok());

                if let (Some(full), Some(design)) = (charge_full, charge_full_design) {
                    let health_val = if design > 0.0 {
                        Some(((full / design) * 100.0).round() as i32)
                    } else {
                        None
                    };
                    (health_val, Some((full / 1_000_000.0) as i32), Some((design / 1_000_000.0) as i32))
                } else {
                    (None, None, None)
                }
            }
        };

        let time_to_empty = fs::read_to_string(path.join("time_to_empty_now"))
            .ok()
            .and_then(|s| s.trim().parse::<i32>().ok());

        let time_to_full = fs::read_to_string(path.join("time_to_full_now"))
            .ok()
            .and_then(|s| s.trim().parse::<i32>().ok());
        
        return Some(BatteryInfo {
            percent,
            health: health.map(|h| h.clamp(0, 100)),
            status,
            cycle_count,
            voltage_now,
            current_now,
            power_now,
            technology,
            manufacturer,
            model,
            serial_number,
            capacity_full,
            capacity_design,
            time_to_empty,
            time_to_full,
        });
    }
    
    None
}

#[cfg(not(any(windows, target_os = "linux")))]
pub fn get_battery_info() -> Option<BatteryInfo> {
    None
}
