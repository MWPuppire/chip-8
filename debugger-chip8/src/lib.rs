use chip8_core::{Chip8Mode, Error, Register, CPU};
use enum_map::{enum_map, EnumMap};
use funty::Unsigned;
use once_cell::sync::Lazy;
use std::collections::BTreeSet;
use std::ops::RangeInclusive;
use std::str::FromStr;
use std::time::Duration;
use strum::{Display, EnumIter, EnumString, IntoEnumIterator};

fn parse_int<I: Unsigned>(src: &str) -> Result<I, Box<dyn std::error::Error>> {
    if src.starts_with("-") {
        let parsed = parse_int::<I>(src)?;
        if parsed >> (I::BITS - 1) == I::ONE {
            return Err(format!("Number too small to fit in {} bits", I::BITS).into());
        }
        return Ok(!parsed + I::ONE);
    }
    if src.starts_with("#") {
        Ok(I::from_str_radix(&src[1..], 16)?)
    } else if src.starts_with("0x") || src.starts_with("0X") {
        Ok(I::from_str_radix(&src[2..], 16)?)
    } else if src.starts_with("0b") || src.starts_with("0B") {
        Ok(I::from_str_radix(&src[2..], 2)?)
    } else {
        Ok(I::from_str_radix(src, 10)?)
    }
}

fn parse_register(src: &str) -> Option<Register> {
    Register::by_name(src).or_else(|| {
        let idx = parse_int::<u8>(src).ok()?;
        idx.try_into().ok()
    })
}

#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    EnumString,
    Display,
    EnumIter,
    enum_map::Enum,
)]
#[strum(serialize_all = "snake_case")]
enum DebugCommand {
    Backtrace,
    Brk,
    Disassemble,
    DumpDisplay,
    DumpMemory,
    Finish,
    Goto,
    Help,
    Keys,
    #[strum(serialize = "listbrk")]
    ListBrk,
    LoadRom,
    Mode,
    Next,
    Pause,
    Read,
    Reboot,
    #[strum(serialize = "recvkey")]
    RecvKey,
    Regs,
    #[strum(serialize = "rembrk")]
    RemBrk,
    Resume,
    #[strum(serialize = "setaddr")]
    SetAddr,
    #[strum(serialize = "setreg")]
    SetReg,
    Step,
    Timers,
    ToggleKey,
    Write,
}

static CMD_ARGC: Lazy<EnumMap<DebugCommand, RangeInclusive<usize>>> = Lazy::new(|| {
    enum_map! {
        DebugCommand::Backtrace => 0..=0,
        DebugCommand::Brk => 1..=1,
        DebugCommand::Disassemble => 0..=1,
        DebugCommand::DumpDisplay => 1..=1,
        DebugCommand::DumpMemory => 1..=1,
        DebugCommand::Finish => 0..=0,
        DebugCommand::Goto => 1..=1,
        DebugCommand::Help => 0..=1,
        DebugCommand::Keys => 0..=0,
        DebugCommand::ListBrk => 0..=0,
        DebugCommand::LoadRom => 1..=1,
        DebugCommand::Mode => 0..=1,
        DebugCommand::Next => 0..=0,
        DebugCommand::Pause => 0..=0,
        DebugCommand::Read => 1..=1,
        DebugCommand::Reboot => 0..=0,
        DebugCommand::RecvKey => 1..=1,
        DebugCommand::Regs => 0..=0,
        DebugCommand::RemBrk => 1..=1,
        DebugCommand::Resume => 0..=0,
        DebugCommand::SetAddr => 1..=1,
        DebugCommand::SetReg => 2..=2,
        DebugCommand::Step => 0..=0,
        DebugCommand::Timers => 0..=0,
        DebugCommand::ToggleKey => 1..=1,
        DebugCommand::Write => 2..=2,
    }
});

