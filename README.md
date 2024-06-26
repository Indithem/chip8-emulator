# CHIP8 emulator

My take on the hello world of emulators.

## Step by step execution

If you, like me also want to make an chip8 interpretor, but you got some 
test cases failing, and you wanna see where you go wrong, then this emulator
might help you. I made a small feature where you can pause the cpu for every
instruction. To enable that, add a small flag `-p simple`.

A dump of all cpu registers is printed onto the console. If you also want to see 
dump of memory add `-pmem` flag.

(Maybe this is the thing that caught your attention onto my emulator 😝)

## Usage
To compile, just do `cargo build --release`. You can run the executable directly with `cargo r --release -- <ARGUMENTS TO EXECUTABLE>`

Here is the help output of the emulator.

```
Usage: chip-8-emulator.exe [OPTIONS] <ROM_PATH>

Arguments:
  <ROM_PATH>
          Path to the rom file

Options:
  -p, --pauses <PAUSES>
          Run with [p]auses the emulator will wait for input after each cycle

          All dump information is sent to the stdout. The debug level must be atleast emulator-only

          [default: none]

          Possible values:
          - none:        Run at almost native speed
          - simple:      Waits for input after each CPU cycle
          - with-memory: Dumps memory after each cycle. shortword/alias: mem

  -d, --debug <DEBUG>
          Debug level

          [default: emulator-only]

          Possible values:
          - none:          Donot print any debug messages
          - verbose:       Print all debug messages that can be fetched
          - emulator-only: Print only the debug messages generated by the emulator

  -h, --help
          Print help (see a summary with '-h')
```
## Customizations
No plans yet to make a config files for customizations, but, you can edit the source code for your customizations.

### Pixel Colors
Change the constant ON|OFF_PIXEL_COLOR. 
It is located in src/graphics.rs
Format is [R, G, B, Alpha] all from 0 to 255

### Controls
Find out your keyboard bindings from winit's documentation.
Edit the match statement in src/graphics.rs, at fn `window_event`- case WindowEvent::KeyboardInput

More importantly, extend/modify the function `from_key_code` in src/input.rs

## Dump Messages

You can customize the dump messages by editing the dump functions of CPU in src/cpu.rs

Example CPU dump,
```
2024-06-25T18:22:04.913556Z  INFO chip_8_emulator::cpu: Starting Cycle: 39, CPU state: CPU Dump:
            Stack: []
            IRegister: 0x7570
            Registry Memory: [48, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
            Current Opcode pointer: 0x0250
            Opcode: Some("124E")
```

