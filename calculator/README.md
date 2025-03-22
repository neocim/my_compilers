<div align="center">
    <h2><code>calculator</code></h2>
</div>

<h2>Quick start</h2>

1. Clone parent repository:
```
git clone https://github.com/neocim/my_compilers
```
2. Go to calculator:
```
cd my_compilers/calculator
```
3. Run examples:
> If you did not specify the file directly, our compiler will find all the files with the `calc` extension in specified directory and run them, displaying the result of the execution for each program.
```
cargo run -- -p ./examples/
```

<h2>Stage</h2>

**Finished**. There's everything here that I wanted to practice with. My next project is a simple language that will most likely be without a standard library. I think it will have a simpler frontend than in this compiler, and I also want it to be translated into llvm ir.

<h2>About</h2>
This is my first working compiler. 

<h4>What can it do</h4>
The only thing he can do is compile and calculate binary expressions with integer and floating numbers. The operations it supports are: 

- addition
- subtraction
- multiplication
- division
- modulo division.

It's all. Then why is there so much code for a simple calculator? I tried to write it normally and make it extensible, meaning if we wanted to add features and so on, it was pretty easy. Compiling is divided into lexical analysis, parsing, lowering the parse tree to a more convenient level for compilation (that is, in our case, the parsing of source strings into numbers occurs at this level, and not during parsing), and the calculation of the binary expression itself. Such an architecture is actually redundant for such a simple compiler, but it was all just my practice.

<h4>Details</h4>

- [Lexer](https://github.com/neocim/my_compilers/blob/master/calculator/src/compiler/src/lexer.rs) divides the input string into tokens. it doesn't return any errors (and there aren't any) to make it easier to use. 
- Different types of code representation are used at different stages of compilation. For example, a lexer returns a token stream, a parser returns an ast, and a [lower](https://github.com/neocim/my_compilers/blob/master/calculator/src/compiler/src/ast_lowering.rs) uses its lowered ast.
- Сompiler [uses](https://github.com/neocim/my_compilers/blob/master/calculator/src/compiler/src/parser.rs) the [top-down recursive descent parser](https://en.wikipedia.org/wiki/Top-down_parsing), which builds an ast from our token stream.
- To return and output errors to the user, a simplified [diagnostic system](https://github.com/neocim/my_compilers/blob/master/calculator/src/compiler/src/errors/diagnostic.rs) from [rustc](https://github.com/rust-lang/rust/tree/master) is used, which can be used for [error recovery](https://en.wikipedia.org/wiki/Burke%E2%80%93Fisher_error_repair) (im not sure if this link is specifically about error recovery, but it seems to fit by definition.).
