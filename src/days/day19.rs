use fxhash::FxHashMap;
use nom::{
    branch::alt, bytes::complete::tag, combinator::map, multi::separated_list1, sequence::tuple,
    IResult,
};

use crate::{DayResult, IntoDayResult};

pub fn solve(_input: &str) -> anyhow::Result<DayResult> {
    let mut rulesets = FxHashMap::default();
    let mut inputs = vec![];
    let mut lines = _input.lines();
    for (_, ruleset) in lines.by_ref().map_while(|line| parse_ruleset(line).ok()) {
        rulesets.insert(ruleset.name, ruleset);
    }
    for line in lines {
        let (_, input) = parse_input(line).map_err(|err| anyhow::anyhow!("{err}"))?;
        inputs.push(input);
    }

    let mut p1 = 0;

    for input in inputs {
        let mut pos = &rulesets["in"];
        loop {
            match pos.apply(input) {
                Action::Termination(termination) => {
                    if termination == Termination::Accept {
                        p1 += input.total();
                    }
                    break;
                }
                Action::Next(next) => {
                    pos = &rulesets[next];
                }
            }
        }
    }

    let p2 = rulesets["in"].max_possible(InputMinMax::default(), &rulesets);

    (p1, p2).into_result()
}

#[derive(Debug)]
struct Ruleset<'a> {
    name: &'a str,
    rules: Vec<Rule<'a>>,
    default: Action<'a>,
}

impl<'a> Ruleset<'a> {
    fn apply(&self, input: Input) -> Action<'a> {
        for rule in &self.rules {
            if let Some(action) = rule.apply(input) {
                return action;
            }
        }
        self.default
    }

    fn max_possible(&self, mut minmax: InputMinMax, rulesets: &FxHashMap<&str, Ruleset>) -> u64 {
        let mut res = 0;

        for rule in &self.rules {
            let mut minmax_branch = minmax;
            let crit = minmax.retrieve_mut(rule.criteria);
            let crit_branch = minmax_branch.retrieve_mut(rule.criteria);

            match rule.op {
                Op::Gt => {
                    crit.max = rule.than;
                    crit_branch.min = rule.than + 1;
                }
                Op::Lt => {
                    crit.min = rule.than;
                    crit_branch.max = rule.than - 1;
                }
            }

            res += match rule.action {
                Action::Termination(termination) => match termination {
                    Termination::Accept => minmax_branch.possible(),
                    Termination::Reject => 0,
                },
                Action::Next(ruleset) => rulesets[ruleset].max_possible(minmax_branch, rulesets),
            };
        }

        res += match self.default {
            Action::Termination(termination) => match termination {
                Termination::Accept => minmax.possible(),
                Termination::Reject => 0,
            },
            Action::Next(ruleset) => rulesets[ruleset].max_possible(minmax, rulesets),
        };

        res
    }
}

fn parse_rule_name(input: &str) -> IResult<&str, &str> {
    nom::bytes::complete::is_a("qwertyuiopasdfghjklzxcvbnm")(input)
}

fn parse_ruleset(input: &str) -> IResult<&str, Ruleset> {
    map(
        tuple((
            parse_rule_name,
            tag("{"),
            separated_list1(tag(","), parse_rule),
            tag(","),
            parse_action,
            tag("}"),
        )),
        |(name, _, rules, _, default, _)| Ruleset {
            name,
            rules,
            default,
        },
    )(input)
}

#[derive(Debug)]
struct Rule<'a> {
    criteria: Criteria,
    op: Op,
    than: u64,
    action: Action<'a>,
}

impl<'a> Rule<'a> {
    fn apply(&self, input: Input) -> Option<Action<'a>> {
        let val = input.retrieve(self.criteria);
        let passes = match self.op {
            Op::Gt => val > self.than,
            Op::Lt => val < self.than,
        };
        if passes {
            return Some(self.action);
        }
        None
    }
}

