use egui::Color32;

pub fn ui_cv(ui: &mut egui::Ui) {
    education(ui);
    ui.separator();
    work_expr(ui);
    ui.separator();
    publication(ui);
    ui.separator();
    misc_expr(ui);
    ui.add_space(10.0);
}

fn education(ui: &mut egui::Ui) {
    ui.add_space(5.0);
    ui.heading("Education");
    ui.add_space(5.0);

    ui.horizontal_wrapped(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("[1] ");
        ui.label(egui::RichText::new("Ph.D. in Software Engineering").strong().underline());
        ui.label(" , East China Normal University, 2023–");
    });

    ui.horizontal_wrapped(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("[2] ");
        ui.label(egui::RichText::new("B.S. in Software Engineering").strong().underline());
        ui.label(" , East China Normal University, 2019–2023");
    });
}

fn work_expr(ui: &mut egui::Ui) {
    ui.add_space(5.0);
    ui.heading("Work Experience");
    ui.add_space(5.0);

    ui.horizontal_wrapped(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("[1] ");
        ui.label(egui::RichText::new("Teaching Assitant").strong().underline());
        ui.label(" , SE Practice, ECNU, Fall 2024");
    });
}

fn publication(ui: &mut egui::Ui) {
    ui.add_space(5.0);
    ui.heading("Publications");
    ui.add_space(5.0);

    ui.horizontal_wrapped(|ui| {
        ui.spacing_mut().item_spacing.x = 2.0;
        ui.label("[1]");
        ui.vertical(|ui| {
            ui.label(egui::RichText::new("Automata-Based Trace Analysis for Aiding Diagnosing GUI Testing Tools for Android").strong().underline());
            ui.label("Enze Ma*, Shan Huang*, Weigang He, Ting Su†, Jue Wang, Huiyu Liu, Geguang Pu, Zhendong Su");
            ui.label("In Proceedings of ESEC/FSE 2023, San Francisco, CA, USA, December 3-9, 2023.");
        });
    });
}

fn misc_expr(ui: &mut egui::Ui) {
    ui.add_space(5.0);
    ui.heading("Miscellaneous");
    ui.add_space(5.0);
    
    ui.label(egui::RichText::new("Honor & Awards")
        .strong()
        .italics()
        .color(Color32::DARK_GREEN)
    );
    ui.add_space(2.0);

    ui.horizontal_wrapped(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("[1] ");
        ui.label(egui::RichText::new("Shanghai OutStanding Graduate").strong().underline());
        ui.label(", East China Normal University, 2023");
    });

    ui.horizontal_wrapped(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("[2] ");
        ui.label(egui::RichText::new("OutStanding Graduate Thesis").strong().underline());
        ui.label(", East China Normal University, 2023");
    });

    ui.horizontal_wrapped(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("[3] ");
        ui.label(egui::RichText::new("Meritorious Winner").strong().underline());
        ui.label(", MCM/ICM, 2021");
    });

    ui.horizontal_wrapped(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("[4] ");
        ui.label(egui::RichText::new("National Scholarship").strong().underline());
        ui.label(", Ministry of Education of the PRC, 2019");
    });

    ui.add_space(2.0);
    ui.label(egui::RichText::new("Internship Experience")
        .strong()
        .italics()
        .color(Color32::DARK_GREEN)
    );
    ui.add_space(2.0);

    ui.horizontal_wrapped(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("[1] ");
        ui.label(egui::RichText::new("Software Engineer").strong().underline());
        ui.label(", Edge Mobile, STCA, Microsoft, 2022.7–2022.9");
    });
}