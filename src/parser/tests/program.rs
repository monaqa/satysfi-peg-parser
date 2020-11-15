#[cfg(test)]
mod tests {

    use crate::parser::Rule;
    use super::super::common::{assert_success, assert_fail};

    #[test]
    fn program() {
        assert_success(Rule::program, "let a = 1 in 1");
        assert_success(Rule::program, "1");
        assert_success(Rule::program, "let a = 1 let b = 2 in a + b");
        assert_success(Rule::program, "let a = 1 in let b = 2 in a + b");
        assert_success(Rule::program, "let a = let b = 2 in a + b in a");

        assert_fail(Rule::program, "let a = 1 in");
    }

}
