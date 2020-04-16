pub mod difficulty;
pub(crate) mod ext;
pub mod info;

#[cfg(feature = "rw")]
pub(crate) mod rw;

pub use crate::{difficulty::Difficulty, info::Info};

#[cfg(feature = "rw")]
pub use crate::rw::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
