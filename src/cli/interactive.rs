use indicatif::{MultiProgress, ProgressBar, ProgressStyle};

/// Interactive UI manager using indicatif
pub struct InteractiveUi {
    multi: MultiProgress,
    main_pb: ProgressBar,
}

impl InteractiveUi {
    /// Create a new interactive UI
    pub fn new(total_steps: u64) -> Self {
        let multi = MultiProgress::new();

        let main_pb = multi.add(ProgressBar::new(total_steps));
        main_pb.set_style(ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta}) {msg}")
            .expect("Failed to set progress style")
            .progress_chars("#>-"));

        Self { multi, main_pb }
    }

    /// Add a child progress bar
    pub fn add_child_pb(&self, len: u64, message: &str) -> ProgressBar {
        let pb = self.multi.add(ProgressBar::new(len));
        pb.set_style(
            ProgressStyle::default_bar()
                .template("  {spinner:.yellow} {msg} [{bar:20.yellow/orange}] {pos}/{len}")
                .expect("Failed to set progress style")
                .progress_chars("=>-"),
        );
        pb.set_message(message.to_string());
        pb
    }

    /// Increment main progress
    pub fn inc_main(&self) {
        self.main_pb.inc(1);
    }

    /// Set main status message
    pub fn set_main_message(&self, msg: &str) {
        self.main_pb.set_message(msg.to_string());
    }

    /// Finish UI
    pub fn finish(&self) {
        self.main_pb.finish_with_message("Scan Completed");
    }
}
