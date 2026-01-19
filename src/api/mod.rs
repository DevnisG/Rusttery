use tiny_http::{Server, Response, Header};
use crate::core::get_battery_info;
use crate::database::Database;

pub fn start_server() {
    let server = Server::http("0.0.0.0:3000").expect("No se pudo iniciar el servidor en puerto 3000");
    let db = Database::new().expect("No se pudo iniciar la base de datos");
    
    println!("Servidor Rusttery ejecutándose en http://localhost:3000");
    println!("\n Endpoints disponibles:");
    println!("  GET /api/v1/battery/check - Carga actual y salud");
    println!("  GET /api/v1/battery/health - Solo salud");
    println!("  GET /api/v1/battery/status - Solo carga actual");
    println!("  GET /api/v1/battery/full - Información completa");
    println!("  GET /api/v1/battery/history?hours=24 - Historial de carga");
    println!("  GET /api/v1/battery/statistics?hours=24 - Estadísticas\n");
    for request in server.incoming_requests() {
        let url = request.url().to_string();
        
        let cors_headers = vec![
            Header::from_bytes(&b"Access-Control-Allow-Origin"[..], &b"*"[..]).unwrap(),
            Header::from_bytes(&b"Access-Control-Allow-Methods"[..], &b"GET, OPTIONS"[..]).unwrap(),
            Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap(),
        ];

        if request.method().as_str() == "OPTIONS" {
            let response = Response::empty(204).with_header(cors_headers[0].clone())
                .with_header(cors_headers[1].clone());
            let _ = request.respond(response);
            continue;
        }

        let (path, query) = url.split_once('?').unwrap_or((&url, ""));

        match path {
            "/api/v1/battery/check" => {
                if let Some(info) = get_battery_info() {
                    let json = serde_json::json!({
                        "status": "ok",
                        "data": {
                            "percent": info.percent,
                            "health": info.health
                        }
                    });
                    
                    let response = Response::from_string(json.to_string())
                        .with_header(cors_headers[0].clone())
                        .with_header(cors_headers[2].clone());
                    let _ = request.respond(response);
                } else {
                    send_error(request, &cors_headers, 500, "No se pudo obtener información de la batería");
                }
            }
            "/api/v1/battery/health" => {
                if let Some(info) = get_battery_info() {
                    let json = serde_json::json!({
                        "status": "ok",
                        "data": {
                            "health": info.health
                        }
                    });
                    
                    let response = Response::from_string(json.to_string())
                        .with_header(cors_headers[0].clone())
                        .with_header(cors_headers[2].clone());
                    let _ = request.respond(response);
                } else {
                    send_error(request, &cors_headers, 500, "No se pudo obtener información de la batería");
                }
            }
            "/api/v1/battery/status" => {
                if let Some(info) = get_battery_info() {
                    let json = serde_json::json!({
                        "status": "ok",
                        "data": {
                            "percent": info.percent
                        }
                    });
                    
                    let response = Response::from_string(json.to_string())
                        .with_header(cors_headers[0].clone())
                        .with_header(cors_headers[2].clone());
                    let _ = request.respond(response);
                } else {
                    send_error(request, &cors_headers, 500, "No se pudo obtener información de la batería");
                }
            }
            "/api/v1/battery/full" => {
                if let Some(info) = get_battery_info() {
                    let json = serde_json::json!({
                        "status": "ok",
                        "data": info
                    });
                    
                    let response = Response::from_string(json.to_string())
                        .with_header(cors_headers[0].clone())
                        .with_header(cors_headers[2].clone());
                    let _ = request.respond(response);
                } else {
                    send_error(request, &cors_headers, 500, "No se pudo obtener información de la batería");
                }
            }
            "/api/v1/battery/history" => {
                let hours = parse_hours(query).unwrap_or(24);
                
                match db.get_history(hours) {
                    Ok(history) => {
                        let json = serde_json::json!({
                            "status": "ok",
                            "data": {
                                "hours": hours,
                                "records": history
                            }
                        });
                        
                        let response = Response::from_string(json.to_string())
                            .with_header(cors_headers[0].clone())
                            .with_header(cors_headers[2].clone());
                        let _ = request.respond(response);
                    }
                    Err(_) => {
                        send_error(request, &cors_headers, 500, "Error al obtener historial");
                    }
                }
            }
            "/api/v1/battery/statistics" => {
                let hours = parse_hours(query).unwrap_or(24);
                
                match db.get_statistics(hours) {
                    Ok(stats) => {
                        let json = serde_json::json!({
                            "status": "ok",
                            "data": {
                                "hours": hours,
                                "statistics": stats
                            }
                        });
                        
                        let response = Response::from_string(json.to_string())
                            .with_header(cors_headers[0].clone())
                            .with_header(cors_headers[2].clone());
                        let _ = request.respond(response);
                    }
                    Err(_) => {
                        send_error(request, &cors_headers, 500, "Error al obtener estadísticas");
                    }
                }
            }
            _ => {
                let json = serde_json::json!({
                    "status": "error",
                    "message": "Ruta no encontrada"
                });
                let response = Response::from_string(json.to_string())
                    .with_status_code(404)
                    .with_header(cors_headers[0].clone())
                    .with_header(cors_headers[2].clone());
                let _ = request.respond(response);
            }
        }
    }
}

fn send_error(request: tiny_http::Request, cors_headers: &[Header], code: u16, message: &str) {
    let json = serde_json::json!({
        "status": "error",
        "message": message
    });
    let response = Response::from_string(json.to_string())
        .with_status_code(code)
        .with_header(cors_headers[0].clone())
        .with_header(cors_headers[2].clone());
    let _ = request.respond(response);
}

fn parse_hours(query: &str) -> Option<i64> {
    query.split('&')
        .find(|p| p.starts_with("hours="))
        .and_then(|p| p.strip_prefix("hours="))
        .and_then(|h| h.parse().ok())
}
