#[cfg(test)]
mod tests {

    use crate::Rule;
    use super::super::common::{assert_success, assert_fail};

    #[test]
    fn math_symbol() {
        assert_success(Rule::math_symbol, "+");
        assert_success(Rule::math_symbol, ":=");
        assert_fail(Rule::math_symbol, "_");
        assert_fail(Rule::math_symbol, "^");
    }

    #[test]
    fn math_cmd_name() {
        assert_success(Rule::math_cmd_name, r"\alpha");
        assert_success(Rule::math_cmd_name, r"\Math.alpha");
        assert_fail(Rule::math_cmd_name, r"\Alpha");
    }

    #[test]
    fn math_cmd_expr_arg() {
        assert_success(Rule::math_cmd_expr_arg, "{a}");
        assert_success(Rule::math_cmd_expr_arg, r"{\alpha}");
        assert_success(Rule::math_cmd_expr_arg, r"!{\alpha;}");
        assert_success(Rule::math_cmd_expr_arg, "!<+par;>");
        assert_success(Rule::math_cmd_expr_arg, "!(1pt + 2pt)");
        assert_fail(Rule::math_cmd_expr_arg, r"{\alpha;}");
    }

    #[test]
    fn math_cmd() {
        assert_success(Rule::math_cmd, r"\alpha");
        assert_success(Rule::math_cmd, r"\alpha");
        assert_success(Rule::math_cmd, r"\sqrt{a}");
        assert_success(Rule::math_cmd, r"\sqrt{\alpha}");
        assert_success(Rule::math_cmd, r"\frac{f}{t}");
        assert_success(Rule::math_cmd, r"\alpha!(t)");
        assert_fail(Rule::math_cmd, r"\alpha;");
    }

    #[test]
    fn math_unary() {
        assert_success(Rule::math_unary, "a");
        assert_success(Rule::math_unary, "A");
        assert_success(Rule::math_unary, "0");
        assert_success(Rule::math_unary, r"\{");
        assert_success(Rule::math_unary, r"+");
        assert_success(Rule::math_unary, r"\sqrt{2}");
        assert_fail(Rule::math_unary, "ab");
        assert_fail(Rule::math_unary, r"\alpha\beta");
    }

    #[test]
    fn math_group() {
        assert_success(Rule::math_group, "a");
        assert_success(Rule::math_group, "{a}");
        assert_success(Rule::math_group, "{ab}");
        assert_fail(Rule::math_group, "ab");
    }

    #[test]
    fn math_token() {
        assert_success(Rule::math_token, "a");
        assert_success(Rule::math_token, "a^b");
        assert_success(Rule::math_token, "a_b");
        assert_success(Rule::math_token, "a^b_c");
        assert_success(Rule::math_token, "a^b_c");
        assert_success(Rule::math_token, r"a^{b\alpha}");
        assert_fail(Rule::math_token, "a^bc");
    }

    #[test]
    fn math_single() {
        assert_success(Rule::math_single, r"a");
        assert_success(Rule::math_single, r"\alpha");
        assert_success(Rule::math_single, r"\alpha\beta");
        assert_success(Rule::math_single, r"\alpha \beta");
        assert_success(Rule::math_single, r"x = \frac{-b \pm \sqrt{b^2 - 4ac}}{2a}");
        assert_fail(Rule::math_single, r"\alpha;");
    }

    #[test]
    fn math_mode() {
        assert_success(Rule::math_mode, "abc");
        assert_success(Rule::math_mode, "|abc|");
        assert_success(Rule::math_mode, "||");
        assert_success(Rule::math_mode, "|abc|def|");
    }

}
