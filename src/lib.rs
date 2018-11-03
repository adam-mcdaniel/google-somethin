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

mod manager;

use self::manager::*;

/// Options passed in with the google method
pub struct Options {
    /// If set to true: logs result to console, defaults to false.
    pub log_to_console: bool,
    /// Limits the results you retrieved, defaults to 10.
    pub limit: i32,
}

/// Google's the query you've passed in, which returns a `ValueConstrucrtor` which contains the `link` and `title` of the result.
/// 
/// # Examples
/// 
/// ```rust
/// let results = google_somethin::google("minecraft", Some(Options { limit: 1, log_to_console: false }));
/// println!("Title : {}\nLink : {}", results[0].title, results[0].link);
/// ```
pub fn google(query: &str, options: Option<Options>) -> Vec<ValueConstructor> {
    match options {
        Some(opts) => GoogleClient::new().return_sections(query, opts.limit, opts.log_to_console),
        None => GoogleClient::new().return_sections(query, 10, false),
    }
}
