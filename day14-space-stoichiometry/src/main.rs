use regex::Regex;
use std::cmp;
use std::collections::HashMap;
use std::{fs::read_to_string, io};

fn main() -> io::Result<()> {
    let file = read_to_string("input.txt")?;

    println!("{}", part1(&file));
    println!("{}", part2(&file));

    Ok(())
}

fn part1(input: &str) -> usize {
    do_reactions(
        &input
            .trim()
            .split('\n')
            .map(|l| parse_reaction(l))
            .collect(),
        &mut vec![Chemical::new("FUEL".to_owned(), 1)],
    )
}

fn part2(input: &str) -> usize {
    let ore_needed_for_1_fuel = do_reactions(
        &input
            .trim()
            .split('\n')
            .map(|l| parse_reaction(l))
            .collect(),
        &mut vec![Chemical::new("FUEL".to_owned(), 1)],
    );

    let mut min_fuel = 1000000000000 / ore_needed_for_1_fuel;
    let mut max_fuel = 2 * min_fuel;
    let mut target_fuel = min_fuel + (max_fuel - min_fuel) / 2;

    loop {
        let ore_needed = do_reactions(
            &input
                .trim()
                .split('\n')
                .map(|l| parse_reaction(l))
                .collect(),
            &mut vec![Chemical::new("FUEL".to_owned(), target_fuel)],
        );

        if ore_needed > 1000000000000 {
            max_fuel = target_fuel;
        } else if ore_needed < 1000000000000 {
            min_fuel = target_fuel;
        } else {
            break;
        }
        target_fuel = min_fuel + (max_fuel - min_fuel) / 2;

        if target_fuel == min_fuel || target_fuel == max_fuel {
            break;
        }
    }

    target_fuel
}

fn parse_chemical(line: &str) -> Chemical {
    let chemical_reg = Regex::new(r"([0-9]+)\s([A-Z]+)").unwrap();
    let captures = chemical_reg.captures_iter(line).next().unwrap();
    Chemical::new(
        captures[2].to_owned(),
        captures[1].parse::<usize>().unwrap(),
    )
}

fn do_reactions(reactions: &Vec<Reaction>, what_is_needed: &mut Vec<Chemical>) -> usize {
    let mut ore_needed = 0;
    let mut over_production_map: HashMap<String, usize> = HashMap::new();
    loop {
        match what_is_needed.pop() {
            Some(chemical_to_produce) => {
                if chemical_to_produce.quantity == 0 {
                    continue;
                }

                let reaction = reactions
                    .iter()
                    .find(|r| chemical_to_produce.kind == r.output.kind)
                    .unwrap();

                let times_reaction_needed =
                    match chemical_to_produce.quantity % reaction.output.quantity {
                        0 => chemical_to_produce.quantity / reaction.output.quantity,
                        _ => chemical_to_produce.quantity / reaction.output.quantity + 1,
                    };

                let over_production = times_reaction_needed * reaction.output.quantity
                    - chemical_to_produce.quantity
                    + match over_production_map.get(&chemical_to_produce.kind) {
                        Some(n) => n,
                        _ => &0,
                    };

                if over_production > 0 {
                    over_production_map.insert(chemical_to_produce.kind, over_production);
                }

                for chemical_input in reaction.inputs.iter() {
                    let mut quantity_needed = chemical_input.quantity * times_reaction_needed;

                    if chemical_input.kind == "ORE" {
                        ore_needed += quantity_needed;
                        continue;
                    }

                    let quantity_over_produced = match over_production_map.get(&chemical_input.kind)
                    {
                        Some(n) => n,
                        _ => &0,
                    };

                    let consumed_quantity =
                        cmp::min(quantity_needed, quantity_over_produced.clone());

                    quantity_needed -= consumed_quantity;

                    if quantity_over_produced > &0 {
                        over_production_map.insert(
                            chemical_input.kind.clone(),
                            quantity_over_produced - consumed_quantity,
                        );
                    }

                    match what_is_needed
                        .iter_mut()
                        .find(|c| c.kind == chemical_input.kind)
                    {
                        Some(chemical) => chemical.quantity += quantity_needed,
                        _ => what_is_needed
                            .push(Chemical::new(chemical_input.kind.clone(), quantity_needed)),
                    }
                }
            }
            _ => break,
        }
    }

    ore_needed
}

fn parse_reaction(line: &str) -> Reaction {
    let reaction_reg = Regex::new(r"(.*)\s=>\s(.*)").unwrap();
    let captures = reaction_reg.captures_iter(line).next().unwrap();
    let inputs: Vec<Chemical> = captures[1]
        .split(",")
        .map(|c| c.trim())
        .map(|c| parse_chemical(c))
        .collect();
    let output: Chemical = parse_chemical(&captures[2]);
    Reaction::new(inputs, output)
}

#[derive(Debug, PartialEq)]
struct Chemical {
    kind: String,
    quantity: usize,
}

impl Chemical {
    fn new(kind: String, quantity: usize) -> Self {
        Chemical {
            kind: kind,
            quantity: quantity,
        }
    }
}

#[derive(Debug, PartialEq)]
struct Reaction {
    inputs: Vec<Chemical>,
    output: Chemical,
}

