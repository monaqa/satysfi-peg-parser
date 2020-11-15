#[cfg(test)]
mod tests {

    use crate::satysfi::Rule;
    use super::super::common::{assert_success, assert_fail};

    #[test]
    fn sig_type_stmt() {
        assert_success(Rule::sig_type_stmt, "type t");
        assert_success(Rule::sig_type_stmt, "type 'a t");
        assert_success(Rule::sig_type_stmt, "type 'a 'b t");
        assert_success(Rule::sig_type_stmt, "type 'a t constraint 'a :: (| idx: int |)");
        assert_success(Rule::sig_type_stmt, "type 'a t constraint 'a :: (| idx: int; pos: length * length |)");
        assert_success(Rule::sig_type_stmt, "type 'a t constraint 'a :: (| person: 'b; pos: length * length |) constraint 'b :: (| name: inline-text |)");
        assert_fail(Rule::sig_type_stmt, "type if");
    }

    #[test]
    fn sig_val_stmt() {
        assert_success(Rule::sig_val_stmt, "val hoge: int");
        assert_success(Rule::sig_val_stmt, "val hoge : int");
        assert_success(Rule::sig_val_stmt, "val (+): int -> int -> int");
        assert_success(Rule::sig_val_stmt, r"val \textbf : [inline-text] inline-cmd");
        assert_success(Rule::sig_val_stmt, "val +section : [inline-text; block-text] block-cmd");
        assert_success(Rule::sig_val_stmt, "val get-name: 'a -> string constraint 'a :: (| name: string |)");
        assert_success(Rule::sig_val_stmt, "val +description : ['a; itemize] block-cmd constraint 'a :: (| title: inline-text |)");
    }

    #[test]
    fn sig_direct_stmt() {
        assert_success(Rule::sig_direct_stmt, r"direct \textbf : [inline-text] inline-cmd");
        assert_success(Rule::sig_direct_stmt, "direct +section : [inline-text; block-text] block-cmd");
        assert_success(Rule::sig_direct_stmt, "direct +description : ['a; itemize] block-cmd constraint 'a :: (| title: inline-text |)");
        assert_fail(Rule::sig_direct_stmt, "direct hoge: int");
    }

    #[test]
    fn sig_inner() {
        assert_success(Rule::sig_inner, r"val hoge: int");
        assert_success(Rule::sig_inner, r"val \fuga: [] inline-cmd");
        assert_success(Rule::sig_inner, r"direct +piyo: [inline-text] block-cmd");
        assert_success(Rule::sig_inner, r"val hoge: int val \fuga: [] inline-cmd direct +piyo: [inline-text] block-cmd");
    }

    #[test]
    fn sig_stmt() {
        assert_success(Rule::sig_stmt, "sig end");
        assert_success(Rule::sig_stmt, r"sig val hoge: int val \fuga: [] inline-cmd direct +piyo: [inline-text] block-cmd end");
    }

    #[test]
    fn struct_stmt() {
        assert_success(Rule::struct_stmt, "struct end");
    }

    #[test]
    fn module_stmt() {
        assert_success(Rule::module_stmt, "module Color : sig
              val gray   : float -> color
              val rgb    : float -> float -> float -> color
              val black  : color
            end = struct
              let gray x = Gray(x)
              let rgb r g b = RGB(r, g, b)
              let black  = gray 0.
              let white  = gray 1.
            end");
    }

}
