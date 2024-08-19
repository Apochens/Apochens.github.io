pub fn set_font_size(ctx: &egui::Context) {
    let mut style = (*ctx.style()).clone();

        for (text_style, font_id) in &mut style.text_styles {
            font_id.size = match text_style {
                egui::TextStyle::Heading => 20.0,
                egui::TextStyle::Body => 15.0,
                egui::TextStyle::Monospace => 15.0,
                egui::TextStyle::Button => 15.0,
                egui::TextStyle::Small => 12.0,
                egui::TextStyle::Name(_) => font_id.size,
            }
        }

        ctx.set_style(style);
}