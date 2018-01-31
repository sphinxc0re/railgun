#[cfg(debug_assertions)]
const _GRAMMAR: &'static str = include_str!("../../src/wast.pest");

#[derive(Parser)]
#[grammar = "wast.pest"]
pub struct WastParser;

#[cfg(test)]
mod tests {
    use super::*;
    use pest::Parser;

    #[test]
    fn simple_parser() {
        let input = r#"
            (module
                (func (export "unaligned") (result i32)
                    (local i32 f64 f64)
                    (set_local 0 (i32.const 10))
                      (block
                        (loop
                          (if
                            (i32.eq (get_local 0) (i32.const 0))
                            (then (br 2))
                          )
                          (set_local 2 (f64.convert_s/i32 (get_local 0)))
                          (f64.store align=1 (get_local 0) (get_local 2))
                          (set_local 1 (f64.load align=1 (get_local 0)))
                          (if
                            (f64.ne (get_local 2) (get_local 1))
                            (then (return (i32.const 0)))
                          )
                          (set_local 0 (i32.sub (get_local 0) (i32.const 1)))
                          (br 0)
                        )
                      )
                      (i32.const 1)
                )
            )
        "#;

        WastParser::parse(Rule::file, input.trim()).unwrap_or_else(|e| panic!("{}", e));
    }
}
