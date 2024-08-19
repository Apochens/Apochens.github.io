pub fn ui_publication(ui: &mut egui::Ui) {
    ui.horizontal_wrapped(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("[1] ");
        ui.vertical(|ui| {
            ui.label(egui::RichText::new("Automata-Based Trace Analysis for Aiding Diagnosing GUI Testing Tools for Android").strong().underline());
            ui.label("Enze Ma*, Shan Huang*, Weigang He, Ting Su†, Jue Wang, Huiyu Liu, Geguang Pu, Zhendong Su");
            ui.label("In Proceedings of ESEC/FSE 2023, San Francisco, CA, USA, December 3-9, 2023.");

            ui.horizontal_wrapped(|ui| {
                ui.label("[ ");
                ui.hyperlink_to("PDF", "assets/fse23-ddroid.pdf");
                ui.label(" | ");
                ui.hyperlink_to("Tool", "https://github.com/DDroid-Android/home");
                ui.label(" ] (*Equal contribution)");
            });
        });
    });

    ui.add_space(10.0);
}