// Reexportacion
mod sound {
  pub mod instrument {
    pub fn clarinet() {
      println!("call clarinet");
    }
  }
}
mod performance_group {
  // Combinar pub y use. Esta técnica se denomina reexportación porque se está
  // introduciendo un elemento en el ámbito de aplicación, pero también se está poniendo a disposición
  // de otros para que lo introduzcan en su ámbito de aplicación

  pub use crate::sound::instrument;
  pub fn clarinet_trio() {
    instrument::clarinet();
    println!("call clarinet_trio");
  }
}
fn main() {
  performance_group::clarinet_trio();
  performance_group::instrument::clarinet();
}