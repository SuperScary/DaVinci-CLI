use DaVinci_CLI::run_editor;

struct CleanUp;

impl Drop for CleanUp {
    fn drop(&mut self) {
        // Cleanup is handled by the library
    }
}

fn main() -> crossterm::Result<()> {
    let _clean_up = CleanUp;
    run_editor()
}