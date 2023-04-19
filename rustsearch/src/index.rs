// use crate::helpers::Config;

pub mod index6;
pub mod index7;
pub mod index8_0;
pub mod index8_1;
pub mod index8_2;
pub mod index8_3;
pub mod index8_4;
pub mod index9_0;
pub mod index9_1;
pub mod index10_0;

pub mod gen_query;

pub struct Index<T, G> {
    pub database: T,
    extra_variables: Option<G>,
}
