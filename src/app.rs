use egui::Color32;

use crate::{cv, publication, utils};

const NAME_PROJECT_PATCHING_DEBUGINFO: &str = "Patching DebugInfo Update Errors in LLVM";

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct BlogApp {
    show_publications: bool,
    show_cv: bool,
    show_project_debug_info_patching: bool,
}

impl Default for BlogApp {
    fn default() -> Self {
        Self {
            show_publications: false,
            show_cv: false,
            show_project_debug_info_patching: false,
        }
    }
}

impl BlogApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for BlogApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui
        utils::set_font_size(ctx);

        self.ui_menu(ctx);
        self.ui_content(ctx);
        self.ui_window(ctx);
    }
}

impl BlogApp {
    fn ui_menu(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
    
            egui::menu::bar(ui, |ui| {
                ui.set_height(30.0);
                egui::widgets::global_dark_light_mode_buttons(ui);
                ui.label(" | ");
    
                // NOTE: no File->Quit on web pages!
                // let is_web = cfg!(target_arch = "wasm32");
                // if !is_web {
                //     ui.menu_button("File", |ui| {
                //         if ui.button("Quit").clicked() {
                //             ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                //         }
                //     });
                //     ui.add_space(32.0);
                // }
    
                /* Publications */
                let mut publication_res = ui.button("Pulications");
    
                if self.show_publications {
                    publication_res = publication_res.highlight();
    
                }
                if publication_res.clicked() {
                    self.show_publications = !self.show_publications;
                }
                ui.label(" | ");
    
                /* Personal CV */
                let mut cv_res = ui.button("CV");            
                
                if self.show_cv {
                    cv_res = cv_res.highlight();
                }

                if cv_res.clicked() {
                    self.show_cv = !self.show_cv;
                }
            });
        });
    }

    fn ui_content(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("Shan Huang");

            self.ui_self_introduction(ui);

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                self.powered_by_egui_and_eframe(ui);
                egui::warn_if_debug_build(ui);
            });
        });
    }

    /// Draw the in-page windows
    fn ui_window(&mut self, ctx: &egui::Context) {
        egui::Window::new("Pulications")
            .min_size([300.0, 400.0])
            .open(&mut self.show_publications)
            .show(ctx, |ui| {
                publication::ui_publication(ui);
            });

        egui::Window::new("CV")
            .min_size([300.0, 400.0])
            .vscroll(true)
            .open(&mut self.show_cv)
            .show(ctx, |ui| {
                cv::ui_cv(ui);
            });

        /* Patching Debug Info Update Errors in LLVM */
        egui::Window::new(NAME_PROJECT_PATCHING_DEBUGINFO)
            .min_size([300.0, 400.0])
            .vscroll(true)
            .open(&mut self.show_project_debug_info_patching)
            .show(ctx, |ui| {
                ui.add_space(5.0);
                ui.heading("Introduction");
                ui.add_space(5.0);
                
                ui.label("2023.12 ~ 2024.07");
                ui.label("Brief intro.");

                ui.separator();

                ui.add_space(5.0);
                ui.heading("Accepcted Patches");
                ui.add_space(5.0);

                egui::Grid::new("patch_list")
                    // .spacing([50.0, 4.0])
                    .num_columns(5)
                    .striped(true)
                    .show(ui, |ui| {
                        ui.label(egui::RichText::new("#").strong().underline());
                        ui.label(egui::RichText::new("Pull Request").strong().underline());
                        ui.label(egui::RichText::new("Optimization Pass").strong().underline());
                        ui.label(egui::RichText::new("Update Op").strong().underline());
                        ui.label(egui::RichText::new("Errors").strong().underline());
                        ui.end_row();

                        let patch_list = [
                            ("76118", "CorrelatedValuePropagation", "P", 1),
                            ("86236", "GVNHoist", "M", 1),
                            ("86269", "TailRecursionElimination", "D", 1),
                            ("91443", "IndVarSimplify", "P", 4),
                            ("91581", "JumpThreading", "P", 3),
                            ("91729", "LICM", "P", 2),
                            ("91839", "LoopLoadElim", "P", 3),
                            ("92545", "NaryReassociate", "P", 2),
                            ("92859", "GVNSink", "M", 1),
                            ("95255", "Reassociate", "D", 1),
                            ("95742", "TailRecursionElimination", "D", 2),
                            ("96045", "DivRemPairs", "P", 5),
                            ("96849", "SeparateConstOffsetFromGEP", "P", 2),
                            ("96889", "JumpThreading", "P", 2),
                            ("97038", "InferAddressSpaces", "P", 1),
                            ("97085", "LoopFlatten", "P", 1),
                            ("97145", "LowerConstantIntrinsics", "P", 1),
                            ("97384", "SpeculativeExecution", "D", 1),
                            ("97389", "SimplifyCFGPass", "P", 1),
                            ("97519", "LoopStrengthReduce", "P", 2),
                            ("97662", "SimpleLoopUnswitch", "P", 6),
                            ("98789", "SimpleLoopUnswitch", "P", 4),
                        ];

                        let mut total_count = 0i32;
                        for (idx, (id, pass, op, count)) in patch_list.iter().enumerate() {
                            total_count += count;
                            ui.label(idx.to_string());
                            ui.hyperlink_to(
                                format!("#{id}"), 
                                format!("https://github.com/llvm/llvm-project/pull/{id}")
                            );
                            ui.label(*pass);
                            ui.label(egui::RichText::new(match *op {
                                "P" => "Preserving",
                                "M" => "Merging",
                                "D" => "Dropping",
                                _ => unreachable!(),
                            })
                            .color(match *op {
                                "P" => Color32::BLUE,
                                "M" => Color32::BROWN,
                                "D" => Color32::LIGHT_RED,
                                _ => unreachable!(),
                            }));
                            ui.label(count.to_string());
                            ui.end_row();
                        }
                    
                        ui.label("");
                        ui.label("");
                        ui.label(egui::RichText::new("Total patched errors").strong().underline());
                        ui.label("");
                        ui.label(total_count.to_string());
                        ui.end_row();
                    });

            });
    }

    fn ui_self_introduction(&mut self, ui: &mut egui::Ui) {
        ui.label("");
        ui.horizontal_wrapped(|ui| {
            ui.spacing_mut().item_spacing.x = 0.0;
    
            ui.label("Shan Huang is now a Ph.D. student of Software Engineering Institute, ");
            ui.hyperlink_to("East China Normal University", "https://www.ecnu.edu.cn/");
            ui.label(" (or ECNU). ");
    
            ui.label("He is supervised by Prof. ");
            ui.hyperlink_to("Ting Su", "https://tingsu.github.io/");
            ui.label(". ");
    
            ui.label("Previously, he obtained the B.S. degree at ECNU (2019-2023).");
    
        });
    
        ui.label("");
        ui.label("His research interests include but not limited to fuzzing, static analysis.");
    
        ui.label("");
        ui.horizontal_wrapped(|ui| {
            ui.label(egui::RichText::new("Projects: ").strong().underline());

            let dip_res = ui.button(NAME_PROJECT_PATCHING_DEBUGINFO);
            if dip_res.clicked() {
                self.show_project_debug_info_patching = !self.show_project_debug_info_patching;
            };
            if self.show_project_debug_info_patching {
                dip_res.highlight();
            }
        });
    }

    pub fn powered_by_egui_and_eframe(&self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 0.0;
            ui.label("Powered by ");
            ui.hyperlink_to("egui", "https://github.com/emilk/egui");
            ui.label(" and ");
            ui.hyperlink_to(
                "eframe",
                "https://github.com/emilk/egui/tree/master/crates/eframe",
            );
            ui.label(".");
        });
    }
}
