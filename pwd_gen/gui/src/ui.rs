impl eframe::epi::App for crate::generator::Generator {
    fn update(&mut self, ctx: &eframe::egui::CtxRef, _frame: &mut eframe::epi::Frame<'_>) {
        self.update_ui(ctx);
    }

    fn name(&self) -> &str {
        "Password Generator"
    }
}
