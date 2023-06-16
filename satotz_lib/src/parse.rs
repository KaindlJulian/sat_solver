use nom::bytes::complete::{tag, take_until};
use nom::multi::{many0, separated_list0};
use nom::sequence::pair;
use nom::IResult;

fn parse_clause(input: &str) -> IResult<&str, Vec<i32>> {
    let (input, clause_input) = take_until("0\n")(input)?;
    let (_, literals) = separated_list0(tag(" "), nom::character::complete::i32)(clause_input)?;
    Ok((input, literals))
}

pub fn parse_dimacs_cnf(input: &str) -> IResult<&str, Vec<Vec<i32>>> {
    let (input, _) = many0(pair(tag("p cnf "), take_until("\n")))(input)?;
    let (input, _) = many0(tag("\n"))(input)?;
    let (input, clauses) = separated_list0(tag("0\n"), parse_clause)(input)?;
    Ok((input, clauses))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_empty_clause() {
        let input = "0\n";
        let result = parse_clause(input).expect("parse error").1;
        assert_eq!(result, vec![]);
    }

    #[test]
    fn test_parse_clause() {
        let input = "1 -2 8 -5 0\n";
        let result = parse_clause(input).expect("parse error").1;
        assert_eq!(result, vec![1, -2, 8, -5]);
    }

    #[test]
    fn test_empty_formula() {
        let input = "p cnf 0 0\n";
        let result: Vec<Vec<i32>> = parse_dimacs_cnf(input).expect("parse error").1;
        assert!(result.is_empty());
    }

    #[test]
    fn test_empty_clause_in_formula_mcs3() {
        let input = "p cnf 3 7\n-1 0\n1 0\n2 0\n-2 0\n3 0\n0\n-3 0\n";
        let result = parse_dimacs_cnf(input).expect("parse error").1;
        assert_eq!(
            result,
            vec![
                vec![-1],
                vec![1],
                vec![2],
                vec![-2],
                vec![3],
                vec![],
                vec![-3]
            ]
        );
    }

    #[test]
    fn test_parse_dimacs_cnf() {
        let input = "p cnf 2 2\n1 0\n-1 2 0\n";
        let result = parse_dimacs_cnf(input).expect("parse error").1;
        assert_eq!(result, vec![vec![1], vec![-1, 2]]);
    }

    #[test]
    fn test_parse_dimacs_cnf_without_first_line() {
        let input = "1 0\n-1 2 0\n";
        let result = parse_dimacs_cnf(input).expect("parse error").1;
        assert_eq!(result, vec![vec![1], vec![-1, 2]]);
    }
}
