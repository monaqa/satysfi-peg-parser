#[cfg(test)]
mod tests {

    use crate::satysfi::Rule;
    use super::super::common::{assert_success, assert_fail};

    #[test]
    fn arg() {
        assert_success(Rule::arg, "42");
        assert_success(Rule::arg, "()");
        assert_success(Rule::arg, "_");
        assert_success(Rule::arg, "hoge");
        assert_success(Rule::arg, "(x, y)");
        assert_success(Rule::arg, "?:hoge");
        assert_success(Rule::arg, "?: hoge");
        assert_fail(Rule::arg, "if");
    }

    #[test]
    fn stmt_argument() {
        assert_success(Rule::stmt_argument, "hoge");
        assert_success(Rule::stmt_argument, "| hoge");
        assert_success(Rule::stmt_argument, "hoge 42");
        assert_fail(Rule::stmt_argument, "| hoge | fuga");
    }

    #[test]
    fn let_mutable_stmt() {
        assert_success(Rule::let_mutable_stmt, "let-mutable x <- 1");
        assert_success(Rule::let_mutable_stmt, "let-mutable x <- embed-string `hoge`");
        assert_fail(Rule::let_mutable_stmt, "let-mutable x = 1");
        assert_fail(Rule::let_mutable_stmt, "let-mutable (x, y) <- (1, 2)");
    }

    #[test]
    fn let_math_stmt() {
        assert_success(Rule::let_math_stmt, r"let-math \alpha = math-char MathOrd `α`");
        assert_fail(Rule::let_math_stmt, r"let-math ctx \alpha = math-char MathOrd `α`");
    }

    #[test]
    fn let_inline_stmt() {
        assert_success(Rule::let_inline_stmt, r"let-inline \ctx={}");
        assert_success(Rule::let_inline_stmt, r"let-inline \ctx = {}");
        assert_success(Rule::let_inline_stmt, r"let-inline \ctx arg = {}");
        assert_success(Rule::let_inline_stmt, r"let-inline \ctx arg1 arg2 = {}");
        assert_success(Rule::let_inline_stmt, r"let-inline ctx \ctx = inline-fil");
        assert_success(Rule::let_inline_stmt, r"let-inline ctx \ctx arg = inline-fil");
        assert_success(Rule::let_inline_stmt, r"let-inline ctx \ctx arg1 arg2 = inline-fil");
        assert_success(Rule::let_inline_stmt, r"let-inline ctx \ctx ?:arg1 arg2 = inline-fil");
        assert_fail(Rule::let_inline_stmt, r"let-inline \ctx ?:arg1 arg2 = {}");
    }

    #[test]
    fn let_block_stmt() {
        assert_success(Rule::let_block_stmt, "let-block +p='<>");
        assert_success(Rule::let_block_stmt, "let-block +p = '<>");
        assert_success(Rule::let_block_stmt, "let-block +p arg = '<>");
        assert_success(Rule::let_block_stmt, "let-block +p arg1 arg2 = '<>");
        assert_success(Rule::let_block_stmt, "let-block ctx +p = block-nil");
        assert_success(Rule::let_block_stmt, "let-block ctx +p arg = block-nil");
        assert_success(Rule::let_block_stmt, "let-block ctx +p arg1 arg2 = block-nil");
        assert_success(Rule::let_block_stmt, "let-block ctx +p ?:arg1 arg2 = block-nil");
        assert_fail(Rule::let_block_stmt, "let-block +p ?:arg1 arg2 = '<>");
    }

    #[test]
    fn let_stmt() {
        assert_success(Rule::let_stmt, "let hoge = 1");
        assert_success(Rule::let_stmt, "let hoge = fuga");
        assert_fail(Rule::let_stmt, "let let = fuga");
        assert_fail(Rule::let_stmt, "let hoge = let");

        assert_success(Rule::let_stmt, "let hoge = let fuga = 2 in 1 + fuga");
        assert_success(Rule::let_stmt, "let hoge = let fuga = 2 in let piyo = 3 in fuga + piyo");
    }

    #[test]
    fn bind_stmt() {
        assert_success(Rule::bind_stmt, "let hoge = fuga in");
    }

    #[test]
    fn preamble() {
        assert_success(Rule::preamble, "let hoge = `a` let fuga = 2");
        assert_success(Rule::preamble, "let hoge = 1 let fuga = 2");
        assert_success(Rule::preamble, "let hoge = a let fuga = 2");
    }

}
