use lexer::cursor::Cursor;

pub struct Lexer<'src> {
    src: &'src str,
    cursor: Cursor<'src>,
}
