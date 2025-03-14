mod calculate;

pub trait Compile {
    type Ret;

    fn compile(&self) -> Self::Ret;
}
