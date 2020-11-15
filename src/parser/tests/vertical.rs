#[cfg(test)]
mod tests {

    use crate::parser::Rule;
    use super::super::common::{assert_success, assert_fail};

    #[test]
    fn block_cmd_name() {
        assert_success(Rule::block_cmd_name, "+p");
        assert_success(Rule::block_cmd_name, "+paragraph");
        assert_success(Rule::block_cmd_name, "+Mod.p");
        assert_success(Rule::block_cmd_name, "+let");
        assert_success(Rule::block_cmd_name, "+in");
        assert_success(Rule::block_cmd_name, "+p-1");
        assert_fail(Rule::block_cmd_name, "+1");
        assert_fail(Rule::block_cmd_name, "+-paragraph");
        assert_fail(Rule::block_cmd_name, "+mod.p");
    }

    #[test]
    fn block_cmd() {
        assert_success(Rule::block_cmd, "+par;");
        assert_success(Rule::block_cmd, "+p(hoge);");
        assert_success(Rule::block_cmd, "+p(hoge)(fuga);");
        assert_success(Rule::block_cmd, "+p?*;");
        assert_success(Rule::block_cmd, "+ctx<>");
        assert_success(Rule::block_cmd, "+p{あああ}");
        assert_success(Rule::block_cmd, "+ctx< +par; >");
        assert_success(Rule::block_cmd, "+ctx< +par(1 > 2); >");
        assert_success(Rule::block_cmd, "+ctx< +par; >{}");
        assert_success(Rule::block_cmd, "+ctx()< +par; >");
        assert_success(Rule::block_cmd, "+ctx?:()< +par; >");

        assert_fail(Rule::block_cmd, "+p");
        assert_fail(Rule::block_cmd, "+p(hoge)");
        assert_fail(Rule::block_cmd, "+ctx<>;");
        assert_fail(Rule::block_cmd, "+ctx<>< +par; >;");
        assert_fail(Rule::block_cmd, "+ctx();< +par; >");
        assert_fail(Rule::block_cmd, "+ctx?:< +par; >();");
    }

    #[test]
    fn vertical_mode() {
        assert_success(Rule::vertical_mode, "+par;");
        assert_success(Rule::vertical_mode, "+par; +p(hoge);");
        assert_success(Rule::vertical_mode, "#par; +p(hoge);");
        assert_fail(Rule::vertical_mode, "+par");
    }

    #[test]
    fn block_text_embedding() {
        assert_success(Rule::block_text_embedding, "#paren;");
        assert_success(Rule::block_text_embedding, "#if;");
        assert_success(Rule::block_text_embedding, "#Mod.paren;");
        assert_fail(Rule::block_text_embedding, "#paren");
        assert_fail(Rule::block_text_embedding, "#1;");
        assert_fail(Rule::block_text_embedding, "#mod.paren;");
    }

}
