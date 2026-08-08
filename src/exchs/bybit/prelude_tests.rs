#[cfg(test)]
pub mod prelude {
    pub use crate::exchs::bybit::prelude::*;
    pub use crate::prelude_tests::prelude::*;

    pub static CL: LazyLock<Client> = LazyLock::new(|| new_rest_client(&S));
}
