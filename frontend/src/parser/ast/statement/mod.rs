mod dml;
mod common;

pub use dml::DML;
pub use common::Common;

pub enum Statement {
    DML(DML),
    Common(Common),
}