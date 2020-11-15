#[cfg(test)]
mod tests {

    use crate::satysfi::Rule;
    use super::super::common::{assert_success, assert_fail};

    #[test]
    fn block_text() {
        assert_success(Rule::block_text, "'<>");
        assert_success(Rule::block_text, "'<+par;>");
        assert_success(Rule::block_text, "'< +p{aaa} +par; >");
        assert_fail(Rule::block_text, "'< +p(); +par >");
    }

    #[test]
    fn horizontal_text() {
        assert_success(Rule::horizontal_text, r"{hoge \textbf{fuga} piyo}");
        assert_success(Rule::horizontal_text, r"{hoge \ctx(set-font-size 12pt){fuga} piyo}");
    }

    #[test]
    fn bin_operator() {
        assert_success(Rule::bin_operator, "+");
        assert_success(Rule::bin_operator, "^");
        assert_success(Rule::bin_operator, "+'");
        assert_success(Rule::bin_operator, "--");
        assert_success(Rule::bin_operator, "^!'");
        assert_success(Rule::bin_operator, "::");
        assert_fail(Rule::bin_operator, "`");
        assert_fail(Rule::bin_operator, "#");
        assert_fail(Rule::bin_operator, "!");
        assert_fail(Rule::bin_operator, "!+");
    }

    #[test]
    fn dyadic_expr() {
        assert_success(Rule::dyadic_expr, "1pt +' 2pt");
        assert_success(Rule::dyadic_expr, "`a` ^ `b`");
        assert_success(Rule::dyadic_expr, "`a` ^ `b` ^ `c`");
        assert_success(Rule::dyadic_expr, "a ^ b");
        assert_success(Rule::dyadic_expr, "mira ^ #`` `fuga` `` ^ kimyo");
    }

    #[test]
    fn var_ptn() {
        assert_success(Rule::var_ptn, "hoge");
        assert_success(Rule::var_ptn, "hoge-fuga");
        assert_success(Rule::var_ptn, "hoge1");
        assert_success(Rule::var_ptn, "hoge-");
        assert_success(Rule::var_ptn, "hoge-1");
        assert_fail(Rule::var_ptn, "Hoge");
        assert_fail(Rule::var_ptn, "HogeFuga");
        assert_fail(Rule::var_ptn, "hoge_fuga");
        assert_fail(Rule::var_ptn, "1hoge");
        assert_fail(Rule::var_ptn, "-hoge");
        assert_fail(Rule::var_ptn, "hoge fuga");
        assert_fail(Rule::var_ptn, "hoge - 1");
        assert_fail(Rule::var_ptn, "hoge 1");
    }

    #[test]
    fn var() {
        assert_success(Rule::var, "hoge");
        assert_fail(Rule::var, "Hoge");
        assert_fail(Rule::var, "let");
        assert_fail(Rule::var, "let-block");
        assert_success(Rule::var, "let1");
        assert_success(Rule::var, "let-block-cmd");
        assert_fail(Rule::var, "let hoge");
    }

    #[test]
    fn module_name() {
        assert_success(Rule::module_name, "Mod");
        assert_success(Rule::module_name, "Mod2");
        assert_success(Rule::module_name, "Mod-2");
        assert_success(Rule::module_name, "ModVariable");
        assert_success(Rule::module_name, "ModVariable-OK");
        assert_fail(Rule::module_name, "mod");
        assert_fail(Rule::module_name, "mOD");
        assert_fail(Rule::module_name, "Mod Name");
        assert_fail(Rule::module_name, "Mod name");
    }

    #[test]
    fn variant_name() {
        assert_success(Rule::variant_name, "Mod");
        assert_success(Rule::variant_name, "Mod2");
        assert_success(Rule::variant_name, "Mod-2");
        assert_success(Rule::variant_name, "ModVariable");
        assert_success(Rule::variant_name, "ModVariable-OK");
        assert_fail(Rule::variant_name, "mod");
        assert_fail(Rule::variant_name, "mOD");
        assert_fail(Rule::variant_name, "Mod Name");
        assert_fail(Rule::variant_name, "Mod name");
    }

