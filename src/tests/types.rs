#[cfg(test)]
mod tests {

    use crate::satysfi::Rule;
    use super::super::common::{assert_success, assert_fail};

    #[test]
    fn type_param() {
        assert_success(Rule::type_param, "'a");
        assert_success(Rule::type_param, "'var");
        assert_success(Rule::type_param, "'if");
        assert_fail(Rule::type_param, "' a");
    }

    #[test]
    fn type_name() {
        assert_success(Rule::type_name, "int");
        assert_success(Rule::type_name, "inline-text");
        assert_success(Rule::type_name, "type-name-1");
        assert_success(Rule::type_name, "Mod.t");
        assert_fail(Rule::type_name, "if");
    }

    #[test]
    fn type_application() {
        assert_success(Rule::type_application, "hoge list");
        assert_success(Rule::type_application, "hoge option list");
        assert_success(Rule::type_application, "'a t list");
        assert_success(Rule::type_application, "'a 'b t list");
        assert_success(Rule::type_application, "(Hoge.t?-> int -> bool) list");
    }

    #[test]
    fn type_list() {
        assert_success(Rule::type_list, "[]");
        assert_success(Rule::type_list, "[t]");
        assert_success(Rule::type_list, "[t;]");
        assert_success(Rule::type_list, "[t; Hoge.t ]");
        assert_success(Rule::type_list, "[t; Hoge.t; ]");
    }

    #[test]
    fn type_record() {
        assert_success(Rule::type_record, "(||)");
        assert_success(Rule::type_record, "(| |)");
        assert_success(Rule::type_record, "( | | )");
        assert_success(Rule::type_record, "(| hoge:t |)");
        assert_success(Rule::type_record, "(| hoge: t |)");
        assert_success(Rule::type_record, "(| hoge : t |)");
        assert_success(Rule::type_record, "(| hoge : t; |)");
    }

    #[test]
    fn constraint() {
        assert_success(Rule::constraint, "constraint 'a :: (||)");
        assert_success(Rule::constraint, "constraint 'a :: (| hoge : int |)");
        assert_success(Rule::constraint, "constraint 'a :: (| hoge : int; fuga : string |)");
        assert_fail(Rule::constraint, "constraint a :: (||)");
        assert_fail(Rule::constraint, "constraint a : (||)");
    }

    #[test]
    fn type_unary() {
        assert_success(Rule::type_unary, "[] inline-cmd");
        assert_success(Rule::type_unary, "[int;] inline-cmd");
        assert_success(Rule::type_unary, "[int; inline-text;] inline-cmd");
        assert_success(Rule::type_unary, "[] block-cmd");
        assert_success(Rule::type_unary, "[] math-cmd");
        assert_success(Rule::type_unary, "(int -> length -> inline-boxes)");
        assert_success(Rule::type_unary, "(| hoge:t |)");
        assert_success(Rule::type_unary, "'a");
        assert_success(Rule::type_unary, "Hoge.t option");
        assert_success(Rule::type_unary, "float");
    }

    #[test]
    fn type_prod() {
        assert_success(Rule::type_prod, "float * Hoge.t option");
        assert_success(Rule::type_prod, "'a list * (int -> inline-boxes)");
        assert_success(Rule::type_prod, "[] inline-cmd * (| hoge: t |)");
    }

    #[test]
    fn type_expr() {
        assert_success(Rule::type_expr, "hoge");
        assert_success(Rule::type_expr, "hoge * fuga");
        assert_success(Rule::type_expr, "int -> int");
        assert_success(Rule::type_expr, "int * int -> float * float");
        assert_success(Rule::type_expr, "'a option -> ('a option -> 'b list) -> (| idx: int; flag: bool |) -> 'b list");
    }

}
