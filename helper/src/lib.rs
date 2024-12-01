// How to use:
// in [dependencies] of Cargo.toml of the day** crate:
// helper ={ path = "../../helper"}

pub mod input; // use helper::input:
pub mod matrix; // use helper::matrix;
pub mod numerical; // use helper::numerical;

#[cfg(test)]
mod tests {}
