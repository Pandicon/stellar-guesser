#[cfg(target_os = "android")]
mod android;
#[cfg(target_os = "android")]
pub use android::*;
#[cfg(any(target_os = "windows", target_os = "linux", target_os = "macos"))]
mod desktop;
#[cfg(any(target_os = "windows", target_os = "linux", target_os = "macos"))]
pub use desktop::*;

fn generate_authors() -> String {
    let mut authors_split = crate::AUTHORS.split(':').collect::<Vec<&str>>();
    let authors = if authors_split.len() > 2 {
        let last = authors_split.pop().unwrap();
        format!("{}, and {}", authors_split.join(", "), last)
    } else {
        authors_split.join(" and ")
    };
    authors
}
