use nom::bytes::complete::{tag, take_until};
use nom::character::complete::{line_ending, space1};
use nom::multi::separated_list1;
use nom::sequence::pair;
use nom::IResult;

fn parse_clause(input: &str) -> IResult<&str, Vec<i32>> {
    let (input, clause_input) = take_until(" 0")(input)?;
    let (_, literals) = separated_list1(space1, nom::character::complete::i32)(clause_input)?;
    Ok((input, literals))
}

pub fn parse_dimacs_cnf(input: &str) -> IResult<&str, Vec<Vec<i32>>> {
    let (input, _) = tag("p cnf ")(input)?;
    let (input, _) = pair(take_until("\n"), tag("\n"))(input)?;
    let (input, clauses) = separated_list1(pair(tag(" 0"), line_ending), parse_clause)(input)?;
    Ok((input, clauses))
}
