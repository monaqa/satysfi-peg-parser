#[cfg(test)]
mod tests {

    use crate::satysfi::Rule;
    use super::super::common::{assert_success, assert_fail};

    #[test]
    fn pattern() {
        assert_success(Rule::pattern, "[hoge; fuga]");
        assert_success(Rule::pattern, "(hoge)");
        assert_success(Rule::pattern, "_");
        assert_success(Rule::pattern, "hoge");
        assert_success(Rule::pattern, "42");
    }

    #[test]
    fn pat_list() {
        assert_success(Rule::pat_list, "[]");
        assert_success(Rule::pat_list, "[1]");
        assert_success(Rule::pat_list, "[1;]");
        assert_success(Rule::pat_list, "[1; 2]");
        assert_success(Rule::pat_list, "[1; 2;]");
        assert_success(Rule::pat_list, "[hoge; _; 3]");
    }

    #[test]
    fn pat_tuple() {
        assert_success(Rule::pat_tuple, "(x, y)");
        assert_success(Rule::pat_tuple, "(_, y)");
        assert_success(Rule::pat_tuple, "(_, y, 1)");
        assert_fail(Rule::pat_tuple, "(x, )");
    }

    #[test]
    fn pat_variant() {
        assert_success(Rule::pat_variant, "Variant");
        assert_success(Rule::pat_variant, "Variant x");
        assert_success(Rule::pat_variant, "Variant(x)");
        assert_success(Rule::pat_variant, "Variant (x)");
        assert_success(Rule::pat_variant, "Variant(x, y)");
        assert_fail(Rule::pat_variant, "Variant 1 2");
    }

    #[test]
    fn match_ptn() {
        assert_success(Rule::match_ptn, "[a1; a2] as a");
        assert_success(Rule::match_ptn, "x :: rest");
        assert_success(Rule::match_ptn, "x :: [x2; x3]");
        assert_fail(Rule::match_ptn, "");
    }

}
