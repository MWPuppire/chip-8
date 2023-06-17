use chip8_lib::*;

const SPLASH_ROM: &[u8] = include_bytes!("./timendus-tests/1-chip8-logo.ch8");
const IBM_ROM: &[u8] = include_bytes!("./timendus-tests/2-ibm-logo.ch8");
const CORAX_ROM: &[u8] = include_bytes!("./timendus-tests/3-corax+.ch8");
const FLAGS_ROM: &[u8] = include_bytes!("./timendus-tests/4-flags.ch8");
const QUIRKS_ROM: &[u8] = include_bytes!("./timendus-tests/5-quirks.ch8");
// Not sure how to make an automated test for this ROM
// const KEYPAD_ROM: &[u8] = include_bytes!("./timendus-tests/6-keypad.ch8");

fn insta_settings() -> insta::Settings {
    let mut settings = insta::Settings::clone_current();
    // keep different feature-sets separate with CHIP-8 modes
    settings.set_snapshot_path(format!("{} snapshots", Chip8Mode::default()));
    settings
}

fn make_emu(rom: &[u8]) -> CPU {
    let mut emu = CPU::new(Chip8Mode::default());
    // Consistent snapshot results
    emu.reseed(0x0);
    emu.load_rom(rom).unwrap();
    emu
}

#[test]
fn test_splash_screen() {
    let _guard = insta_settings().bind_to_scope();
    let mut emu = make_emu(SPLASH_ROM);
    // README: Run the ROM for 39 cycles to see this splash screen on the
    // display... If you run the ROM for more than 39 cycles, it will enter an
    // endless loop.
    for _ in 0..39 {
        emu.step().unwrap();
    }
    insta::allow_duplicates! {
        // Confirm the infinite loop doesn't change any state
        insta::assert_debug_snapshot!(emu);
        for _ in 0..100 {
            emu.step().unwrap();
        }
        insta::assert_debug_snapshot!(emu);
    }
}

#[test]
fn test_ibm_logo() {
    let _guard = insta_settings().bind_to_scope();
    let mut emu = make_emu(IBM_ROM);
    // README: Run the ROM for 20 cycles to see the IBM logo on the display...
    // If you run the ROM for more than 20 cycles, it will enter an endless
    // loop.
    for _ in 0..20 {
        emu.step().unwrap();
    }
    insta::allow_duplicates! {
        // Confirm the infinite loop doesn't change any state
        insta::assert_debug_snapshot!(emu);
        for _ in 0..100 {
            emu.step().unwrap();
        }
        insta::assert_debug_snapshot!(emu);
    }
}

#[test]
fn test_corax_opcodes() {
    let _guard = insta_settings().bind_to_scope();
    let mut emu = make_emu(CORAX_ROM);
    // 284 cycles not documented, calculated by my own testing.
    for _ in 0..284 {
        emu.step().unwrap();
    }
    insta::assert_debug_snapshot!(emu);
}

#[test]
fn test_flags() {
    let _guard = insta_settings().bind_to_scope();
    let mut emu = make_emu(FLAGS_ROM);
    // 945 cycles not documented, calculated by my own testing.
    for _ in 0..945 {
        emu.step().unwrap();
    }
    insta::assert_debug_snapshot!(emu);
}

#[test]
fn test_quirks() {
    let _guard = insta_settings().bind_to_scope();
    let mut emu = make_emu(QUIRKS_ROM);
    let mode = emu.mode as u8;
    emu.memory[0x1FF] = mode + 1;
    // cycle count not documented, calculated by my own testing
    const CYCLE_COUNTS: [usize; 3] = [
        2400, // COSMAC
        2396, // Super-CHIP
        2407, // XO-CHIP
    ];
    for _ in 0..CYCLE_COUNTS[mode as usize] {
        emu.step().unwrap();
    }
    insta::assert_debug_snapshot!(emu);
}
