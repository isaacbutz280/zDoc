mod assign;

trait App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame);

    fn get_display_name(&self) -> String;
}

/**
 * Wrap app is the top level app, enca
 */
/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(Default, serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct WrapApp {
    // this how you opt-out of serialization of a member
    #[serde(skip)]
    value: f32,

    #[serde(skip)]
    apps: Vec<Box<dyn App>>,

    selcted: usize,
}

impl WrapApp {
    /// Called once before the first frame.
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.

        // Until I can store the apps vec, this won't work
        // if let Some(storage) = cc.storage {
        //     return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        // }

        Self {
            value: 2.7,
            apps: vec![Box::<assign::Assign>::default()],
            selcted: 0,
        }
    }

    fn menu_bar(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        _frame.close();
                    }
                });
            });
        });
    }

    fn side_bar(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("side_bar")
            .resizable(true)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading("zDoc");
                });

                ui.separator();

                let mut selected_anchor = self.selcted;

                for (ind, app) in self.apps.iter().enumerate() {
                    if ui
                        .selectable_label(selected_anchor == ind, app.get_display_name())
                        .clicked()
                    {
                        selected_anchor = ind;

                        // if frame.is_web() {
                        //     ui.output().open_url(format!("#{}", anchor));
                        // }
                    }
                }

                self.selcted = selected_anchor;

                ui.separator();
            });
    }
}

impl eframe::App for WrapApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        // if ctx
        //     .input_mut()
        //     .consume_key(egui::Modifiers::NONE, egui::Key::F11)
        // {
        //     frame.set_fullscreen(!frame.info().window_info.fullscreen);
        // }

        self.menu_bar(ctx, frame);

        self.side_bar(ctx, frame);

        // Can assume here
        self.apps[self.selcted].update(ctx, frame);
    }
}
