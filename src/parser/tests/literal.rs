#[cfg(test)]
mod tests {

    use crate::parser::Rule;
    use super::super::common::{assert_success, assert_fail};

    #[test]
    fn int() {
        assert_success(Rule::int_const, "123");
        assert_success(Rule::int_const, "0x2f1f");
        assert_success(Rule::int_const, "0x2F1F");
        assert_fail(Rule::int_const, ".123");
        assert_fail(Rule::int_const, "x123");
        assert_fail(Rule::int_const, "0b0110");
        assert_fail(Rule::int_const, "123 4");
        assert_fail(Rule::int_const, "123 m");
    }

    #[test]
    fn float() {
        assert_success(Rule::float_const, "123.56");
        assert_success(Rule::float_const, ".56");
        assert_success(Rule::float_const, "123.");
    }

    #[test]
    fn string() {
        assert_success(Rule::string_const, "`a`");
        assert_success(Rule::string_const, "`` a` ``");
        assert_success(Rule::string_const, "`` hoge`fuga ``");
        assert_success(Rule::string_const, "`` hogefuga ``");
        assert_success(Rule::string_const, "#`` hoge`fuga ``");
        assert_success(Rule::string_const, "#`` hoge`fuga ``#");
        assert_success(Rule::string_const, "#```` hoge```fuga ````#");
        assert_fail(Rule::string_const, "`` hogefuga `` ``");

        // string literal 中ではコメントが無視されない
        assert_fail(Rule::string_const, "`` hogefuga %``\n ``");
    }

    #[test]
    fn length() {
        assert_success(Rule::length_const, "0pt");
        assert_success(Rule::length_const, "0cm");
        assert_success(Rule::length_const, "0aa");
        assert_success(Rule::length_const, "12pt");
        assert_success(Rule::length_const, "12.3pt");
        assert_success(Rule::length_const, ".3pt");
        assert_success(Rule::length_const, ".3pt2");
    }

    #[test]
    fn bool() {
        assert_success(Rule::bool_const, "true");
        assert_success(Rule::bool_const, "false");
        assert_fail(Rule::bool_const, "True");
    }

    #[test]
    fn unit() {
        assert_success(Rule::unit_const, "()");
        assert_success(Rule::unit_const, "(  )");
        assert_success(Rule::unit_const, "(\n)");
    }

    #[test]
    fn literal() {
        assert_success(Rule::literal, "(  )");
        assert_success(Rule::literal, "true");
        assert_success(Rule::literal, "#` hoge`");
        assert_success(Rule::literal, "2.3pt");
        assert_success(Rule::literal, "2.3");
        assert_success(Rule::literal, "2");
    }

}