Example CPU dump with memory
```
2024-06-25T18:24:03.576140Z  INFO chip_8_emulator::cpu: Starting Cycle: 977, CPU state: CPU Dump:
            Memory: Memory([240, 144, 144, 144, 240, 32, 96, 32, 32, 112, 240, 16, 240, 128, 240, 240, 16, 240, 16, 240, 144, 144, 240, 16, 16, 240, 128, 240, 16, 240, 240, 128, 240, 144, 240, 240, 16, 32, 64, 64, 240, 144, 240, 144, 240, 240, 144, 240, 16, 240, 240, 144, 240, 144, 144, 224, 144, 224, 144, 224, 240, 128, 128, 128, 240, 224, 144, 144, 144, 224, 240, 128, 240, 128, 240, 240, 128, 240, 128, 128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 18, 160, 96, 0, 224, 161, 18, 4, 112, 1, 64, 16, 0, 238, 18, 4, 252, 101, 34, 118, 65, 0, 0, 238, 128, 16, 34, 118, 66, 0, 0, 238, 128, 32, 34, 118, 67, 0, 0, 238, 128, 48, 34, 118, 68, 0, 0, 238, 128, 64, 34, 118, 69, 0, 0, 238, 128, 80, 34, 118, 70, 0, 0, 238, 128, 96, 34, 118, 71, 0, 0, 238, 128, 112, 34, 118, 72, 0, 0, 238, 128, 128, 34, 118, 73, 0, 0, 238, 128, 144, 34, 118, 74, 0, 0, 238, 128, 160, 34, 118, 75, 0, 0, 238, 128, 176, 34, 118, 76, 0, 0, 238, 128, 192, 34, 118, 0, 238, 165, 87, 240, 30, 221, 228, 125, 4, 0, 238, 165, 91, 142, 208, 142, 238, 142, 238, 254, 30, 218, 180, 122, 5, 0, 238, 165, 88, 146, 192, 165, 85, 123, 1, 218, 179, 122, 4, 123, 255, 0, 238, 0, 224, 106, 50, 107, 27, 166, 9, 218, 180, 106, 58, 166, 13, 218, 180, 109, 0, 110, 0, 165, 247, 34, 16, 106, 22, 107, 0, 97, 15, 109, 1, 34, 128, 99, 15, 111, 20, 131, 241, 111, 0, 98, 50, 130, 17, 142, 240, 108, 63, 34, 144, 130, 224, 108, 0, 34, 144, 130, 48, 108, 31, 34, 144, 122, 5, 109, 2, 34, 128, 99, 15, 111, 20, 131, 242, 111, 0, 98, 50, 130, 18, 142, 240, 108, 2, 34, 144, 130, 224, 108, 0, 34, 144, 130, 48, 108, 4, 34, 144, 123, 5, 106, 0, 109, 3, 34, 128, 99, 15, 111, 20, 131, 243, 111, 0, 98, 50, 130, 19, 142, 240, 108, 61, 34, 144, 130, 224, 108, 0, 34, 144, 130, 48, 108, 27, 34, 144, 122, 5, 109, 4, 34, 128, 111, 20, 143, 20, 132, 240, 99, 15, 111, 20, 131, 244, 111, 170, 98, 50, 130, 20, 142, 240, 108, 65, 34, 144, 130, 224, 108, 0, 34, 144, 130, 48, 108, 35, 34, 144, 130, 64, 108, 0, 34, 144, 122, 1, 109, 5, 34, 128, 111, 20, 143, 21, 132, 240, 99, 20, 111, 15, 131, 245, 101, 10, 111, 10, 133, 245, 133, 240, 111, 170, 98, 50, 130, 21, 53, 1, 111, 2, 142, 240, 108, 35, 34, 144, 130, 224, 108, 1, 34, 144, 130, 48, 108, 5, 34, 144, 130, 64, 108, 1, 34, 144, 123, 5, 106, 0, 109, 6, 34, 128, 111, 60, 143, 246, 131, 240, 111, 170, 98, 60, 130, 38, 142, 240, 108, 30, 34, 144, 130, 224, 108, 0, 34, 144, 130, 48, 108, 0, 34, 144, 122, 5, 109, 7, 34, 128, 111, 10, 143, 23, 132, 240, 99, 15, 111, 20, 131, 247, 101, 10, 111, 10, 133, 247, 133, 240, 111, 170, 98, 15, 97, 50, 130, 23, 53, 1, 111, 2, 142, 240, 108, 35, 34, 144, 130, 224, 108, 1, 34, 144, 130, 48, 108, 5, 34, 144, 130, 64, 108, 1, 34, 144, 122, 1, 109, 14, 34, 128, 111, 50, 143, 254, 131, 240, 111, 170, 98, 50, 130, 46, 142, 240, 108, 100, 34, 144, 130, 224, 108, 0, 34, 144, 130, 48, 108, 0, 34, 144, 109, 0, 110, 16, 165, 253, 34, 16, 106, 22, 107, 16, 97, 100, 109, 4, 34, 128, 111, 200, 143, 20, 132, 240, 99, 100, 111, 200, 131, 244, 111, 170, 98, 200, 130, 20, 142, 240, 108, 44, 34, 144, 130, 224, 108, 1, 34, 144, 130, 48, 108, 44, 34, 144, 130, 64, 108, 1, 34, 144, 122, 1, 109, 5, 34, 128, 111, 95, 143, 21, 132, 240, 99, 95, 111, 100, 131, 245, 111, 170, 98, 95, 130, 21, 142, 240, 108, 251, 34, 144, 130, 224, 108, 0, 34, 144, 130, 48, 108, 251, 34, 144, 130, 64, 108, 0, 34, 144, 123, 5, 106, 0, 109, 6, 34, 128, 111, 61, 143, 246, 131, 240, 111, 170, 98, 61, 130, 38, 142, 240, 108, 30, 34, 144, 130, 224, 108, 1, 34, 144, 130, 48, 108, 1, 34, 144, 122, 5, 109, 7, 34, 128, 111, 105, 143, 23, 132, 240, 99, 105, 111, 100, 131, 247, 111, 170, 98, 105, 130, 23, 142, 240, 108, 251, 34, 144, 130, 224, 108, 0, 34, 144, 130, 48, 108, 251, 34, 144, 130, 64, 108, 0, 34, 144, 122, 1, 109, 14, 34, 128, 111, 188, 143, 254, 131, 240, 111, 170, 98, 188, 130, 46, 142, 240, 108, 120, 34, 144, 130, 224, 108, 1, 34, 144, 130, 48, 108, 1, 34, 144, 109, 0, 110, 27, 166, 3, 34, 16, 106, 22, 107, 27, 109, 15, 34, 128, 122, 255, 109, 14, 34, 128, 165, 68, 97, 16, 241, 30, 96, 170, 240, 85, 165, 84, 240, 101, 130, 0, 108, 170, 34, 144, 165, 68, 111, 16, 255, 30, 96, 85, 240, 85, 165, 84, 240, 101, 130, 0, 108, 85, 34, 144, 21, 66, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 85, 160, 192, 128, 160, 64, 160, 224, 160, 160, 224, 192, 64, 64, 224, 224, 32, 192, 224, 224, 96, 32, 224, 160, 224, 32, 32, 224, 192, 32, 192, 224, 128, 224, 224, 224, 32, 32, 32, 224, 224, 160, 224, 224, 224, 32, 224, 64, 160, 224, 160, 192, 224, 160, 224, 224, 128, 128, 224, 192, 160, 160, 192, 224, 192, 128, 224, 224, 128, 192, 128, 96, 128, 160, 96, 160, 224, 160, 160, 224, 64, 64, 224, 96, 32, 32, 192, 160, 192, 160, 160, 128, 128, 128, 224, 224, 224, 160, 160, 192, 160, 160, 160, 224, 160, 160, 224, 192, 160, 192, 128, 64, 160, 224, 96, 192, 160, 192, 160, 96, 192, 32, 192, 224, 64, 64, 64, 160, 160, 160, 96, 160, 160, 160, 64, 160, 160, 224, 224, 160, 64, 160, 160, 160, 160, 64, 64, 224, 96, 128, 224, 0, 0, 0, 0, 0, 224, 0, 0, 0, 0, 0, 64, 72, 44, 104, 104, 140, 0, 52, 44, 112, 112, 140, 0, 100, 120, 72, 60, 112, 0, 10, 174, 162, 66, 16, 48, 16, 184, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
            Stack: []
            IRegister: 0x1365
            Registry Memory: [85, 16, 85, 60, 112, 0, 10, 174, 162, 66, 39, 27, 85, 14, 56, 0]
            Current Opcode pointer: 0x0544
            Opcode: Some("1542")

```