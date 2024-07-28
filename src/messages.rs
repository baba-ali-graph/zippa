use colored::*;
pub fn get_zipping_message(src: &str, dest: &str) -> String {
    let formmatted_src = String::from(src).bold().blue();
    let formatted_dest = String::from(dest).bold().blue();
    format!("Zipping {} to {}", formmatted_src, formatted_dest)
}