    #[test]
    fn modvar() {
        assert_success(Rule::modvar, "Mod.t");
        assert_success(Rule::modvar, "Mod.hoge-1");
        assert_fail(Rule::modvar, "Mod");
        assert_fail(Rule::modvar, "var");
        assert_fail(Rule::modvar, "mod.var");
        assert_fail(Rule::modvar, "Mod.Var");
    }

    #[test]
    fn reserved_word() {
        assert_success(Rule::reserved_word, "let");
        assert_success(Rule::reserved_word, "let-block");
        assert_success(Rule::reserved_word, "in");
        assert_success(Rule::reserved_word, "true");
        assert_success(Rule::reserved_word, "false");

        assert_fail(Rule::reserved_word, "letter");
        assert_fail(Rule::reserved_word, "let-block1");
        assert_fail(Rule::reserved_word, "inline-graphics");
        assert_fail(Rule::reserved_word, "sin");
    }

    #[test]
    fn tuple() {
        assert_success(Rule::tuple, "(1pt, 2pt)");
        assert_success(Rule::tuple, "(1pt, let x = 1 in x + 2)");
        assert_fail(Rule::tuple, "(1pt)");
        assert_fail(Rule::tuple, "()");
    }

    #[test]
    fn list() {
        assert_success(Rule::list, "[]");
        assert_success(Rule::list, "[ ]");
        assert_success(Rule::list, "[1]");
        assert_success(Rule::list, "[ 1 ]");
        assert_success(Rule::list, "[1;]");
        assert_success(Rule::list, "[1; 2]");
        assert_success(Rule::list, "[1; 2;]");
        assert_success(Rule::list, "[1; 2; 3]");
        assert_success(Rule::list, "[hoge fuga]");
        assert_success(Rule::list, "[hoge fuga;]");
        assert_success(Rule::list, "[hoge; piyo]");
        assert_success(Rule::list, "[hoge; piyo;]");
        assert_fail(Rule::list, "[;]");
    }

    #[test]
    fn record_unit() {
        assert_success(Rule::record_unit, "hoge = 1pt");
        assert_success(Rule::record_unit, "hoge = let x = 1 in x + 2");
        assert_success(Rule::record_unit, "hoge = 2pt + 3cm");
        assert_fail(Rule::record_unit, "hoge");
        assert_fail(Rule::record_unit, "hoge : 1pt");
    }

    #[test]
    fn record_inner() {
        assert_success(Rule::record_inner, "hoge = 1pt; fuga = 2.0");
        assert_success(Rule::record_inner, "hoge = 1pt; fuga = 2.0;");
        assert_fail(Rule::record_inner, "hoge; fuga = 2.0");
    }

    #[test]
    fn record() {
        assert_success(Rule::record, "(||)");
        assert_success(Rule::record, "(| |)");
        assert_success(Rule::record, "(| a = 1 |)");
        assert_success(Rule::record, "(| a = 1; b = 2pt |)");
        assert_success(Rule::record, "(| a = 1; b = 2pt; |)");
        assert_success(Rule::record, "(| a = 1; b = 2pt; c = 3pt |)");
        assert_success(Rule::record, "(|a=1;b=2pt;c=3pt|)");
        assert_success(Rule::record, "(| rec with a = 1 |)");
        assert_success(Rule::record, "(| rec with a = 1; b = 2pt; c = 3pt |)");
    }

    #[test]
    fn application() {
        assert_success(Rule::application, "set-font-size 12pt");
        assert_success(Rule::application, "read-inline ctx it");
        assert_success(Rule::application, "math-char MathOrd `α`");
        assert_success(Rule::application, "hoge ?:fuga");
        assert_success(Rule::application, "hoge ?*");
        assert_success(Rule::application, "hoge ?* ?:fuga");
        assert_success(Rule::application, r"command \code");
        assert_fail(Rule::application, "hoge");
        assert_fail(Rule::application, r"command \code;");
        assert_fail(Rule::application, r"command +par");
    }

