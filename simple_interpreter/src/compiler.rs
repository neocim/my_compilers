mod interpreter;

pub use interpreter::Interpreter;

pub trait Compiler {
    type Ret;

    fn compile(&self) -> Self::Ret;
}
