pub use self::types::*;
pub use self::functions::*;
pub use self::enums::*;

mod types;
mod functions;
mod enums;


#[derive(Show)]
pub struct AvahiResolveResult {
  pub name: String,
  pub host_name: String,
  pub address: String,
  pub port: u16
}
