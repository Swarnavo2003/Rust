use chrono::{Utc,Local};
fn main() {
  // Ge the current data and time in UTC
  let now = Utc::now();
  println!("Current data and time in UTC: {}", now);

  // Format the data and time
  let formatted = now.format("%Y-%m-%d %H:%M:%S");
  println!("Formatted date and time: {}", formatted);

  // Get Local time
  let local = Local::now();
  println!("Current data and time in local: {}", local);
}
