use std::time::Duration;

use egui::*;
use egui_notify::{Anchor, Toasts};

use crate::dasm::Disassembly;
use crate::gameboy::{Gameboy, CLOCK_SPEED_HZ};
use crate::loader::Loader;
use crate::symbols::Symbols;
use crate::time::Instant;
use crate::ui::*;

const REFRESH_HZ: u64 = 60;

pub struct App<'a> {
    gameboy: Gameboy,
    loader: Loader,
    disassembly: Option<Disassembly>,
    bp_string: String,
    toasts: Toasts,
    memory_viewer_state: MemoryViewerState,
    disasm_panel_state: DisasmPanelState,
    console: String,
    speed: Average,
    last_time: Option<Instant>,
    tile_data_image: Option<Image<'a>>,
    tile_data_texture: Option<TextureHandle>,
    background_image: Option<Image<'a>>,
    background_texture: Option<TextureHandle>,
    screen_image: Option<Image<'a>>,
    screen_texture: Option<TextureHandle>,
    developer_mode: bool,
}

impl<'a> App<'a> {
    pub fn new(gameboy: Gameboy, loader: Loader, symbols: Option<Symbols>) -> Self {
        Self {
            gameboy,
            loader,
            disassembly: None,
            bp_string: "c000".to_owned(),
            toasts: Toasts::default().with_anchor(Anchor::BottomRight),
            memory_viewer_state: MemoryViewerState::default(),
            disasm_panel_state: DisasmPanelState::new(symbols),
            console: String::default(),
            speed: Average::default(),
            last_time: None,
            tile_data_image: None,
            tile_data_texture: None,
            background_image: None,
            background_texture: None,
            screen_image: None,
            screen_texture: None,
            developer_mode: true,
        }
    }
}

impl eframe::App for App<'_> {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if let Some(last) = self.last_time {
            let now = Instant::now();
            let duration = now.duration_since(last);
            let target_duration = Duration::from_secs_f64(1. / REFRESH_HZ as f64);
            let speed = target_duration.as_secs_f64() / duration.as_secs_f64();
            self.speed.update(speed);
            self.last_time = Some(now);
        } else {
            self.last_time = Some(Instant::now());
        }

        for _ in 0..(CLOCK_SPEED_HZ / REFRESH_HZ) {
            if let Some(c) = self.gameboy.tick() {
                self.console.push(c as char);
            }
        }

        egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
            Toolbar::new(&mut self.gameboy, &mut self.loader, &mut self.developer_mode).ui(ui);
        });

        egui::TopBottomPanel::bottom("status").show(ctx, |ui| {
            Status::new(self.speed.get_average()).ui(ui);
        });

        if self.developer_mode {
            egui::SidePanel::left("dism_panel")
                .min_width(250.)
                .show(ctx, |ui| {
                    DisasmPanel::new(
                        &mut self.disassembly,
                        &self.gameboy.mem,
                        self.gameboy.cpu.pc,
                        &mut self.disasm_panel_state,
                        &mut self.toasts,
                    )
                    .ui(ui);
                    ui.separator();
                    Breakpoints::new(
                        &mut self.gameboy.debugger,
                        &mut self.toasts,
                        &mut self.bp_string,
                    )
                    .ui(ui);
                    ui.separator();
                    Callstack::new(&mut self.gameboy.debugger).ui(ui);
                });

            egui::SidePanel::right("cpu_panel")
                .min_width(250.)
                .show(ctx, |ui| {
                    CpuPanel::new(&self.gameboy.cpu).ui(ui);
                    ui.separator();
                    Registers::new(&self.gameboy.mem).ui(ui);
                });

            egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
                ui.horizontal(|ui| {
                    MemoryViewer::new(
                        &self.gameboy.mem,
                        &self.gameboy.cpu,
                        &mut self.toasts,
                        &mut self.memory_viewer_state,
                    )
                    .ui(ui);
                    ui.separator();
                    SerialConsole::new(&mut self.console).ui(ui);
                });
            });
        }

        egui::CentralPanel::default().show(ctx, |_ui| {
            if self.developer_mode {
                egui::SidePanel::right("tile_panel")
                    .min_width(250.)
                    .show(ctx, |ui| {
                        TileData::new(
                            &self.gameboy.mem,
                            &mut self.tile_data_image,
                            &mut self.tile_data_texture,
                        )
                        .ui(ui);
                        ui.separator();
                        Background::new(
                            &self.gameboy.ppu,
                            &mut self.background_image,
                            &mut self.background_texture,
                        )
                        .ui(ui);
                    });
            }

            egui::CentralPanel::default().show(ctx, |ui| {
                Screen::new(
                    &self.gameboy.ppu,
                    &mut self.screen_image,
                    &mut self.screen_texture,
                )
                .ui(ui);
            });
        });

        self.toasts.show(ctx);

        ctx.input(|i| {
            let buttons = &mut self.gameboy.mem.buttons;
            buttons.start = !i.key_down(Key::Enter);
            buttons.select = !i.key_down(Key::Space);
            buttons.a = !(i.key_down(Key::Z) || i.key_down(Key::Comma));
            buttons.b = !(i.key_down(Key::X) || i.key_down(Key::Period));
            buttons.down = !(i.key_down(Key::ArrowDown) || i.key_down(Key::S));
            buttons.up = !(i.key_down(Key::ArrowUp) || i.key_down(Key::W));
            buttons.left = !(i.key_down(Key::ArrowLeft) || i.key_down(Key::A));
            buttons.right = !(i.key_down(Key::ArrowRight) || i.key_down(Key::D));
        });

        let render_time = self.last_time.unwrap().elapsed();
        let frame_time = std::time::Duration::from_millis(1000 / REFRESH_HZ);
        match frame_time.checked_sub(render_time) {
            Some(delta) => ctx.request_repaint_after(delta),
            None => ctx.request_repaint(),
        }
    }
}
