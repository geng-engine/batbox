//! Retrieving program arguments
//!
//! Works on web using query string
#![warn(missing_docs)]

pub mod prelude {
    //! Items intended to be added to global namespace

    pub use crate as cli;
    pub use clap;
}

/// Returns a list of program arguments
///
/// On the web, program arguments are constructed from the query string:
/// `?flag&key=value1&key=value2` turns into `--flag --key=value1 --key=value2`.
/// Also, `args=something` just adds an arg to the list: `?args=test` turns into `test`.
pub fn get() -> Vec<String> {
    #[cfg(target_arch = "wasm32")]
    return {
        let mut args = vec!["program".to_owned()]; // `Program` itself is the first arg
        let url = url::Url::parse(&web_sys::window().unwrap().location().href().unwrap())
            .expect("Failed to parse window.location.href");
        for (key, value) in url.query_pairs() {
            let key: &str = &key;
            let value: &str = &value;
            if key == "args" {
                args.push(value.to_owned());
            } else if value.is_empty() {
                args.push("--".to_owned() + key);
            } else {
                args.push("--".to_owned() + key + "=" + value);
            }
        }
        log::trace!("href => args: {:?}", args);
        args
    };
    #[cfg(not(target_arch = "wasm32"))]
    return std::env::args().collect();
}

/// Parse program arguments
pub fn parse<T: clap::Parser>() -> T {
    match clap::Parser::try_parse_from(get()) {
        Ok(args) => args,
        Err(err) => {
            // On web - panic application so the geng panic handler catches it and the error is displayed
            #[cfg(target_arch = "wasm32")]
            panic!("Failed to parse program arguments: {}", err);
            // Native - use the regular clap behavior that exits the application
            #[cfg(not(target_arch = "wasm32"))]
            err.exit();
        }
    }
}
