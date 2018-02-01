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
        let input = include_str!("testfiles/test1.wast");

        WastParser::parse(Rule::file, input.trim()).unwrap_or_else(|e| panic!("{}", e));
    }

    #[test]
    fn fission_test() {
        let input = include_str!("testfiles/fission.wast");

        WastParser::parse(Rule::file, input.trim()).unwrap_or_else(|e| panic!("{}", e));
    }
}
