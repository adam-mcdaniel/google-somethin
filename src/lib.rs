//! # Google Somethin
//!
//! A simple library that grabs Google search results...
//!
//! ## How to use
//!
//! ```rust
//! extern crate google_somethin;
//!
//! fn main() {
//!     use google_somethin::google;
//!     
//!     let results = google("roblox", None);
//!     println!("The first title of the roblox results! {}", results[0].title);
//! }
//! ```
extern crate reqwest;
extern crate scraper;
extern crate select;

mod managers;

use self::managers::*;

/// Options passed in with the google method
pub struct Options {
    /// If set to true: logs result to console, defaults to false.
    pub log_to_console: bool,
    /// Limits the results you retrieved, defaults to 10.
    pub limit: u32,
}

/// Google's the query you've passed in, which returns a `Section` which contains the `link` and `title` of the result.
///
/// # Examples
///
/// ```rust
/// use google_somethin::{google, Options};
/// let results = google("minecraft", Some(Options { limit: 1, log_to_console: false }));
/// println!("Title : {}\nLink : {}", results[0].title, results[0].link);
/// ```
pub fn google(query: &str, options: Option<Options>) -> Vec<Section> {
    match options {
        Some(opts) => return_results(query, opts.log_to_console, opts.limit),
        None => return_results(query, false, 10),
    }
}

#[cfg(test)]
mod google_somethin_tests {
    use super::*;
    #[test]
    fn google_title() {
        let results = google("minecraft", None);
        assert_eq!("Minecraft: Official site", results[0].title);
    }

    #[test]
    fn google_link() {
        let results = google("minecraft", None);
        assert_eq!("https://minecraft.net/", results[0].link);
    }
}
