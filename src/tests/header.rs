#[cfg(test)]
mod tests {

    use crate::Rule;
    use super::super::common::{assert_success, assert_fail};

    #[test]
    fn header_stage() {
        assert_success(Rule::header_stage, "@stage: 0\n");
        assert_success(Rule::header_stage, "@stage: 1\n");
        assert_success(Rule::header_stage, "@stage: persistent\n");
        assert_success(Rule::header_stage, "@stage:0\n");
        assert_success(Rule::header_stage, "@stage:   0\n");
        assert_fail(Rule::header_stage, "@stage: 2\n");
        assert_fail(Rule::header_stage, "@stage : 2\n");
    }

    #[test]
    fn header() {
        assert_success(Rule::header, "@require: hoge\n");
        assert_success(Rule::header, "@require:    hoge\n");
        assert_success(Rule::header, "@import: hoge/fuga\n");
        assert_success(Rule::header, "@import: hoge fuga\n");
        assert_fail(Rule::header, "@require : hoge\n");
        assert_fail(Rule::header, "@require : hoge\nfuga");
    }

}
