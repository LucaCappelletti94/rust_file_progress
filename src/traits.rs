pub trait FileProgress: ToString {
    /// Increase by one.
    fn tick(&mut self);

    /// Get the path to the file.
    fn get_path(&self) -> &str;

    /// Returns whether the process has completed.
    fn is_complete(&self) -> bool;

    /// Set length of the progress.
    fn set_len(&mut self, len: usize);

    /// Get the current count.
    fn get_count(&self) -> usize;

    /// Print progress to file.
    fn print(&self) -> std::io::Result<()> {
        if self.is_verbose() {
            std::fs::write(self.get_path(), self.to_string())?
        }
        Ok(())
    }

    /// Set verbosity.
    fn set_verbose(&mut self, verbose: bool);

    /// Return whether the verbose.
    fn is_verbose(&self) -> bool;

    /// Increase by one the progress and prints it to file.
    fn tick_and_print(&mut self) -> std::io::Result<()> {
        self.tick();
        self.print()
    }
}