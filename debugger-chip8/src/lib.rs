extern crate chip8_core;
extern crate enum_map;
extern crate funty;
extern crate image;
extern crate once_cell;
extern crate strum;

use chip8_core::display::{COLOR_SET, SCREEN_HEIGHT, SCREEN_WIDTH};
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
    if let Some(src) = src.strip_prefix('-') {
        let parsed = parse_int::<I>(src)?;
        if parsed >> (I::BITS - 1) == I::ONE {
            return Err(format!("Number too small to fit in {} bits", I::BITS).into());
        }
        return Ok(!parsed + I::ONE);
    }
    if let Some(src) = src.strip_prefix('#') {
        Ok(I::from_str_radix(src, 16)?)
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
        DebugCommand::DumpDisplay => "dump_display <file> [scale] - write the screen contents to <file>, optionally up-scaled",
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
type CommandBody = fn(&mut Chip8Debugger, &[&str]) -> CommandResult;
static CMD_FUNCS: Lazy<EnumMap<DebugCommand, CommandBody>> = Lazy::new(|| {
    enum_map! {
        DebugCommand::Backtrace => Chip8Debugger::cmd_backtrace,
        DebugCommand::Brk => Chip8Debugger::cmd_brk,
        DebugCommand::Disassemble => Chip8Debugger::cmd_disassemble,
        DebugCommand::DumpDisplay => Chip8Debugger::cmd_dump_display,
        DebugCommand::DumpMemory => Chip8Debugger::cmd_dump_memory,
        DebugCommand::Finish => Chip8Debugger::cmd_finish,
        DebugCommand::Goto => Chip8Debugger::cmd_goto,
        DebugCommand::Help => Chip8Debugger::cmd_help,
        DebugCommand::Keys => Chip8Debugger::cmd_keys,
        DebugCommand::ListBrk => Chip8Debugger::cmd_listbrk,
        DebugCommand::LoadRom => Chip8Debugger::cmd_load_rom,
        DebugCommand::Mode => Chip8Debugger::cmd_mode,
        DebugCommand::Next => Chip8Debugger::cmd_next,
        DebugCommand::Pause => Chip8Debugger::cmd_pause,
        DebugCommand::Read => Chip8Debugger::cmd_read,
        DebugCommand::Reboot => Chip8Debugger::cmd_reboot,
        DebugCommand::RecvKey => Chip8Debugger::cmd_recvkey,
        DebugCommand::Regs => Chip8Debugger::cmd_regs,
        DebugCommand::RemBrk => Chip8Debugger::cmd_rembrk,
        DebugCommand::Resume => Chip8Debugger::cmd_resume,
        DebugCommand::SetAddr => Chip8Debugger::cmd_setaddr,
        DebugCommand::SetReg => Chip8Debugger::cmd_setreg,
        DebugCommand::Step => Chip8Debugger::cmd_step,
        DebugCommand::Timers => Chip8Debugger::cmd_timers,
        DebugCommand::ToggleKey => Chip8Debugger::cmd_toggle_key,
        DebugCommand::Write => Chip8Debugger::cmd_write,
    }
});

#[derive(Clone, Debug)]
pub struct Chip8Debugger {
    cpu: CPU,
    breaks: BTreeSet<u16>,
    paused: bool,
    has_rom: bool,
}

impl Chip8Debugger {
    #[inline]
    pub fn new(mode: Chip8Mode) -> Self {
        Chip8Debugger {
            cpu: CPU::new(mode),
            breaks: BTreeSet::default(),
            paused: true,
            has_rom: false,
        }
    }

    fn cmd_backtrace(&mut self, _args: &[&str]) -> CommandResult {
        Ok(self
            .cpu
            .call_stack
            .iter()
            .rfold(String::new(), |out, frame| {
                out + &format!("0x{:0>4X}", frame)
            }))
    }

    fn cmd_brk(&mut self, args: &[&str]) -> CommandResult {
        let brk = parse_int::<u16>(args[0])?;
        self.breaks.insert(brk);
        Ok("".into())
    }

    fn cmd_disassemble(&mut self, args: &[&str]) -> CommandResult {
        if !args.is_empty() {
            let idx = parse_int::<u16>(args[0])?;
            self.cpu
                .disassemble(idx)
                .ok_or(format!("Unknown instruction at {}", idx).into())
                .map(Into::into)
        } else {
            self.cpu
                .disassemble_next()
                .ok_or("Unknown instruction ahead".into())
                .map(Into::into)
        }
    }

    fn cmd_dump_display(&mut self, args: &[&str]) -> CommandResult {
        let scale = if args.len() > 1 {
            parse_int::<u32>(args[1])?
        } else {
            1
        };
        let buf = image::ImageBuffer::from_fn(
            scale * SCREEN_WIDTH as u32,
            scale * SCREEN_HEIGHT as u32,
            |x, y| {
                let pix = self
                    .cpu
                    .screen
                    .read_pixel_unchecked((x / scale) as u8, (y / scale) as u8);
                let color = COLOR_SET[pix];
                image::Rgb([(color >> 16) as u8, (color >> 8) as u8, color as u8])
            },
        );
        buf.save(args[0])?;
        Ok("".into())
    }

    fn cmd_dump_memory(&mut self, args: &[&str]) -> CommandResult {
        std::fs::write(args[0], self.cpu.memory)?;
        Ok("".into())
    }

    fn cmd_finish(&mut self, _args: &[&str]) -> CommandResult {
        if !self.has_rom {
            return Err(Box::new(Error::NoRomLoaded));
        }
        let mut nested = 0;
        let mut cycles = 0;
        loop {
            let next_inst = self.cpu.read_memory_word(self.cpu.pc)?;
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
            cycles += self.cpu.step()?;
        }
        Ok(format!("Stepped {} cycles before returning", cycles))
    }

    fn cmd_goto(&mut self, args: &[&str]) -> CommandResult {
        let pos = parse_int::<u16>(args[0])?;
        self.cpu.pc = pos;
        Ok("".into())
    }

    fn cmd_help(&mut self, args: &[&str]) -> CommandResult {
        if !args.is_empty() {
            if let Ok(cmd) = DebugCommand::from_str(args[0]) {
                Ok(format!("{}", CMD_HELP_TEXT[cmd]))
            } else {
                Err(format!("Unknown command `{}`.\nFor help, use `help`.", args[0]).into())
            }
        } else {
            Ok(
                DebugCommand::iter()
                    .fold(String::new(), |out, help| out + CMD_HELP_TEXT[help] + ""),
            )
        }
    }

    fn cmd_keys(&mut self, _args: &[&str]) -> CommandResult {
        Ok((0..16).fold(String::new(), |out, key| {
            if self.cpu.is_key_down(key) {
                out + &format!("{:0>4X} key", key)
            } else {
                out
            }
        }))
    }

    fn cmd_listbrk(&mut self, _args: &[&str]) -> CommandResult {
        if self.breaks.is_empty() {
            Ok("No breakpoints".into())
        } else {
            Ok(self
                .breaks
                .iter()
                .fold(String::new(), |out, brk| out + &format!("0x{:0>4X}", brk)))
        }
    }

    fn cmd_load_rom(&mut self, args: &[&str]) -> CommandResult {
        let buf = std::fs::read(args[0])?;
        self.load_rom(&buf)?;
        Ok("".into())
    }

    fn cmd_mode(&mut self, args: &[&str]) -> CommandResult {
        if !args.is_empty() {
            let mode = Chip8Mode::from_str(args[0])?;
            self.cpu = CPU::new(mode);
            self.has_rom = false;
            self.paused = true;
            Ok("".into())
        } else {
            Ok(format!("{}", self.cpu.mode))
        }
    }

    fn cmd_next(&mut self, _args: &[&str]) -> CommandResult {
        self.cpu
            .disassemble_next()
            .ok_or("Unknown instruction ahead".into())
            .map(Into::into)
    }

    fn cmd_pause(&mut self, _args: &[&str]) -> CommandResult {
        self.paused = true;
        Ok("".into())
    }

    fn cmd_read(&mut self, args: &[&str]) -> CommandResult {
        let pos = parse_int::<u16>(args[0])?;
        let byte = self.cpu.read_memory_byte(pos)?;
        Ok(format!("0x{:0>2X}", byte))
    }

    fn cmd_reboot(&mut self, _args: &[&str]) -> CommandResult {
        self.cpu = CPU::new(self.cpu.mode);
        self.has_rom = false;
        self.paused = true;
        Ok("".into())
    }

    fn cmd_recvkey(&mut self, args: &[&str]) -> CommandResult {
        let key = parse_int::<u8>(args[0])?;
        if key > 16 {
            return Err(format!("Key 0x{:X} out of range; must be 0x0-0xF", key).into());
        }
        self.cpu.press_key(key);
        self.cpu.release_key(key);
        Ok("".into())
    }

    fn cmd_regs(&mut self, _args: &[&str]) -> CommandResult {
        let mut out = String::new();
        for (reg, &val) in self.cpu.registers.iter() {
            out += &format!("{} = {}", reg, val);
        }
        out += &format!("PC = {}", self.cpu.pc);
        out += &format!("Address = {}", self.cpu.index);
        Ok(out)
    }

    fn cmd_rembrk(&mut self, args: &[&str]) -> CommandResult {
        let brk = parse_int::<u16>(args[0])?;
        if self.breaks.remove(&brk) {
            Ok("".into())
        } else {
            Err(format!("Breakpoint 0x{:0<4X} not set", brk).into())
        }
    }

    fn cmd_resume(&mut self, _args: &[&str]) -> CommandResult {
        self.paused = false;
        Ok("".into())
    }

    fn cmd_setaddr(&mut self, args: &[&str]) -> CommandResult {
        let val = parse_int::<u16>(args[0])?;
        self.cpu.index = val;
        Ok("".into())
    }

    fn cmd_setreg(&mut self, args: &[&str]) -> CommandResult {
        let reg = parse_register(args[0]).ok_or(format!("Invalid register `{}`", args[0]))?;
        let byte = parse_int::<u8>(args[1])?;
        self.cpu.registers[reg] = byte;
        Ok("".into())
    }

    fn cmd_step(&mut self, _args: &[&str]) -> CommandResult {
        if !self.has_rom {
            return Err(Box::new(Error::NoRomLoaded));
        }
        self.cpu.step()?;
        Ok("".into())
    }

    fn cmd_timers(&mut self, _args: &[&str]) -> CommandResult {
        Ok(format!(
            "Delay timer: {}\nSound timer: {}",
            self.cpu.delay_timer, self.cpu.sound_timer
        ))
    }

    fn cmd_toggle_key(&mut self, args: &[&str]) -> CommandResult {
        let key = parse_int::<u8>(args[0])?;
        if key > 16 {
            return Err(format!("Key 0x{:X} out of range; must be 0x0-0xF", key).into());
        }
        if self.cpu.is_key_down(key) {
            self.cpu.release_key(key);
        } else {
            self.cpu.press_key(key);
        }
        Ok("".into())
    }

    fn cmd_write(&mut self, args: &[&str]) -> CommandResult {
        let pos = parse_int::<u16>(args[0])?;
        let byte = parse_int::<u8>(args[1])?;
        self.cpu.write_memory_byte(pos, byte)?;
        Ok("".into())
    }

    #[inline]
    pub fn load_rom(&mut self, contents: &[u8]) -> Result<(), Error> {
        self.cpu.load_rom(contents)?;
        self.has_rom = true;
        Ok(())
    }

    pub fn execute_debug_cmd(&mut self, line: &str) -> CommandResult {
        let mut split = line.split_whitespace();
        if let Some(cmd) = split.next() {
            if let Ok(cmd) = DebugCommand::from_str(cmd) {
                let args: Vec<&str> = split.collect();
                let argc = args.len();
                let target_args = &CMD_ARGC[cmd];
                if target_args.contains(&argc) {
                    (CMD_FUNCS[cmd])(self, &args)
                } else if argc < *target_args.start() {
                    Err(format!(
                        "Expected {} more arguments for `{}`.\nFor help, use `help`.",
                        *target_args.start() - argc,
                        cmd
                    )
                    .into())
                } else {
                    Err(format!("Too many arguments to `{}`; expected {}, received {}.\nFor help, use `help`.", cmd, *target_args.end(), argc).into())
                }
            } else {
                Err(format!("Unknown command `{}`.\nFor help, use `help`.", cmd).into())
            }
        } else {
            Ok("".into())
        }
    }

    pub fn emulate_until_breakpoints(&mut self, dur: Duration) -> Result<(), Error> {
        if self.paused {
            return Ok(());
        }
        if !self.has_rom {
            return Err(Error::NoRomLoaded);
        }
        self.cpu
            .emulate_for_until(dur, |cpu| self.breaks.contains(&cpu.pc))
            .map_err(|err| match err {
                Error::EarlyExitRequested => {
                    self.paused = true;
                    Error::Breakpoint(self.cpu.pc)
                }
                x => x,
            })
    }
}

impl Default for Chip8Debugger {
    #[inline]
    fn default() -> Self {
        Chip8Debugger::new(Chip8Mode::default())
    }
}

impl std::ops::Deref for Chip8Debugger {
    type Target = CPU;
    #[inline]
    fn deref(&self) -> &CPU {
        &self.cpu
    }
}
impl std::ops::DerefMut for Chip8Debugger {
    #[inline]
    fn deref_mut(&mut self) -> &mut CPU {
        &mut self.cpu
    }
}

impl From<CPU> for Chip8Debugger {
    #[inline]
    fn from(cpu: CPU) -> Self {
        // Guesswork; if the PC's moved, there probably was a ROM loaded that it
        // executed some code for.
        let has_rom = cpu.pc != 0x200;
        Chip8Debugger {
            cpu,
            breaks: BTreeSet::default(),
            paused: true,
            has_rom,
        }
    }
}
