use std::time::Duration;

use egui::*;
use egui_notify::{Anchor, Toasts};

use crate::dasm::Disassembly;
use crate::gameboy::Gameboy;
use crate::governor::Governor;
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
    screen: ViewerContext<'a>,
    viewer_select_state: ViewerSelectState<'a>,
    developer_mode: bool,
    governor: Governor,
    last_toast: Instant,
}

impl<'a> App<'a> {
    pub fn new(gameboy: Gameboy, loader: Loader, symbols: Option<Symbols>) -> Self {
        Self {
            gameboy,
            loader,
            disassembly: None,
            bp_string: "c000".to_owned(),
            toasts: Toasts::default().with_anchor(Anchor::TopRight),
            memory_viewer_state: MemoryViewerState::default(),
            disasm_panel_state: DisasmPanelState::new(symbols),
            console: String::default(),
            screen: ViewerContext::default(),
            viewer_select_state: ViewerSelectState::default(),
            developer_mode: false,
            governor: Governor::default(),
            last_toast: Instant::now(),
        }
    }
}

impl eframe::App for App<'_> {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let frame_start = Instant::now();

        self.governor.tick(&mut self.gameboy, &mut self.console);

        egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
            Toolbar::new(
                &mut self.gameboy,
                &mut self.loader,
                &mut self.developer_mode,
            )
            .ui(ui);
        });

        egui::TopBottomPanel::bottom("status").show(ctx, |ui| {
            Status::new(&mut self.toasts, &mut self.governor).ui(ui);
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
                        ViewerSelect::new(
                            &mut self.gameboy.ppu,
                            &mut self.viewer_select_state,
                        ).ui(ui);
                    });
            }

            egui::CentralPanel::default().show(ctx, |ui| {
                Viewer::new(&mut self.screen).ui(ui, "screen", self.gameboy.ppu.get_screen());
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

        let render_time = frame_start.elapsed();
        let frame_time = std::time::Duration::from_millis(1000 / REFRESH_HZ);
        match frame_time.checked_sub(render_time) {
            Some(delta) => ctx.request_repaint_after(delta),
            None => {
                if self.last_toast.elapsed().as_secs() >= 10 {
                    self.toasts
                        .warning(format!("Emulator is falling behind"))
                        .set_duration(Some(Duration::from_secs(5)));
                    self.last_toast = Instant::now();
                }
                self.governor.skip();
                ctx.request_repaint()
            }
        }
    }
}
