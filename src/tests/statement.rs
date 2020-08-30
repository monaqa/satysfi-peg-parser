#[cfg(test)]
mod tests {

    use crate::Rule;
    use super::super::common::{assert_success, assert_fail};

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
    fn let_block_stmt() {
        assert_success(Rule::let_block_stmt, "let-block ctx +a = fuga");
        assert_success(Rule::let_block_stmt, "let-block ctx +p x = block-nil");
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
