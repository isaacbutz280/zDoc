use chrono;
use eframe::epaint;
use egui::{
    vec2, CursorIcon, Id, InnerResponse, Label, LayerId, Order, Rect, RichText, ScrollArea, Sense,
    Shape, Ui, Vec2,
};
use egui_extras::{Column, Table, TableBuilder, TableRow};

#[derive(Debug, Clone)]
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

pub struct Assign {
    location: String,
    date: Option<chrono::NaiveDate>,
    service: String,
    sort_by: bool, // true is name, false is age
    ct_sort_by: bool,
    dragging_caretaker: Option<Caretaker>,
    data_l: Vec<Task>,
    data_r: Vec<Caretaker>,
}

impl Default for Assign {
    fn default() -> Self {
        Self {
            location: Default::default(),
            date: Default::default(),
            service: Default::default(),
            sort_by: Default::default(),
            ct_sort_by: Default::default(),
            dragging_caretaker: Default::default(),
            data_l: vec![
                Task::new("Angela Johnson", "Dog Therapy"),
                Task::new("Dax Quil", "Verbal Therapy"),
                Task::new("Peter Groot", "Teethburshing"),
                Task::new("Peter Groot", "Dog Therapy"),
                Task::new("Peter Groot", "Stress Therapy"),
                Task::new("Peter Groot", "Handwashing"),
                Task::new("Gamora Thanos", "Stress Therepy"),
            ],
            data_r: vec![
                Caretaker::new("Jim Butz", "Nurse RN"),
                Caretaker::new("Dave Butz", "DSP"),
                Caretaker::new("Isaac Butz", "Developer"),
                Caretaker::new("Steve Butz", "None"),
            ],
        }
    }
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
            // Make this table drag and drop

            let mut src_row = None;
            let mut dst_row = None;

            TableBuilder::new(ui)
                .striped(false)
                .column(Column::auto().resizable(true))
                .column(Column::remainder())
                .body(|mut body| {
                    body.row(1000.0, |mut b_row| {
                        // Left side
                        b_row.col(|ui| {
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
                                        for (i, d) in self.data_l.iter().enumerate() {
                                            body.row(30.0, |mut row| {
                                                row.col(|ui| {
                                                    let response =
                                                        Assign::drop_target(ui, true, |ui| {
                                                            ui.label(match &d.assignee {
                                                                Some(a) => a.name.clone(),
                                                                None => "".to_string(),
                                                            });
                                                        })
                                                        .response;
                                                    let is_being_dragged = ui.memory(|mem| {
                                                        mem.is_anything_being_dragged()
                                                    });
                                                    if is_being_dragged
                                                        && true
                                                        && response.hovered()
                                                    {
                                                        dst_row = Some(i);
                                                    }
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
                            // Sorting state

                            self.data_r.sort_by(|a, b| {
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
                                            let id_source = "id";
                                            for (i, d) in self.data_r.iter().enumerate() {
                                                body.row(30.0, |mut row| {
                                                    let item_id = Id::new(id_source).with(i);
                                                    row.col(|ui| {
                                                        Assign::drag_source(ui, item_id, |ui| {
                                                            ui.label(&d.name);
                                                        });

                                                        if ui.memory(|mem| {
                                                            mem.is_being_dragged(item_id)
                                                        }) {
                                                            src_row = Some(i);
                                                        }
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

            if let Some(sr) = src_row {
                if let Some(dr) = dst_row {
                    if ui.input(|i| i.pointer.any_released()) {
                        self.data_l[dr].assignee = Some(self.data_r[sr].clone());
                        self.data_r[sr].case += 1;

                        // do the drop:
                    }
                }
            }
        });
    }

    fn get_display_name(&self) -> String {
        "Assign".to_string()
    }
}

impl Assign {
    fn send_alert() {}

    fn drag_source(ui: &mut Ui, id: Id, body: impl FnOnce(&mut Ui)) {
        let is_being_dragged = ui.memory(|mem| mem.is_being_dragged(id));

        if !is_being_dragged {
            let response = ui.scope(body).response;

            // Check for drags:
            let response = ui.interact(response.rect, id, Sense::drag());
            if response.hovered() {
                ui.ctx().set_cursor_icon(CursorIcon::Grab);
            }
        } else {
            ui.ctx().set_cursor_icon(CursorIcon::Grabbing);

            // Paint the body to a new layer:
            let layer_id = LayerId::new(Order::Tooltip, id);
            let response = ui.with_layer_id(layer_id, body).response;

            // Now we move the visuals of the body to where the mouse is.
            // Normally you need to decide a location for a widget first,
            // because otherwise that widget cannot interact with the mouse.
            // However, a dragged component cannot be interacted with anyway
            // (anything with `Order::Tooltip` always gets an empty [`Response`])
            // So this is fine!

            if let Some(pointer_pos) = ui.ctx().pointer_interact_pos() {
                let delta = pointer_pos - response.rect.center();
                ui.ctx().translate_layer(layer_id, delta);
            }
        }
    }

    pub fn drop_target<R>(
        ui: &mut Ui,
        can_accept_what_is_being_dragged: bool,
        body: impl FnOnce(&mut Ui) -> R,
    ) -> InnerResponse<R> {
        let is_being_dragged = ui.memory(|mem| mem.is_anything_being_dragged());

        let margin = Vec2::splat(4.0);

        let outer_rect_bounds = ui.available_rect_before_wrap();
        let inner_rect = outer_rect_bounds.shrink2(margin);
        let where_to_put_background = ui.painter().add(Shape::Noop);
        let mut content_ui = ui.child_ui(inner_rect, *ui.layout());
        let ret = body(&mut content_ui);
        let outer_rect =
            Rect::from_min_max(outer_rect_bounds.min, content_ui.min_rect().max + margin);
        let (rect, response) = ui.allocate_at_least(outer_rect.size(), Sense::hover());

        let style = if is_being_dragged && can_accept_what_is_being_dragged && response.hovered() {
            ui.visuals().widgets.active
        } else {
            ui.visuals().widgets.inactive
        };

        let mut fill = style.bg_fill;
        let mut stroke = style.bg_stroke;
        if is_being_dragged && !can_accept_what_is_being_dragged {
            fill = ui.visuals().gray_out(fill);
            stroke.color = ui.visuals().gray_out(stroke.color);
        }

        ui.painter().set(
            where_to_put_background,
            epaint::RectShape {
                rounding: style.rounding,
                fill,
                stroke,
                rect,
            },
        );

        InnerResponse::new(ret, response)
    }

    // fn ui(&mut self, ui: &mut Ui) {
    //     ui.columns(self.columns.len(), |uis| {
    //         for (col_idx, column) in self.columns.clone().into_iter().enumerate() {
    //             let ui = &mut uis[col_idx];
    //             let can_accept_what_is_being_dragged = true; // We accept anything being dragged (for now) ¯\_(ツ)_/¯
    //             let response = Assign::drop_target(ui, can_accept_what_is_being_dragged, |ui| {
    //                 ui.set_min_size(vec2(64.0, 100.0));
    //                 for (row_idx, item) in column.iter().enumerate() {
    //                     Assign::drag_source(ui, item_id, |ui| {
    //                         ui.add(Label::new(item).sense(Sense::click()));
    //                     });

    //                 }
    //             })
    //             .response;

    //         }
    //     });

    // }
}
