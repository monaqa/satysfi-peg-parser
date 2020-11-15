#[cfg(test)]
mod tests {

    use crate::parser::Rule;
    use super::super::common::{assert_success, assert_fail};

    #[test]
    fn cmd_expr_arg() {
        assert_success(Rule::cmd_expr_arg, "()");
        assert_success(Rule::cmd_expr_arg, "( )");
        assert_success(Rule::cmd_expr_arg, "(hoge)");
        assert_success(Rule::cmd_expr_arg, "('< +p(hoge); >)");
        assert_success(Rule::cmd_expr_arg, "(set-font-size 12pt)");
        assert_success(Rule::cmd_expr_arg, "[]");
        assert_success(Rule::cmd_expr_arg, "[1pt; 2pt]");
        assert_success(Rule::cmd_expr_arg, "(| height = 1pt; width = 2pt; |)");
        assert_fail(Rule::cmd_expr_arg, "{}");
        assert_fail(Rule::cmd_expr_arg, "'<>");
    }

    #[test]
    fn cmd_expr_option() {
        assert_success(Rule::cmd_expr_option, "?:()");
        assert_success(Rule::cmd_expr_option, "?*");
        assert_success(Rule::cmd_expr_option, "?:[]");
        assert_success(Rule::cmd_expr_option, "?:[1; 2]");
        assert_success(Rule::cmd_expr_option, "?:(| aaa = 1; bbb = 2 |)");
        assert_fail(Rule::cmd_expr_option, "?:*");
        assert_fail(Rule::cmd_expr_option, "?");
    }

    #[test]
    fn cmd_text_arg() {
        assert_success(Rule::cmd_text_arg, "<>");
        assert_success(Rule::cmd_text_arg, "{}");
        assert_success(Rule::cmd_text_arg, "<+par;>");
        assert_success(Rule::cmd_text_arg, "{hoge}");
        assert_success(Rule::cmd_text_arg, "{| aaa |}");
        assert_success(Rule::cmd_text_arg, "{* aaa}");
        assert_fail(Rule::cmd_text_arg, "(aaa)");
    }

