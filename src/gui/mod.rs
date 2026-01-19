use eframe::egui;
use std::time::Duration;
use crate::core::{get_battery_info, BatteryInfo};

pub struct BatteryApp {
    battery_info: Option<BatteryInfo>,
}

impl Default for BatteryApp {
    fn default() -> Self {
        Self {
            battery_info: get_battery_info(),
        }
    }
}

impl eframe::App for BatteryApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.battery_info = get_battery_info();

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(20.0);
                    
                    ui.heading("🔋 Rusttery");
                    ui.add_space(20.0);

                    if let Some(info) = &self.battery_info {
                        ui.label(
                            egui::RichText::new(format!("{}%", info.percent))
                                .size(70.0)
                                .color(egui::Color32::from_rgb(0, 180, 100))
                        );
                        
                        ui.add_space(5.0);
                        ui.label(
                            egui::RichText::new("Carga actual")
                                .size(18.0)
                        );

                        ui.add_space(20.0);

                        if let Some(health) = info.health {
                            ui.label(
                                egui::RichText::new(format!("Salud: {}%", health))
                                    .size(28.0)
                                    .color(egui::Color32::from_rgb(100, 150, 255))
                            );
                        }

                        ui.add_space(15.0);
                        ui.separator();
                        ui.add_space(10.0);

                        ui.label(egui::RichText::new("Detalles").size(20.0).strong());
                        ui.add_space(10.0);

                        egui::Grid::new("battery_grid")
                            .num_columns(2)
                            .spacing([20.0, 8.0])
                            .striped(true)
                            .show(ui, |ui| {
                                if let Some(status) = &info.status {
                                    ui.label("Estado:");
                                    ui.label(status);
                                    ui.end_row();
                                }

                                if let Some(cycles) = info.cycle_count {
                                    ui.label("Ciclos:");
                                    ui.label(format!("{}", cycles));
                                    ui.end_row();
                                }

                                if let Some(voltage) = info.voltage_now {
                                    ui.label("Voltaje:");
                                    ui.label(format!("{:.2} V", voltage));
                                    ui.end_row();
                                }

                                if let Some(current) = info.current_now {
                                    ui.label("Corriente:");
                                    ui.label(format!("{:.2} A", current));
                                    ui.end_row();
                                }

                                if let Some(power) = info.power_now {
                                    ui.label("Potencia:");
                                    ui.label(format!("{:.2} W", power));
                                    ui.end_row();
                                }

                                if let Some(tech) = &info.technology {
                                    ui.label("Tecnología:");
                                    ui.label(tech);
                                    ui.end_row();
                                }

                                if let Some(manufacturer) = &info.manufacturer {
                                    ui.label("Fabricante:");
                                    ui.label(manufacturer);
                                    ui.end_row();
                                }

                                if let Some(model) = &info.model {
                                    ui.label("Modelo:");
                                    ui.label(model);
                                    ui.end_row();
                                }

                                if let Some(cap_full) = info.capacity_full {
                                    ui.label("Capacidad actual:");
                                    ui.label(format!("{} mWh", cap_full));
                                    ui.end_row();
                                }

                                if let Some(cap_design) = info.capacity_design {
                                    ui.label("Capacidad diseño:");
                                    ui.label(format!("{} mWh", cap_design));
                                    ui.end_row();
                                }

                                if let Some(time) = info.time_to_empty {
                                    ui.label("Tiempo restante:");
                                    ui.label(format!("{} min", time));
                                    ui.end_row();
                                }

                                if let Some(time) = info.time_to_full {
                                    ui.label("Tiempo hasta carga:");
                                    ui.label(format!("{} min", time));
                                    ui.end_row();
                                }
                            });

                        ui.add_space(10.0);
                    } else {
                        ui.label(
                            egui::RichText::new("❌ No se pudo obtener información de la batería")
                                .size(18.0)
                                .color(egui::Color32::RED)
                        );
                    }
                });
            });
        });

        ctx.request_repaint_after(Duration::from_secs(3));
    }
}

pub fn run() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([500.0, 600.0])
            .with_resizable(true),
        ..Default::default()
    };

    eframe::run_native(
        "Rusttery",
        options,
        Box::new(|_cc| Ok(Box::new(BatteryApp::default()))),
    )
}
