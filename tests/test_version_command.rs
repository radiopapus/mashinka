mod common;

#[cfg(test)]
pub mod test_version_command {
    use std::env;
    use std::str::from_utf8;
    use mashinka::command::{VERSION_COMMAND_NAME};
    use mashinka::config::VERSION;
    use crate::common::{BIN_NAME};

    ///
    /// Сравниваем версию `CARGO_PKG_VERSION` c тем, что возвращает `mashinka version`
    #[test]
    fn test_run_version_command() {
        let output = test_bin::get_test_bin(BIN_NAME).arg(VERSION_COMMAND_NAME).output().unwrap();

        let v = env::var("CARGO_PKG_VERSION").unwrap();

        dbg!(VERSION, &v);

        assert!(&output.status.success());

        let as_string = output.stdout.to_ascii_lowercase();
        let stdout = from_utf8(&as_string).unwrap();
        assert!(&stdout.contains(v.as_str()), "Env CARGO_PKG_VERSION value should be equal to config::VERSION");
    }
}

