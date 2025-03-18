mod calculate;

pub use calculate::Program;

pub const SOURCE_FILE_EXTENSION: &'static str = "calc";

pub trait Compile {
    type Ret;

    fn compile(&self) -> Self::Ret;
}
