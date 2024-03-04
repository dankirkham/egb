use std::collections::{HashSet, VecDeque};
use std::fmt::Display;

use crate::cpu::Cpu;
use crate::memory::ProgramMemory;
use crate::registers::CpuFlags;
use crate::symbols::Symbols;

#[derive(PartialEq)]
pub enum Command {
    Continue,
    Pause,
    Step,
    StepIn,
    StepOut,
    InsertBreakpoint(u16),
    RemoveBreakpoint(u16),
}

#[derive(Default, PartialEq)]
enum State {
    #[default]
    Continue,
    StepOut(usize),
    StepIn(usize),
    Step,
    Pause,
}

pub struct Call {
    addr: u16,
    caller: u16,
    symbol: Option<String>,
}

impl Call {
    pub fn new(addr: u16, caller: u16, symbols: Option<&Symbols>) -> Self {
        let symbol = if let Some(symbols) = symbols {
            symbols.get_symbol(addr)
        } else {
            None
        };

        Self {
            addr,
            caller,
            symbol,
        }
    }
}

impl Display for Call {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(ref symbol) = self.symbol {
            write!(
                f,
                "0x{:04x} ({}) from 0x{:04x}",
                self.addr, symbol, self.caller
            )
        } else {
            write!(f, "0x{:04x} from 0x{:04x}", self.addr, self.caller)
        }
    }
}

#[derive(Default)]
pub struct Debugger {
    state: State,
    breakpoints: HashSet<u16>,
    commands: VecDeque<Command>,
    callstack: Vec<Call>,
    last_pc: Option<u16>,
    symbols: Option<Symbols>,
}

impl Debugger {
    pub fn new(symbols: Option<Symbols>) -> Self {
        Debugger {
            symbols,
            ..Default::default()
        }
    }

    pub fn reset(&mut self) {
        self.commands = Default::default();
        self.callstack = Default::default();
        self.last_pc = Default::default();
    }

    fn call(&mut self, cpu: &Cpu, addr: u16) {
        self.callstack
            .push(Call::new(addr, cpu.pc, self.symbols.as_ref()));
        if let State::StepIn(height) = self.state {
            if height == self.callstack.len() {
                self.state = State::Step;
            }
        }
    }

    pub fn is_running(&self) -> bool {
        self.state == State::Continue
    }

    fn ret(&mut self) {
        self.callstack.pop();
        if let State::StepOut(height) = self.state {
            if height == self.callstack.len() {
                self.state = State::Step;
            }
        }
    }

    pub fn command(&mut self, command: Command) {
        self.commands.push_back(command);
    }

    pub fn get_breakpoints(&self) -> Vec<u16> {
        let mut v: Vec<_> = self.breakpoints.iter().cloned().collect();
        v.sort();
        v
    }

    pub fn get_callstack(&self) -> &Vec<Call> {
        &self.callstack
    }

    pub fn tick(&mut self, cpu: &Cpu, mem: &impl ProgramMemory) -> bool {
        while let Some(command) = self.commands.pop_front() {
            match command {
                Command::Continue => {
                    self.state = State::Continue;
                }
                Command::Step => {
                    self.state = State::Step;
                }
                Command::Pause => {
                    self.state = State::Pause;
                }
                Command::StepIn => {
                    self.state = State::StepIn(self.callstack.len() + 1);
                }
                Command::StepOut => {
                    if !self.callstack.is_empty() {
                        self.state = State::StepOut(self.callstack.len() - 1);
                    }
                }
                Command::InsertBreakpoint(addr) => {
                    self.breakpoints.insert(addr);
                }
                Command::RemoveBreakpoint(addr) => {
                    self.breakpoints.remove(&addr);
                }
            }
        }

        let is_new_pc = if let Some(last_pc) = self.last_pc {
            if last_pc != cpu.pc {
                self.last_pc = Some(cpu.pc);
                true
            } else {
                false
            }
        } else {
            self.last_pc = Some(cpu.pc);
            true
        };

        if is_new_pc {
            match self.state {
                State::Continue | State::StepIn(_) | State::StepOut(_) => {
                    if self.breakpoints.contains(&cpu.pc) {
                        self.state = State::Pause;
                    }
                }
                State::Step => {
                    self.state = State::Pause;
                }
                State::Pause => {
                    // self.read(cpu, mem);
                }
            }

            // Process callstack
            match mem.get_u8(cpu.pc) {
                0xcd => self.call(cpu, mem.get_u16(cpu.pc + 1)),
                0xc4 => {
                    if !cpu.f.contains(CpuFlags::Z) {
                        self.call(cpu, mem.get_u16(cpu.pc + 1));
                    }
                }
                0xcc => {
                    if cpu.f.contains(CpuFlags::Z) {
                        self.call(cpu, mem.get_u16(cpu.pc + 1));
                    }
                }
                0xd4 => {
                    if !cpu.f.contains(CpuFlags::C) {
                        self.call(cpu, mem.get_u16(cpu.pc + 1));
                    }
                }
                0xdc => {
                    if cpu.f.contains(CpuFlags::C) {
                        self.call(cpu, mem.get_u16(cpu.pc + 1));
                    }
                }
                0xc7 => self.call(cpu, 0x00),
                0xcf => self.call(cpu, 0x08),
                0xd7 => self.call(cpu, 0x10),
                0xdf => self.call(cpu, 0x18),
                0xe7 => self.call(cpu, 0x20),
                0xef => self.call(cpu, 0x28),
                0xf7 => self.call(cpu, 0x30),
                0xff => self.call(cpu, 0x38),
                0xc9 | 0xd9 => self.ret(),
                0xc0 => {
                    if !cpu.f.contains(CpuFlags::Z) {
                        self.ret();
                    }
                }
                0xc8 => {
                    if cpu.f.contains(CpuFlags::Z) {
                        self.ret();
                    }
                }
                0xd0 => {
                    if !cpu.f.contains(CpuFlags::C) {
                        self.ret();
                    }
                }
                0xd8 => {
                    if cpu.f.contains(CpuFlags::C) {
                        self.ret();
                    }
                }
                _ => (),
            }
        }

        // Return if we are running or not
        self.state != State::Pause
    }
}
