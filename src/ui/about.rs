use egui::*;

pub struct About<'a> {
    show_about: &'a mut bool,
}

impl<'a> About<'a> {
    pub fn new(
        show_about: &'a mut bool,
    ) -> Self {
        Self {
            show_about,
        }
    }

    pub fn ctx(&mut self, ctx: &Context) {
        egui::Window::new("About")
            .collapsible(false)
            .open(self.show_about)
            .show(ctx, |ui| {
               ui.heading("egb");
               ui.label("A Nintendo Gameboy Emulator written in Rust");
               ui.label("by Dan Kirkham");
               ui.hyperlink("https://github.com/dankirkham/egb");
               ui.add_space(10.0);
               let version: &'static str = option_env!("CARGO_PKG_VERSION").unwrap_or("unversioned");
               ui.label(format!("Version: {version}"));
               let git_hash: &'static str = option_env!("GIT_HASH").unwrap_or("development");
               ui.label(format!("Git Commit: {git_hash}"));
               ui.add_space(10.0);
               ui.label("egb is not intended for use on a mobile device.");
               ui.label("To enable graphical debug UI, go to \"menu > developer mode\"");

               ui.add_space(10.0);
               ui.heading("Controls");
               ui.label("Up - W or Up Arrow");
               ui.label("Left - A or Left Arrow");
               ui.label("Down - S or Down Arrow");
               ui.label("Right - D or Right Arrow");
               ui.label("A - Z or , (comma)");
               ui.label("B - X or . (period)");
               ui.label("Start - Enter");
               ui.label("Select - Space");

               ui.add_space(10.0);
               ui.heading("ROM Credits");
               ui.label("2048-gb by Sangui");
               ui.hyperlink("https://github.com/Sanqui/2048-gb");
               ui.label("cpu_instr test by Blargg");
               ui.hyperlink("http://www.slack.net/~ant/");

            });
    }
}
