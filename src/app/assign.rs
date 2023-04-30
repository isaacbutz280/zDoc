use chrono;
use egui::{RichText, ScrollArea};
use egui_extras::{Column, Table, TableBuilder, TableRow};

struct Caretaker {
    name: String,
    cred: String,
    case: u32,
}

impl Caretaker {
    fn new(name: &str, cred: &str) -> Self {
        Self {
            name: name.to_string(),
            cred: cred.to_string(),
            case: 0,
        }
    }
}

impl ToString for Caretaker {
    fn to_string(&self) -> String {
        todo!()
    }
}

struct Task {
    assignee: Option<Caretaker>,
    client_name: String,
    job: String,
}

impl Task {
    fn new(client_name: &str, job: &str) -> Self {
        Self {
            assignee: None,
            client_name: String::from(client_name),
            job: String::from(job),
        }
    }
}

#[derive(Default)]
pub struct Assign {
    location: String,
    date: Option<chrono::NaiveDate>,
    service: String,
    sort_by: bool, // true is name, false is age
    ct_sort_by: bool,
}

#[derive(Debug, Clone, PartialEq)]
struct Data {
    name: String,
    age: u32,
}

impl Data {
    fn new(name: &str, age: u32) -> Self {
        Self {
            name: name.to_string(),
            age,
        }
    }
}

impl super::App for Assign {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Header
            ui.vertical_centered(|ui| {
                ui.label("Assign");
            });

            ui.separator();

            // Filter boxes
            ui.horizontal(|ui| {
                egui::ComboBox::from_label("Location")
                    .selected_text(format!("{}", self.location))
                    .show_ui(ui, |ui| {
                        ui.style_mut().wrap = Some(false);
                        ui.set_min_width(60.0);
                        ui.selectable_value(&mut self.location, "X".to_string(), "Location X");
                        ui.selectable_value(&mut self.location, "Y".to_string(), "Location Y");
                        ui.selectable_value(&mut self.location, "Z".to_string(), "Location Z");
                    });

                ui.separator();

                #[cfg(feature = "chrono")]
                {
                    let date = self
                        .date
                        .get_or_insert_with(|| chrono::offset::Utc::now().date_naive());
                    ui.add(egui_extras::DatePickerButton::new(date));
                    ui.separator();
                }

                egui::ComboBox::from_label("Service")
                    .selected_text(format!("{}", self.service))
                    .show_ui(ui, |ui| {
                        ui.style_mut().wrap = Some(false);
                        ui.set_min_width(60.0);
                        ui.selectable_value(&mut self.service, "All".to_string(), "All");
                        ui.selectable_value(&mut self.service, "Some".to_string(), "Some");
                        ui.selectable_value(&mut self.service, "None".to_string(), "None");
                    });
            });

            ui.separator();

            // Assignmnet table
            TableBuilder::new(ui)
                .striped(false)
                .column(Column::auto().resizable(true))
                .column(Column::remainder())
                .body(|mut body| {
                    body.row(1000.0, |mut b_row| {
                        // Left side
                        b_row.col(|ui| {
                            let mut data = vec![
                                Task::new("Angela Johnson", "Dog Therapy"),
                                Task::new("Dax Quil", "Verbal Therapy"),
                                Task::new("Peter Groot", "Teethburshing"),
                                Task::new("Peter Groot", "Dog Therapy"),
                                Task::new("Peter Groot", "Stress Therapy"),
                                Task::new("Peter Groot", "Handwashing"),
                                Task::new("Gamora Thanos", "Stress Therepy"),
                            ];

                            // Sort data

                            ScrollArea::new([false, true]).show(ui, |ui| {
                                TableBuilder::new(ui)
                                    .striped(true)
                                    .column(Column::auto().resizable(true))
                                    .column(Column::auto().resizable(true))
                                    .column(Column::auto().resizable(true))
                                    .column(Column::remainder())
                                    .header(20.0, |mut header| {
                                        header.col(|ui| {
                                            if ui.heading("Assigned").clicked() {
                                                self.sort_by = true;
                                            }
                                        });
                                        header.col(|ui| {
                                            if ui.heading("Client Name").clicked() {
                                                self.sort_by = true;
                                            }
                                        });
                                        header.col(|ui| {
                                            if ui.heading("Task").clicked() {
                                                self.sort_by = true;
                                            }
                                        });
                                        header.col(|ui| {
                                            ui.heading("Alert?");
                                        });
                                    })
                                    .body(|mut body| {
                                        for d in data {
                                            body.row(30.0, |mut row| {
                                                row.col(|ui| {
                                                    ui.label(match d.assignee {
                                                        Some(a) => a.name,
                                                        None => "".to_string(),
                                                    });
                                                });
                                                row.col(|ui| {
                                                    ui.label(&d.client_name);
                                                });
                                                row.col(|ui| {
                                                    ui.label(&d.job);
                                                });
                                                row.col(|ui| {
                                                    if ui.button("Alert").clicked() {
                                                        Assign::send_alert();
                                                    }
                                                });
                                            });
                                        }
                                    });
                            });
                        });

                        // Right side
                        b_row.col(|ui| {
                            let mut data = vec![
                                Caretaker::new("Jim Butz", "Nurse RN"),
                                Caretaker::new("Dave Butz", "DSP"),
                                Caretaker::new("Isaac Butz", "Developer"),
                                Caretaker::new("Steve Butz", "None"),
                            ];

                            // Sorting state

                            data.sort_by(|a, b| {
                                if self.ct_sort_by {
                                    a.name.cmp(&b.name)
                                } else {
                                    b.cred.cmp(&a.cred)
                                }
                            });

                            // Must wrap in a push id to prevent id overlap
                            ui.push_id("Col 2", |ui| {
                                ScrollArea::new([false, true]).show(ui, |ui| {
                                    TableBuilder::new(ui)
                                        .striped(true)
                                        .column(Column::auto().resizable(true))
                                        .column(Column::auto().resizable(true))
                                        .column(Column::remainder())
                                        .header(20.0, |mut header| {
                                            header.col(|ui| {
                                                if ui.heading("Caretaker").clicked() {
                                                    self.ct_sort_by = true;
                                                }
                                            });
                                            header.col(|ui| {
                                                if ui.heading("Credentials").clicked() {
                                                    self.ct_sort_by = false;
                                                }
                                            });
                                            header.col(|ui| {
                                                if ui.heading("Case Count").clicked() {
                                                    self.ct_sort_by = false;
                                                }
                                            });
                                        })
                                        .body(|mut body| {
                                            for d in data {
                                                body.row(30.0, |mut row| {
                                                    row.col(|ui| {
                                                        ui.label(&d.name);
                                                    });
                                                    row.col(|ui| {
                                                        ui.label(&d.cred);
                                                    });
                                                    row.col(|ui| {
                                                        ui.label(&d.case.to_string());
                                                    });
                                                });
                                            }
                                        });
                                });
                            });
                        });
                    });
                });

            // Prepare your data
            // second row
        });
    }

    fn get_display_name(&self) -> String {
        "Assign".to_string()
    }
}

impl Assign {
    fn send_alert() {}
}
