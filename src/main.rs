#[cfg(not(target_arch = "wasm32"))]
fn main() -> Result<(), std::io::Error> {
    use clap::Parser;
    use egb::app::App;
    use egb::args::Args;
    use egb::loader::Loader;
    use egb::rom::Rom;
    use egb::symbols::Symbols;
    use std::fs::File;

    fn load_symbols(path: Option<String>) -> Result<Option<Symbols>, std::io::Error> {
        if let Some(ref path) = path {
            let file = File::open(path)?;
            let symbols = Symbols::try_from(file)?;
            Ok(Some(symbols))
        } else {
            Ok(None)
        }
    }

    let args = Args::parse();

    let rom = args.rom.parse::<Rom>().unwrap();

    if rom == Rom::File {
        assert!(args.rom_file.is_some());
    }

    let symbols = load_symbols(args.symbols)?;

    let loader = Loader::new(rom, args.rom_file).with_symbols(symbols.clone());
    let gameboy = loader.load_rom()?;

    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1920.0, 1080.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Gameboy Emulator",
        options,
        Box::new(|_| {
            let app = App::new(gameboy, loader, symbols);
            Box::new(app)
        }),
    )
    .unwrap();

    Ok(())
}

#[cfg(target_arch = "wasm32")]
fn main() {
    use egb::app::App;
    use egb::loader::Loader;
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    let web_options = eframe::WebOptions::default();

    let loader = Loader::default();
    let gameboy = loader.load_rom().unwrap();

    wasm_bindgen_futures::spawn_local(async {
        eframe::WebRunner::new()
            .start(
                "the_canvas_id", // hardcode it
                web_options,
                Box::new(|_| {
                    let app = App::new(gameboy, loader, None);
                    Box::new(app)
                }),
            )
            .await
            .expect("failed to start eframe");
    });
}