impl Reaction {
    fn new(inputs: Vec<Chemical>, output: Chemical) -> Self {
        Reaction {
            inputs: inputs,
            output: output,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::parse_reaction;
    use super::part1;
    use super::part2;
    use super::Chemical;
    use super::Reaction;

    #[test]
    fn it_should_parse_input_lines() {
        assert_eq!(
            parse_reaction(&"10 ORE => 10 A"),
            Reaction::new(
                vec![Chemical::new("ORE".to_owned(), 10)],
                Chemical::new("A".to_owned(), 10)
            )
        );

        assert_eq!(
            parse_reaction(&"7 A, 1 E => 1 FUEL"),
            Reaction::new(
                vec![
                    Chemical::new("A".to_owned(), 7),
                    Chemical::new("E".to_owned(), 1)
                ],
                Chemical::new("FUEL".to_owned(), 1)
            )
        );
    }

    #[test]
    fn it_should_compute_the_ore_quantity_needed_1() {
        assert_eq!(
            part1(
                &"
            10 ORE => 10 A
            1 ORE => 1 B
            7 A, 1 B => 1 C
            7 A, 1 C => 1 D
            7 A, 1 D => 1 E
            7 A, 1 E => 1 FUEL
        "
            ),
            31
        );
    }

    #[test]
    fn it_should_compute_the_ore_quantity_needed_2() {
        assert_eq!(
            part1(
                &"
            9 ORE => 2 A
            8 ORE => 3 B
            7 ORE => 5 C
            3 A, 4 B => 1 AB
            5 B, 7 C => 1 BC
            4 C, 1 A => 1 CA
            2 AB, 3 BC, 4 CA => 1 FUEL
        "
            ),
            165
        );
    }

    #[test]
    fn it_should_compute_the_ore_quantity_needed_3() {
        assert_eq!(
            part1(
                &"
            157 ORE => 5 NZVS
            165 ORE => 6 DCFZ
            44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
            12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
            179 ORE => 7 PSHF
            177 ORE => 5 HKGWZ
            7 DCFZ, 7 PSHF => 2 XJWVT
            165 ORE => 2 GPVTF
            3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT
        "
            ),
            13312
        );
    }

    #[test]
    fn it_should_compute_the_ore_quantity_needed_4() {
        assert_eq!(
            part1(
                &"
            2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
            17 NVRVD, 3 JNWZP => 8 VPVL
            53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
            22 VJHF, 37 MNCFX => 5 FWMGM
            139 ORE => 4 NVRVD
            144 ORE => 7 JNWZP
            5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
            5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
            145 ORE => 6 MNCFX
            1 NVRVD => 8 CXFTF
            1 VJHF, 6 MNCFX => 4 RFSQX
            176 ORE => 6 VJHF
        "
            ),
            180697
        );
    }

    #[test]
    fn it_should_compute_the_ore_quantity_needed_5() {
        assert_eq!(
            part1(
                &"
            171 ORE => 8 CNZTR
            7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
            114 ORE => 4 BHXH
            14 VRPVC => 6 BMBT
            6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
            6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
            15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
            13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
            5 BMBT => 4 WPTQ
            189 ORE => 9 KTJDG
            1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
            12 VRPVC, 27 CNZTR => 2 XDBXC
            15 KTJDG, 12 BHXH => 5 XCVML
            3 BHXH, 2 VRPVC => 7 MZWV
            121 ORE => 7 VRPVC
            7 XCVML => 6 RJRHP
            5 BHXH, 4 VRPVC => 5 LTCX
        "
            ),
            2210736
        );
    }

    #[test]
    fn it_should_compute_the_fuel_output_quantity_1() {
        assert_eq!(
            part2(
                &"
            157 ORE => 5 NZVS
            165 ORE => 6 DCFZ
            44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
            12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
            179 ORE => 7 PSHF
            177 ORE => 5 HKGWZ
            7 DCFZ, 7 PSHF => 2 XJWVT
            165 ORE => 2 GPVTF
            3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT
        "
            ),
            82892753
        );
    }

    #[test]
    fn it_should_compute_the_fuel_output_quantity_2() {
        assert_eq!(
            part2(
                &"
            2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
            17 NVRVD, 3 JNWZP => 8 VPVL
            53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
            22 VJHF, 37 MNCFX => 5 FWMGM
            139 ORE => 4 NVRVD
            144 ORE => 7 JNWZP
            5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
            5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
            145 ORE => 6 MNCFX
            1 NVRVD => 8 CXFTF
            1 VJHF, 6 MNCFX => 4 RFSQX
            176 ORE => 6 VJHF
        "
            ),
            5586022
        );
    }

    #[test]
    fn it_should_compute_the_fuel_output_quantity_3() {
        assert_eq!(
            part2(
                &"
            171 ORE => 8 CNZTR
            7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
            114 ORE => 4 BHXH
            14 VRPVC => 6 BMBT
            6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
            6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
            15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
            13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
            5 BMBT => 4 WPTQ
            189 ORE => 9 KTJDG
            1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
            12 VRPVC, 27 CNZTR => 2 XDBXC
            15 KTJDG, 12 BHXH => 5 XCVML
            3 BHXH, 2 VRPVC => 7 MZWV
            121 ORE => 7 VRPVC
            7 XCVML => 6 RJRHP
            5 BHXH, 4 VRPVC => 5 LTCX
        "
            ),
            460664
        );
    }
}