fn parse_rule(input: &str) -> IResult<&str, Rule> {
    map(
        tuple((
            parse_criteria,
            parse_op,
            nom::character::complete::u64,
            tag(":"),
            parse_action,
        )),
        |(criteria, op, than, _, action)| Rule {
            criteria,
            op,
            than,
            action,
        },
    )(input)
}

#[derive(Debug, Copy, Clone)]
enum Criteria {
    X,
    M,
    A,
    S,
}

fn parse_criteria(input: &str) -> IResult<&str, Criteria> {
    alt((
        map(tag("x"), |_| Criteria::X),
        map(tag("m"), |_| Criteria::M),
        map(tag("a"), |_| Criteria::A),
        map(tag("s"), |_| Criteria::S),
    ))(input)
}

#[derive(Debug)]
enum Op {
    Gt,
    Lt,
}

fn parse_op(input: &str) -> IResult<&str, Op> {
    alt((map(tag("<"), |_| Op::Lt), map(tag(">"), |_| Op::Gt)))(input)
}

#[derive(Debug, Copy, Clone)]
enum Action<'a> {
    Termination(Termination),
    Next(&'a str),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Termination {
    Accept,
    Reject,
}

fn parse_action(input: &str) -> IResult<&str, Action> {
    alt((
        map(tag("R"), |_| Action::Termination(Termination::Reject)),
        map(tag("A"), |_| Action::Termination(Termination::Accept)),
        map(parse_rule_name, Action::Next),
    ))(input)
}

#[derive(Debug, Clone, Copy)]
struct Input {
    x: u64,
    m: u64,
    a: u64,
    s: u64,
}

impl Input {
    fn retrieve(self, criteria: Criteria) -> u64 {
        match criteria {
            Criteria::X => self.x,
            Criteria::M => self.m,
            Criteria::A => self.a,
            Criteria::S => self.s,
        }
    }

    fn total(self) -> u64 {
        let Input { x, m, a, s } = self;
        x + m + a + s
    }
}

fn parse_input(input: &str) -> IResult<&str, Input> {
    map(
        tuple((
            tag("{x="),
            nom::character::complete::u64,
            tag(",m="),
            nom::character::complete::u64,
            tag(",a="),
            nom::character::complete::u64,
            tag(",s="),
            nom::character::complete::u64,
            tag("}"),
        )),
        |(_, x, _, m, _, a, _, s, _)| Input { x, m, a, s },
    )(input)
}

#[derive(Debug, Default, Copy, Clone)]
struct InputMinMax {
    x: MinMax,
    m: MinMax,
    a: MinMax,
    s: MinMax,
}

impl InputMinMax {
    fn retrieve_mut(&mut self, criteria: Criteria) -> &mut MinMax {
        match criteria {
            Criteria::X => &mut self.x,
            Criteria::M => &mut self.m,
            Criteria::A => &mut self.a,
            Criteria::S => &mut self.s,
        }
    }

    fn possible(&self) -> u64 {
        self.x.possible() * self.m.possible() * self.a.possible() * self.s.possible()
    }
}

#[derive(Debug, Copy, Clone)]
struct MinMax {
    min: u64,
    max: u64,
}

impl MinMax {
    fn possible(self) -> u64 {
        (self.max - self.min) + 1
    }
}

impl Default for MinMax {
    fn default() -> Self {
        MinMax { min: 1, max: 4000 }
    }
}

#[cfg(test)]
mod tests {
    use crate::{days::day19::solve, IntoDayResult};

    #[test]
    fn works_for_example() {
        const INPUT: &str = include_str!("../../input/day19_test.txt");
        let solution = solve(INPUT).unwrap();
        assert_eq!(
            (19_114, 167_409_079_868_000_isize).into_day_result(),
            solution
        );
    }

    #[test]
    fn works_for_input() {
        const INPUT: &str = include_str!("../../input/day19.txt");
        let solution = solve(INPUT).unwrap();
        assert_eq!(
            (476_889, 132_380_153_677_887_isize).into_day_result(),
            solution
        );
    }
}