static CMD_HELP_TEXT: Lazy<EnumMap<DebugCommand, &'static str>> = Lazy::new(|| {
    enum_map! {
        DebugCommand::Backtrace => "backtrace - display the current call stack",
        DebugCommand::Brk => "brk <x> - halt when PC reaches <x>",
        DebugCommand::Disassemble => "disassemble [x] - disassemble the instruction at <x> or PC",
        DebugCommand::DumpDisplay => "dump_display <file> - write the screen contents to JPEG <file>",
        DebugCommand::DumpMemory => "dump_memory <file> - write memory contents to binary <file>",
        DebugCommand::Finish => "run until the current function returns",
        DebugCommand::Goto => "goto <x> - set PC to <x>",
        DebugCommand::Help => "help [cmd] - display help text for <cmd> or all commands",
        DebugCommand::Keys => "keys - display currently held keys",
        DebugCommand::ListBrk => "listbrk - list all breakpoints",
        DebugCommand::LoadRom => "load_rom <file> - load a new ROM <file>, resetting the emulator",
        DebugCommand::Mode => "mode [mode] - query the current emulation mode or change it to <mode>",
        DebugCommand::Next => "next - print the next instruction without executing it",
        DebugCommand::Pause => "pause - pause execution",
        DebugCommand::Read => "read <x> - read byte at memory <x> and display it",
        DebugCommand::Reboot => "reboot - shut down and reboot CPU, resetting the emulator (and unloading the ROM)",
        DebugCommand::RecvKey => "recvkey <key> - press and release <key>",
        DebugCommand::Regs => "regs - dump all registers",
        DebugCommand::RemBrk => "rembrk <x> - remove the breakpoint at <x>",
        DebugCommand::Resume => "resume - start or continue execution",
        DebugCommand::SetAddr => "setaddr <x> - set the address register to <x>",
        DebugCommand::SetReg => "set <x> <y> - set register <x> to byte <y>",
        DebugCommand::Step => "step - execute only the next instruction",
        DebugCommand::Timers => "timers - display the current timer status",
        DebugCommand::ToggleKey => "toggle_key <key> - toggle holding a key down",
        DebugCommand::Write => "write <x> <y> - write byte <y> to memory <x>",
    }
});

type CommandResult = Result<String, Box<dyn std::error::Error>>;
type CommandBody = fn(&mut Chip8Debugger, &mut CPU, &[&str]) -> CommandResult;
static CMD_FUNCS: Lazy<EnumMap<DebugCommand, CommandBody>> = Lazy::new(|| {
    enum_map! {
        DebugCommand::Backtrace => Chip8Debugger::backtrace,
        DebugCommand::Brk => Chip8Debugger::brk,
        DebugCommand::Disassemble => Chip8Debugger::disassemble,
        DebugCommand::DumpDisplay => Chip8Debugger::dump_display,
        DebugCommand::DumpMemory => Chip8Debugger::dump_memory,
        DebugCommand::Finish => Chip8Debugger::finish,
        DebugCommand::Goto => Chip8Debugger::goto,
        DebugCommand::Help => Chip8Debugger::help,
        DebugCommand::Keys => Chip8Debugger::keys,
        DebugCommand::ListBrk => Chip8Debugger::listbrk,
        DebugCommand::LoadRom => Chip8Debugger::load_rom,
        DebugCommand::Mode => Chip8Debugger::mode,
        DebugCommand::Next => Chip8Debugger::next,
        DebugCommand::Pause => Chip8Debugger::pause,
        DebugCommand::Read => Chip8Debugger::read,
        DebugCommand::Reboot => Chip8Debugger::reboot,
        DebugCommand::RecvKey => Chip8Debugger::recvkey,
        _ => todo!(),
    }
});

#[derive(Clone, Debug, Default)]
pub struct Chip8Debugger {
    breaks: BTreeSet<u16>,
    paused: bool,
    has_rom: bool,
}

impl Chip8Debugger {
    fn backtrace(&mut self, cpu: &mut CPU, _args: &[&str]) -> CommandResult {
        Ok(cpu.call_stack.iter().rfold(String::new(), |out, frame| {
            out + &format!("0x{:0>4X}\n", frame)
        }))
    }

    fn brk(&mut self, _cpu: &mut CPU, args: &[&str]) -> CommandResult {
        let brk = parse_int::<u16>(args[0])?;
        self.breaks.insert(brk);
        Ok("".into())
    }

    fn disassemble(&mut self, cpu: &mut CPU, args: &[&str]) -> CommandResult {
        if !args.is_empty() {
            let idx = parse_int::<u16>(args[0])?;
            cpu.disassemble(idx)
                .ok_or(format!("Unknown instruction at {}\n", idx).into())
                .map(Into::into)
        } else {
            cpu.disassemble_next()
                .ok_or("Unknown instruction ahead\n".into())
                .map(Into::into)
        }
    }

    fn dump_display(&mut self, _cpu: &mut CPU, _args: &[&str]) -> CommandResult {
        todo!();
    }

    fn dump_memory(&mut self, cpu: &mut CPU, args: &[&str]) -> CommandResult {
        std::fs::write(args[0], cpu.memory)?;
        Ok("".into())
    }

    fn finish(&mut self, cpu: &mut CPU, _args: &[&str]) -> CommandResult {
        let mut nested = 0;
        let mut cycles = 0;
        loop {
            let next_inst = cpu.read_memory_word(cpu.pc)?;
            // RET
            if next_inst == 0x00EE {
                if nested == 0 {
                    break;
                } else {
                    nested -= 1;
                }
            } else if (next_inst & 0xF000) == 0x2000 {
                nested += 1;
            }
            cycles += cpu.step()?;
        }
        Ok(format!("Stepped {} cycles before returning\n", cycles))
    }

    fn goto(&mut self, cpu: &mut CPU, args: &[&str]) -> CommandResult {
        let pos = parse_int::<u16>(args[0])?;
        cpu.pc = pos;
        Ok("".into())
    }

