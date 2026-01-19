use std::{thread, time::Duration};
use crate::core::get_battery_info;
use crate::database::Database;

pub fn run() {
    let interval = Duration::from_secs(3);
    let mut last_percent: Option<i32> = None;
    let mut last_health: Option<i32> = None;

    let db = Database::new().ok();

    #[cfg(not(any(windows, target_os = "linux")))]
    {
        eprintln!("Error: Este sistema operativo no está soportado.");
        eprintln!("Rusttery solo funciona en Windows y Linux.");
        return;
    }

    loop {
        if let Some(info) = get_battery_info() {
            let percent_changed = last_percent != Some(info.percent);
            let health_changed = last_health != info.health;
            
            if percent_changed || health_changed {
                print!("Batería: {}%", info.percent);
                
                if let Some(health) = info.health {
                    print!(" | Salud: {}%", health);
                }
                
                if let Some(status) = &info.status {
                    print!(" | Estado: {}", status);
                }
                
                if let Some(power) = info.power_now {
                    print!(" | Potencia: {:.2}W", power);
                }
                
                println!();
                
                last_percent = Some(info.percent);
                last_health = info.health;

                if let Some(ref database) = db {
                    let _ = database.save_battery_info(&info);
                }
            }
        } else {
            println!("No se pudo obtener información de la batería.");
        }

        thread::sleep(interval);
    }
}

pub fn run_json() {
    if let Some(info) = get_battery_info() {
        match serde_json::to_string_pretty(&info) {
            Ok(json) => println!("{}", json),
            Err(_) => {
                eprintln!("{{\"error\":\"Error al serializar datos\"}}");
                std::process::exit(1);
            }
        }
    } else {
        eprintln!("{{\"error\":\"No se pudo obtener información de la batería\"}}");
        std::process::exit(1);
    }
}
