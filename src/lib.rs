#[cfg(test)]
mod tests;
pub mod cpi;
pub mod parser;


#[allow(unused_imports)]
pub mod prelude {

    use crate::cpi::container::CpiCommandType as ContainerCommand;
    use crate::cpi::vm::CpiCommandType as VMCommandType;
}