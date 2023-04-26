use crate::cnf::CNF;
use crate::literal::Literal;
use nom::bytes::complete::{tag, take_until};
use nom::character::complete::{i32, line_ending, space1, u32};
use nom::multi::separated_list1;
use nom::sequence::pair;
use nom::{IResult, Parser};

fn parse_clause(input: &str) -> IResult<&str, Vec<Literal>> {
    let (input, clause_input) = take_until(" 0")(input)?;
    let (_, literals) =
        separated_list1(space1, i32.map(|x| Literal::from_dimacs(x)))(clause_input)?;
    Ok((input, literals))
}

pub fn parse_dimacs_cnf(input: &str) -> IResult<&str, CNF> {
    let (input, _) = tag("p cnf ")(input)?;
    let (input, variable_count) = u32(input)?;
    let (input, _) = pair(take_until("\n"), tag("\n"))(input)?;
    let (input, clauses) = separated_list1(pair(tag(" 0"), line_ending), parse_clause)(input)?;
    Ok((input, CNF::new(clauses, variable_count as usize)))
}
