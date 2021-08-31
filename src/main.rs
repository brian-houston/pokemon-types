use std::fs;
use std::error::Error;
use std::io::Write;
use itertools::Itertools;

fn main() -> Result<(), Box<dyn Error>> {
    let (types, chart) = parse_type_chart();
    let mut file = fs::File::create("complete_chart.csv")?;

    file.write_all("Attacking,".as_bytes())?;
    file.write_all(types.to_vec().join(",").as_bytes())?;

    for i in 1..=types.len() {
        let combinations = (0..types.len()).combinations(i);

        for combination in combinations {
            let type_string = get_type_string(&combination, &types);

            file.write_all("\n".as_bytes())?;
            file.write_all(type_string.as_bytes())?;

            for attack_type in 0..types.len() {
                let multiplier = calculate_multiplier(attack_type, &combination, &chart);

                let multiplier = match multiplier {
                    Some(n) if n >= 0 => 2_i32.pow(n as u32).to_string(),
                    Some(n) if n < 0 =>  2_f32.powf(n as f32).to_string(),
                    _ => String::from("0"), 
                };

                file.write_all(",".as_bytes())?;
                file.write_all(multiplier.as_bytes())?;
            }
        }
    }

    Ok(())
}

fn get_type_string(combination: &[usize], types: &[String]) -> String {
    combination
        .iter()
        .map(|i| types[*i].clone())
        .collect::<Vec<_>>()
        .join("-")
}

fn calculate_multiplier(attack_type: usize, combination: &[usize], chart: &[Vec<Option<i32>>]) -> Option<i32> {
    let mut multiplier = 0;

    for i in combination {
        multiplier += match chart[attack_type][*i] {
            Some(n) => n,
            None => return None,
        }
    }

    Some(multiplier)
}

fn parse_type_chart() -> (Vec<String>, Vec<Vec<Option<i32>>>) {
    let type_chart_string = fs::read_to_string("chart.csv").unwrap();
    let mut type_chart_lines = type_chart_string.lines();

    let types = type_chart_lines
        .next()
        .unwrap()
        .split(",")
        .skip(1)
        .map(str::to_string)
        .collect::<Vec<String>>();

    let mut type_chart = vec![];

    for line in type_chart_lines {
        let row = line
            .split(",")
            .skip(1)
            .map(|n| match n { 
                "0.5" => Some(-1),
                "1" => Some(0),
                "2" => Some(1),
                _ => None,
            })
        .collect::<Vec<Option<i32>>>();

        type_chart.push(row);
    }

    (types, type_chart)
}
