# CHIP-8

A CHIP-8 emulator. Written such that the frontend (as in, input and rendering) is separate from the backend, so the emulator backend could be used as a library not tied to any specific framework (the backend is even `no_std`).

## Controls (keys and corresponding CHIP-8 buttons)

| Keyboard   | CHIP-8  |
|:----------:|:-------:|
| 1 2 3 4    | 1 2 3 C |
| Q W E R    | 4 5 6 D |
| A S D F    | 7 8 9 E |
| Z X C V    | A 0 B F |

## Accuracy

I'm not very confident in audio generation, but it sort-of works. As far as other instructions, the emulator has been manually tested on the following ROMs:

* [Sergey Naydenov's](https://github.com/metteo/chip8-test-rom)
* [Timendus's](https://github.com/timendus/chip8-test-suite)
* [Corax89's](https://github.com/corax89/chip8-test-rom)
* [Delay timer and random number tests](https://github.com/mattmikolay/chip-8)

And I have automated testing for Timendus's test suite in `chip8-core/tests/`.

Note that none of these tests specifically check for SUPER-CHIP and XO-CHIP instructions or functionality (other than the "Quirks" test in Timendus's suite), so the guarantees on those emulators' accuracies are much weaker.

## TODO

* documentation
* finish the stubbed debug console
* quick save / quick load
* configurable keys
* rapid-fire keys (maybe by holding shift?)
* keys to pause, fast-forward, slow-motion, step forward
* screen-shot and/or record keys
* dialog to change emulator mode (i.e. Cosmac, SUPER-CHIP, XO-CHIP)
