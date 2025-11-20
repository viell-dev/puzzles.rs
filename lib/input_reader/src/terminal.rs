#[expect(clippy::print_stdout, reason = "intentional user-facing output")]
pub(crate) fn print_help(identifier: &str) {
    println!(
        "\
USAGE: {identifier} [OPTIONS] [DATA...]

OPTIONS:
    -h, --help              Print this help message
    -i, --input <METHOD>    Set input method (no value defaults to file)
                            Methods: file, args, stdin
    -s, --save              Save input to file for future runs
    -f, --force             Force operations without prompts

ARGS:
    [DATA...]               Input data (when using args method)

NOTES:
    - Short flags can be grouped: -hsf, -sfi stdin
    - Use -- to treat remaining arguments as data
    - Unknown flags are treated as data"
    );
}

#[expect(clippy::print_stdout, reason = "intentional user-facing output")]
pub(crate) fn print_request_for_input() {
    println!(
        "\
Please provide the input, ending with two blank lines:"
    );
}

#[expect(clippy::print_stdout, reason = "intentional user-facing output")]
pub(crate) fn print_no_input() {
    println!(
        "\
No input data found. Exiting."
    );
}

/// Prompts the user for confirmation to overwrite an existing file.
///
/// Returns `true` if the user confirms (responds with 'y' or 'Y'),
/// `false` otherwise.
#[expect(clippy::print_stdout, reason = "intentional user-facing output")]
pub(crate) fn prompt_overwrite_confirmation() -> bool {
    use std::io::{self, Write};

    println!(
        "\
Input file already exists. Overwrite? (y/N): "
    );
    io::stdout().flush().ok();

    let mut response = String::new();
    if io::stdin().read_line(&mut response).is_ok() {
        let response = response.trim().to_lowercase();
        response == "y" || response == "yes"
    } else {
        false
    }
}
