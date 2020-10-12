This is port of [echoprint-codegen](https://github.com/spotify/echoprint-codegen) in Rust.

This is not exact port, a lot of changes were made in the process:
- Code is thoroughly refactored for readability purposes
- A lot of unhandled edge cases of the original version are fixed (some input data could crash the codegen process)
- All external dependencies have been removed, but you need to provide raw 16bit pcm data (sampling rate 11025, LittleEndian) - so it's best to use this in pair with some audio decoder (tested with ffmpeg)
