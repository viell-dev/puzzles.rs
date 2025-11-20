# Plan:

1.  Take an identifier string when called.
    e.g. `read_input("aoc_2025_day01")` or some such.
2.  Check arguments for known flags.
3.  Read or demand input.

Inputs will be stored in `./input/<identifier>.txt` files.

When reading with no flags, try to find the file first, then check from
input provided as args, then finally ask for input to be entered (ending with
double blank line - three \n's). If input was provided; ask if it should be
saved only if a file doesn't already exist.

If the `--input` flag is provided, only try to read existing input files.
`--input file` would be the same thing with an explicit value. Other values
are `--input args` and `--input stdin` where, if set, only that input method
is attempted when reading.

If the `--save` flag is provided, the input should be saved into an input
file, assuming it wasn't read from there to begin with, in which case this
flag does nothing. A message that there was nothing to save should still
be shown.

When `--save` and `--input stdin` or `input args` are both given, or
args/stdin was resolved as the input method due to a file not existing; any
existing input file should be prompted for overwriting. Args should be saved
as one per line. Empty lines in input-files only matter in the middle, so the
content should always be trimmed.

All input should be valid UTF-8 strings or UTF-8 encoded text/plain files.

When built, the binary using this library should look for a "input.txt" file
next to the binary. When `cargo run` the library should resolve
`./input/<identifier>.txt` relative to the workspace (git repo) root.

It may thus be possible to filter out the identifier from the binaries and
have it as DEBUG only, or some such...
