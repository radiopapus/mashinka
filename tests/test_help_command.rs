mod common;

#[cfg(test)]
pub mod test_version_command {
    use std::str::from_utf8;
    use mashinka::command::{HELP_COMMAND_NAME};
    use crate::common::{BIN_NAME};

    #[test]
    fn test_run_help_command() {
        let output = test_bin::get_test_bin(BIN_NAME).arg(HELP_COMMAND_NAME).output().unwrap();

        assert!(&output.status.success());

        let as_string = output.stdout.to_ascii_lowercase();
        let stdout = from_utf8(&as_string).unwrap();
        assert!(&stdout.contains("usage"), "Check usage");
        assert!(&stdout.contains("example"), "Check Example");
        assert!(&stdout.contains("miscellaneous"), "Check Miscellaneous part");
    }
}