    #[test]
    fn horizontal_special_char() {
        assert_success(Rule::horizontal_special_char, "@");
        assert_success(Rule::horizontal_special_char, "`");
        assert_success(Rule::horizontal_special_char, r"\");
        assert_success(Rule::horizontal_special_char, "{");
        assert_success(Rule::horizontal_special_char, "}");
        assert_success(Rule::horizontal_special_char, "|");
        assert_success(Rule::horizontal_special_char, "*");
        assert_success(Rule::horizontal_special_char, "$");
        assert_success(Rule::horizontal_special_char, "#");
        assert_success(Rule::horizontal_special_char, ";");
    }

    #[test]
    fn inline_cmd_name() {
        assert_success(Rule::inline_cmd_name, r"\t");
        assert_success(Rule::inline_cmd_name, r"\textbf");
        assert_success(Rule::inline_cmd_name, r"\Mod.textbf");
        assert_success(Rule::inline_cmd_name, r"\let");
        assert_success(Rule::inline_cmd_name, r"\in");
        assert_success(Rule::inline_cmd_name, r"\t-1");
        assert_fail(Rule::inline_cmd_name, r"\Textbf");
        assert_fail(Rule::inline_cmd_name, r"\1");
        assert_fail(Rule::inline_cmd_name, r"\-textbf");
        assert_fail(Rule::inline_cmd_name, r"\mod.textbf");
    }

    #[test]
    fn inline_cmd() {
        assert_success(Rule::inline_cmd, r"\textbf;");
        assert_success(Rule::inline_cmd, r"\textbf(hoge);");
        assert_success(Rule::inline_cmd, r"\textbf(hoge)(fuga);");
        assert_success(Rule::inline_cmd, r"\textbf?*;");
        assert_success(Rule::inline_cmd, r"\textbf{}");
        assert_success(Rule::inline_cmd, r"\hoge<>");
        assert_success(Rule::inline_cmd, r"\hoge<>{}");
        assert_success(Rule::inline_cmd, r"\ctx(set-font-size 12pt){fuga}");
        assert_success(Rule::inline_cmd, r"\easytable?:[t; b][l; lw 90pt]{| a | b |}");
        assert_success(Rule::inline_cmd, r"\hoge(| x = 1; y = 2; |);");

        assert_fail(Rule::inline_cmd, r"\textbf");
        assert_fail(Rule::inline_cmd, r"\textbf(hoge)");
        assert_fail(Rule::inline_cmd, r"\hoge{};");
        assert_fail(Rule::inline_cmd, r"\hoge{}();");
    }

    #[test]
    fn horizontal_text_embedding() {
        assert_success(Rule::horizontal_text_embedding, "#text;");
        assert_success(Rule::horizontal_text_embedding, "#if;");
        assert_success(Rule::horizontal_text_embedding, "#Mod.text;");
        assert_fail(Rule::horizontal_text_embedding, "#text");
        assert_fail(Rule::horizontal_text_embedding, "#1;");
        assert_fail(Rule::horizontal_text_embedding, "#mod.text;");
    }

    #[test]
    fn horizontal_token() {
        assert_success(Rule::horizontal_token, "`a`");
        assert_success(Rule::horizontal_token, r"\textbf;");
        assert_success(Rule::horizontal_token, r"\textbf();");
        assert_success(Rule::horizontal_token, r"\textbf<>");
        assert_success(Rule::horizontal_token, r"\ctx(set-font-size 12pt){fuga}");
        assert_success(Rule::horizontal_token, r"#text;");
        assert_success(Rule::horizontal_token, r"\$");
        assert_success(Rule::horizontal_token, "aaa");
        assert_success(Rule::horizontal_token, "aaa: bbb");
        assert_success(Rule::horizontal_token, "あい うえお");
        assert_success(Rule::horizontal_token, "🤔🤔🤔");
        assert_fail(Rule::horizontal_token, "a$");
        assert_fail(Rule::horizontal_token, "a{}");
        assert_fail(Rule::horizontal_token, "a;");
    }

    #[test]
    fn horizontal_single() {
        assert_success(Rule::horizontal_single, "");
        assert_success(Rule::horizontal_single, "hoge");
        assert_success(Rule::horizontal_single, r"hoge\textbf();fuga");
        assert_success(Rule::horizontal_single, r"hoge`code`fuga");
        assert_success(Rule::horizontal_single, r"hoge `code` fuga");
        assert_success(Rule::horizontal_single, "hoge % comment\n fuga");
        assert_success(Rule::horizontal_single, r"hoge \ctx(set-font-size 12pt){fuga} piyo");
        assert_fail(Rule::horizontal_single, "hoge{code}fuga");
        assert_fail(Rule::horizontal_single, r"hoge\textbf fuga");
    }

    #[test]
    fn horizontal_list() {
        assert_success(Rule::horizontal_list, "|hoge|");
        assert_success(Rule::horizontal_list, "|hoge|fuga|");
        assert_success(Rule::horizontal_list, "| hoge | fuga |");
        assert_fail(Rule::horizontal_list, "|hoge");
        assert_fail(Rule::horizontal_list, "hoge|");
        assert_fail(Rule::horizontal_list, "hoge|fuga");
    }

    #[test]
    fn horizontal_bullet_list() {
        assert_success(Rule::horizontal_bullet_list, "* hoge");
        assert_success(Rule::horizontal_bullet_list, "* hoge * fuga");
        assert_success(Rule::horizontal_bullet_list, "* hoge \n* fuga");
        assert_success(Rule::horizontal_bullet_list, "* hoge ** fuga");
        assert_success(Rule::horizontal_bullet_list, "* hoge ** fuga * piyo");
        assert_success(Rule::horizontal_bullet_list, "** hoge ** fuga * piyo");  // 構文解析では弾かない
        assert_fail(Rule::horizontal_bullet_list, "hoge * fuga");
    }

    #[test]
    fn horizontal_mode() {
        assert_success(Rule::horizontal_mode, "hoge");
        assert_success(Rule::horizontal_mode, "|hoge|");
        assert_success(Rule::horizontal_mode, "*hoge");
    }

}
