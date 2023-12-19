use std::collections::HashMap;

use nom::{
    bytes::complete::{is_not, tag},
    character::complete::{self, newline},
    character::complete::{alpha1, one_of},
    combinator::all_consuming,
    multi::separated_list1,
    sequence::{delimited, preceded, separated_pair},
    IResult,
};

use crate::types::{Comparison, Part, Rule, Workflow};

fn part(input: &str) -> IResult<&str, Part> {
    let (rest, params) = delimited(
        tag("{"),
        separated_list1(
            tag(","),
            separated_pair(one_of("xmas"), tag("="), complete::u32),
        ),
        tag("}"),
    )(input)?;

    let pair = Part {
        x: params.iter().find(|elem| elem.0 == 'x').expect("x").1,
        m: params.iter().find(|elem| elem.0 == 'm').expect("m").1,
        a: params.iter().find(|elem| elem.0 == 'a').expect("a").1,
        s: params.iter().find(|elem| elem.0 == 's').expect("s").1,
    };

    Ok((rest, pair))
}

fn rule(input: &str) -> IResult<&str, Rule> {
    let (input, param) = one_of("xmas")(input)?;
    let (input, cmp) = one_of("<>")(input)?;
    let (rest, (threshold, target)) = separated_pair(complete::u32, tag(":"), alpha1)(input)?;

    let rule = Rule {
        cat: param.into(),
        cmp: match cmp {
            '<' => Comparison::LessThan,
            '>' => Comparison::GreaterThan,
            _ => panic!("invalid char"),
        },
        threshold,
        target,
    };

    Ok((rest, rule))
}

fn workflow(input: &str) -> IResult<&str, Workflow> {
    let (input, name) = alpha1(input)?;
    let (rest, mut rules_raw) =
        delimited(tag("{"), separated_list1(tag(","), is_not(",}")), tag("}"))(input)?;

    let catch_all = rules_raw.pop().expect("rules is not empty");
    let workflow = Workflow {
        name: name,
        rules: rules_raw
            .iter()
            .map(|input| rule(input).expect("rule").1)
            .collect(),
        catch_all,
    };

    Ok((rest, workflow))
}

pub fn parse_file(input: &str) -> IResult<&str, (HashMap<&str, Workflow>, Vec<Part>)> {
    let (input, workflows) = separated_list1(newline, workflow)(input)?;
    let (rest, parts) = preceded(tag("\n\n"), separated_list1(newline, part))(input)?;

    let workflows = HashMap::from_iter(
        workflows.into_iter().map(|w| (w.name, w))
    );

    Ok((rest, (workflows, parts)))
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[test]
    fn quick_test() {}

    #[rstest]
    #[case("s<537:gd")]
    #[case("s>3448:A")]
    fn parsing_rule(#[case] fixture: &str) {
        let (_, content) = rule(fixture).expect("parse ok");

        println!("{:?}", content);
    }

    #[rstest]
    #[case("rfg{s<537:gd,x>2440:R,A}")]
    #[case("hdj{m>838:A,pv}")]
    fn parsing_workflow(#[case] fixture: &str) {
        let (_, content) = workflow(fixture).expect("parse ok");

        println!("{:?}", content);
    }

    #[test]
    fn parsin_part() {
        let fixture = "{x=787,m=2655,a=1222,s=2876}";
        let (_, content) = part(fixture).expect("parse ok");

        println!("{:?}", content);
    }

    #[test]
    fn parsing_file() {
        let fixture = include_str!("../test-1.txt");
        let (_, content) = parse_file(fixture).expect("parse ok");

        println!("{:?}", content);
    }
}