    fn help(&mut self, _cpu: &mut CPU, args: &[&str]) -> CommandResult {
        if !args.is_empty() {
            if let Ok(cmd) = DebugCommand::from_str(args[0]) {
                Ok(format!("{}\n", CMD_HELP_TEXT[cmd]))
            } else {
                Err(format!("Unknown command `{}`.\nFor help, use `help`.\n", args[0]).into())
            }
        } else {
            Ok(DebugCommand::iter()
                .fold(String::new(), |out, help| out + CMD_HELP_TEXT[help] + "\n"))
        }
    }

    fn keys(&mut self, cpu: &mut CPU, _args: &[&str]) -> CommandResult {
        Ok((0..16).fold(String::new(), |out, key| {
            if cpu.is_key_down(key) {
                out + &format!("{:0>4X} key\n", key)
            } else {
                out
            }
        }))
    }

    fn listbrk(&mut self, _cpu: &mut CPU, _args: &[&str]) -> CommandResult {
        if self.breaks.is_empty() {
            Ok("No breakpoints\n".into())
        } else {
            Ok(self
                .breaks
                .iter()
                .fold(String::new(), |out, brk| out + &format!("0x{:0>4X}\n", brk)))
        }
    }

    fn load_rom(&mut self, cpu: &mut CPU, args: &[&str]) -> CommandResult {
        let buf = std::fs::read(args[0])?;
        cpu.load_rom(&buf)?;
        self.has_rom = true;
        Ok("".into())
    }

    fn mode(&mut self, cpu: &mut CPU, args: &[&str]) -> CommandResult {
        if !args.is_empty() {
            let mode = Chip8Mode::from_str(args[0])?;
            let _ = std::mem::replace(cpu, CPU::new(mode));
            self.has_rom = false;
            self.paused = true;
            Ok("".into())
        } else {
            Ok(format!("{}\n", cpu.mode))
        }
    }

    fn next(&mut self, cpu: &mut CPU, _args: &[&str]) -> CommandResult {
        cpu.disassemble_next()
            .ok_or("Unknown instruction ahead\n".into())
            .map(Into::into)
    }

    fn pause(&mut self, _cpu: &mut CPU, _args: &[&str]) -> CommandResult {
        self.paused = true;
        Ok("".into())
    }

    fn read(&mut self, cpu: &mut CPU, args: &[&str]) -> CommandResult {
        let pos = parse_int::<u16>(args[0])?;
        let byte = cpu.read_memory_byte(pos)?;
        Ok(format!("0x{:0>2X}", byte))
    }

    fn reboot(&mut self, cpu: &mut CPU, _args: &[&str]) -> CommandResult {
        let _ = std::mem::replace(cpu, CPU::new(cpu.mode));
        self.has_rom = false;
        self.paused = true;
        Ok("".into())
    }

    fn recvkey(&mut self, cpu: &mut CPU, args: &[&str]) -> CommandResult {
        let key = parse_int::<u8>(args[0])?;
        if key > 16 {
            return Err(format!("Key 0x{:X} out of range; must be 0x0-0xF", key).into());
        }
        cpu.press_key(key);
        cpu.release_key(key);
        Ok("".into())
    }

    pub fn execute_line(&mut self, cpu: &mut CPU, line: &str) -> CommandResult {
        let mut split = line.split_whitespace();
        if let Some(cmd) = split.next() {
            if let Ok(cmd) = DebugCommand::from_str(cmd) {
                let args: Vec<&str> = split.collect();
                let argc = args.len();
                let target_args = &CMD_ARGC[cmd];
                if target_args.contains(&argc) {
                    (CMD_FUNCS[cmd])(self, cpu, &args)
                } else if argc < *target_args.start() {
                    Err(format!(
                        "Expected {} more arguments for `{}`.\nFor help, use `help`.\n",
                        *target_args.start() - argc,
                        cmd
                    )
                    .into())
                } else {
                    Err(format!("Too many arguments to `{}`; expected {}, received {}.\nFor help, use `help`.\n", cmd, *target_args.end(), argc).into())
                }
            } else {
                Err(format!("Unknown command `{}`.\nFor help, use `help`.\n", cmd).into())
            }
        } else {
            return Ok("".into());
        }
    }

    pub fn emulate_until_breakpoints(&mut self, cpu: &mut CPU, dur: Duration) -> Result<(), Error> {
        if self.paused {
            return Ok(());
        }
        if !self.has_rom {
            return Err(Error::NoRomLoaded);
        }
        cpu.emulate_for_until(dur, |cpu| self.breaks.contains(&cpu.pc))
            .map_err(|err| match err {
                Error::EarlyExitRequested => {
                    self.paused = true;
                    Error::Breakpoint(cpu.pc)
                }
                x => x,
            })
    }
}