    #[test]
    fn unary() {
        assert_success(Rule::unary, "'< +par; >");
        assert_success(Rule::unary, "{あああ}");
        assert_success(Rule::unary, "{* あああ}");
        assert_success(Rule::unary, r"{* あああ \textbf{いいい} \ctx(set-font-size 12pt){ううう}}");
        assert_success(Rule::unary, "(|hoge = 1pt|)");
        assert_success(Rule::unary, "[hoge;]");
        assert_success(Rule::unary, "(1pt, `hoge`)");
        assert_success(Rule::unary, "(let x = 3 in x + 4)");
        assert_success(Rule::unary, "(+)");
        assert_success(Rule::unary, "( + )");
        assert_success(Rule::unary, "inline-nil");
        assert_success(Rule::unary, "Mod.text");
        assert_success(Rule::unary, "Mod.(let x = 3 in x + 4)");
        assert_success(Rule::unary, "`constant`");
        assert_fail(Rule::unary, "");
        assert_fail(Rule::unary, "hoge fuga");
    }

    #[test]
    fn unary_operator_expr() {
        assert_success(Rule::unary_operator_expr, "-2.0pt");
        assert_success(Rule::unary_operator_expr, "-2");
        assert_success(Rule::unary_operator_expr, "-2.5");
        assert_success(Rule::unary_operator_expr, "not true");
        assert_success(Rule::unary_operator_expr, "not (x > y)");
        // assert_fail(Rule::unary_operator_expr, "notify");  // TODO: これを通す
    }

    #[test]
    fn variant_constructor() {
        assert_success(Rule::variant_constructor, "Variant");
        assert_success(Rule::variant_constructor, "Variant 1");
        assert_success(Rule::variant_constructor, "Variant(1)");
        assert_success(Rule::variant_constructor, "Variant(1, x)");
        assert_success(Rule::variant_constructor, "Variant (1, x)");
        assert_fail(Rule::variant_constructor, "Variant 1 2");
    }

    #[test]
    fn record_member() {
        assert_success(Rule::record_member, "hoge#fuga");
        assert_success(Rule::record_member, "hoge # fuga");
        assert_success(Rule::record_member, "(hoge fuga)#fuga");
        assert_fail(Rule::record_member, "hoge#let");
    }

    #[test]
    fn expr_with_mod() {
        assert_success(Rule::expr_with_mod, "Mod.(1 + 2)");
        assert_fail(Rule::expr_with_mod, "Mod . (1 + 2)");
        assert_fail(Rule::expr_with_mod, "Mod. (1 + 2)");
        assert_fail(Rule::expr_with_mod, "Mod .(1 + 2)");
    }

    #[test]
    fn match_arm() {
        assert_success(Rule::match_arm, "_ -> 0");
        assert_success(Rule::match_arm, "x -> let y = 2 + 1 in x + y");
        assert_success(Rule::match_arm, "x when x > 3 -> let y = 2 + 1 in x + y");
    }

    #[test]
    fn match_expr() {
        assert_success(Rule::match_expr, "match x with 1 -> 2");
        assert_success(Rule::match_expr, "match x with
            | 1 -> 2");
        assert_success(Rule::match_expr, "match x with
            | 1 -> 2
            | 2 -> 4
            | _ -> 0");
        assert_fail(Rule::match_expr, "");
    }

    #[test]
    fn ctrl_while() {
        assert_success(Rule::ctrl_while, "while x > 2 do loop");
    }

    #[test]
    fn ctrl_if() {
        assert_success(Rule::ctrl_if, "if x > 2 then x else 2");
        assert_success(Rule::ctrl_if, "if let y = 1 in x > y then x else 2");
    }

    #[test]
    fn expr() {
        assert_success(Rule::expr, "let hoge = 1pt in 1pt");
        assert_success(Rule::expr, "1pt +' 3cm");
        assert_success(Rule::expr, "read-inline ctx it");
        assert_success(Rule::expr, "inline-nil");

        assert_success(Rule::expr, "let hoge = 1pt in hoge +' fuga");
        assert_success(Rule::expr, "math-char MathOrd `α`");

        assert_fail(Rule::expr, "");
        assert_fail(Rule::expr, "let hoge = 1pt in");
        assert_fail(Rule::expr, "1pt 3pt");
        assert_fail(Rule::expr, "let hoge = 1pt in in");

        let long_txt = r"let hoge =
              let fuga = {aaa} in
              read-block ctx '<
                +p{
                  あああ\ctx(set-font-size (12pt +' 3pt)){いいい} #fuga;
                }
              >
            in
            hoge +++ block-nil";

        assert_success(Rule::expr, long_txt);
        // dbg!(SatysfiParser::parse(Rule::expr, long_txt).unwrap());
    }

}
