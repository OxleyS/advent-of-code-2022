use crate::helpers::iterate_file_lines;

struct UnresolvedValve {
    valve_name: String,
    flow_rate: usize,
    tunnels: Vec<String>,
}

#[derive(Debug)]
struct Valve {
    flow_rate: usize,
    tunnels: Vec<usize>,
}

pub fn solve() {
    let (valves, start_idx) = parse_valves();
    let max_pressure = solve_part1(&valves, start_idx);
    println!("Max releasable pressure is {max_pressure}");
}

fn solve_part1(valves: &[Valve], start_idx: usize) -> usize {
    fn recurse(
        valves: &[Valve],
        cur_released: usize,
        release_rate: usize,
        cur_valve: usize,
        prev_valve: usize,
        minutes: usize,
        valves_open: &[bool],
    ) -> usize {
        let valve = &valves[cur_valve];
        let cur_released = cur_released + release_rate;
        if minutes == 1 {
            // One minute left, might as well open up the valve here
            return cur_released;
        }

        // They can go to another valve, but no point doubling back without doing anything
        let best_tunnel = valve
            .tunnels
            .iter()
            .filter(|&&idx| idx != prev_valve)
            .map(|idx| {
                recurse(
                    valves,
                    cur_released,
                    release_rate,
                    *idx,
                    cur_valve,
                    minutes - 1,
                    valves_open,
                )
            })
            .max()
            .unwrap_or(cur_released);

        // Or they can open this valve if there's a point
        let best_score = if valve.flow_rate != 0 && !valves_open[cur_valve] {
            let mut with_opened = valves_open.to_vec();
            with_opened[cur_valve] = true;
            recurse(
                valves,
                cur_released,
                release_rate + valve.flow_rate,
                cur_valve,
                cur_valve,
                minutes - 1,
                &with_opened,
            )
            .max(best_tunnel)
        } else {
            best_tunnel
        };

        best_score
    }

    recurse(valves, 0, 0, start_idx, start_idx, 30, &vec![false; valves.len()])
}

fn parse_valves() -> (Vec<Valve>, usize) {
    let unresolved_valves: Vec<UnresolvedValve> = iterate_file_lines("day16input.txt")
        .map(|line| {
            let (valve_str, tunnels_str) = line.split_once(';').expect("Malformed line");
            let (start, flow_rate_str) = valve_str.split_once('=').expect("Malformed flow section");

            let name_offset = "Valve ".len();
            let valve_name = start[name_offset..name_offset + 2].to_string();
            let flow_rate = flow_rate_str.parse::<usize>().expect("Malformed flow rate");

            let tunnels_start = &tunnels_str[" tunnels lead to ".len()..];
            let tunnels: Vec<String> = if tunnels_start.starts_with("valves") {
                tunnels_start["valves ".len()..].split(", ").map(|s| s.to_string()).collect()
            } else {
                vec![tunnels_start["valve ".len()..].to_string()]
            };

            UnresolvedValve { valve_name, flow_rate, tunnels }
        })
        .collect();

    let valves = unresolved_valves
        .iter()
        .map(|unresolved| {
            let tunnels = unresolved
                .tunnels
                .iter()
                .map(|name| {
                    unresolved_valves
                        .iter()
                        .position(|v| v.valve_name == *name)
                        .expect("Bad valve reference")
                })
                .collect();
            Valve { flow_rate: unresolved.flow_rate, tunnels }
        })
        .collect::<Vec<Valve>>();

    let start_idx =
        unresolved_valves.iter().position(|v| v.valve_name == "AA").expect("Start valve missing");

    (valves, start_idx)
}
