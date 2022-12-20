use std::time::Instant;

use crate::helpers::iterate_file_lines;

struct UnresolvedTunnel {
    dest_valve: String,
    minutes: usize,
}

struct UnresolvedValve {
    valve_name: String,
    flow_rate: usize,
    tunnels: Vec<UnresolvedTunnel>,
}

#[derive(Debug)]
struct Tunnel {
    dest_idx: usize,
    minutes: usize,
}

#[derive(Debug)]
struct Valve {
    flow_rate: usize,
    tunnels: Vec<Tunnel>,
}

pub fn solve() {
    let (valves, start_idx) = parse_valves();

    let start_time = Instant::now();
    let max_pressure = solve_part1(&valves, start_idx);
    let time_taken = Instant::elapsed(&start_time);
    println!("Max releasable pressure alone is {max_pressure}");
    println!("This took {} seconds", time_taken.as_secs_f32());

    // let start_time = Instant::now();
    // let max_pressure = solve_part2(&valves, start_idx);
    // let time_taken = Instant::elapsed(&start_time);
    // println!("Max releasable pressure with elephant is {max_pressure}");
    // println!("This took {} seconds", time_taken.as_secs_f32());
}

fn solve_part1(valves: &[Valve], start_idx: usize) -> usize {
    fn recurse(
        valves: &[Valve],
        cur_released: usize,
        cur_valve: usize,
        prev_valve: usize,
        minutes: usize,
        valves_open: &mut [bool],
        num_open: usize,
    ) -> usize {
        let valve = &valves[cur_valve];
        if minutes <= 1 || num_open == valves.len() {
            // Nothing else to do
            return cur_released;
        }

        // They can go to another valve, but no point doubling back without doing anything
        let best_tunnel = valve
            .tunnels
            .iter()
            .filter(|tunnel| tunnel.dest_idx != prev_valve)
            .map(|tunnel| {
                recurse(
                    valves,
                    cur_released,
                    tunnel.dest_idx,
                    cur_valve,
                    minutes.saturating_sub(tunnel.minutes),
                    valves_open,
                    num_open,
                )
            })
            .max()
            .unwrap_or(cur_released);

        // If we can't open the valve, stop here
        if valves_open[cur_valve] {
            return best_tunnel;
        }

        valves_open[cur_valve] = true;
        let new_released = cur_released + (valve.flow_rate * minutes.saturating_sub(1));
        let best_if_opened = recurse(
            valves,
            new_released,
            cur_valve,
            cur_valve,
            minutes - 1,
            valves_open,
            num_open + 1,
        );
        valves_open[cur_valve] = false;
        best_if_opened.max(best_tunnel)
    }

    let mut num_open = 0;
    let mut valves_open = vec![false; valves.len()];
    for (i, valve) in valves.iter().enumerate() {
        if valve.flow_rate == 0 {
            valves_open[i] = true;
            num_open += 1;
        }
    }

    recurse(valves, 0, start_idx, start_idx, 30, &mut valves_open, num_open)
}

// fn solve_part2(valves: &[Valve], start_idx: usize) -> usize {
//     fn recurse_human(
//         valves: &[Valve],
//         cur_released: usize,
//         release_rate: usize,
//         cur_human: usize,
//         prev_human: usize,
//         cur_elephant: usize,
//         prev_elephant: usize,
//         minutes: usize,
//         valves_open: &mut [bool],
//         num_open: usize,
//     ) -> usize {
//         let valve = &valves[cur_human];
//         let cur_released = cur_released + release_rate;
//         if minutes == 1 || num_open == valves.len() {
//             // Nothing else to do
//             return cur_released + (release_rate * (minutes - 1));
//         }

//         // They can go to another valve, but no point doubling back without doing anything
//         let best_tunnel = valve
//             .tunnels
//             .iter()
//             .filter(|&&idx| idx != prev_human)
//             .map(|idx| {
//                 recurse_elephant(
//                     valves,
//                     cur_released,
//                     release_rate,
//                     *idx,
//                     cur_human,
//                     cur_elephant,
//                     prev_elephant,
//                     minutes,
//                     valves_open,
//                     num_open,
//                 )
//             })
//             .max()
//             .unwrap_or(cur_released);

//         // If we can't open the valve, stop here
//         if valves_open[cur_human] {
//             return best_tunnel;
//         }

//         valves_open[cur_human] = true;
//         let best_if_opened = recurse_elephant(
//             valves,
//             cur_released,
//             release_rate + valve.flow_rate,
//             cur_human,
//             cur_human,
//             cur_elephant,
//             prev_elephant,
//             minutes,
//             valves_open,
//             num_open + 1,
//         );
//         valves_open[cur_human] = false;
//         best_if_opened.max(best_tunnel)
//     }

//     fn recurse_elephant(
//         valves: &[Valve],
//         cur_released: usize,
//         release_rate: usize,
//         cur_human: usize,
//         prev_human: usize,
//         cur_elephant: usize,
//         prev_elephant: usize,
//         minutes: usize,
//         valves_open: &mut [bool],
//         num_open: usize,
//     ) -> usize {
//         let valve = &valves[cur_elephant];

//         // They can go to another valve, but no point doubling back without doing anything
//         let best_tunnel = valve
//             .tunnels
//             .iter()
//             .filter(|&&idx| idx != prev_elephant)
//             .map(|idx| {
//                 recurse_human(
//                     valves,
//                     cur_released,
//                     release_rate,
//                     cur_human,
//                     prev_human,
//                     *idx,
//                     cur_elephant,
//                     minutes - 1,
//                     valves_open,
//                     num_open,
//                 )
//             })
//             .max()
//             .unwrap_or(cur_released);

//         // If we can't open the valve, stop here
//         if valves_open[cur_elephant] {
//             return best_tunnel;
//         }

//         valves_open[cur_elephant] = true;
//         let best_if_opened = recurse_human(
//             valves,
//             cur_released,
//             release_rate + valve.flow_rate,
//             cur_human,
//             prev_human,
//             cur_elephant,
//             cur_elephant,
//             minutes - 1,
//             valves_open,
//             num_open + 1,
//         );
//         valves_open[cur_elephant] = false;
//         best_if_opened.max(best_tunnel)
//     }

//     let mut num_open = 0;
//     let mut valves_open = vec![false; valves.len()];
//     for (i, valve) in valves.iter().enumerate() {
//         if valve.flow_rate == 0 {
//             valves_open[i] = true;
//             num_open += 1;
//         }
//     }

//     recurse_human(
//         valves,
//         0,
//         0,
//         start_idx,
//         start_idx,
//         start_idx,
//         start_idx,
//         26,
//         &mut valves_open,
//         num_open,
//     )
// }

fn parse_valves() -> (Vec<Valve>, usize) {
    let mut unresolved_valves: Vec<UnresolvedValve> = iterate_file_lines("day16input.txt")
        .map(|line| {
            let (valve_str, tunnels_str) = line.split_once(';').expect("Malformed line");
            let (start, flow_rate_str) = valve_str.split_once('=').expect("Malformed flow section");

            let name_offset = "Valve ".len();
            let valve_name = start[name_offset..name_offset + 2].to_string();
            let flow_rate = flow_rate_str.parse::<usize>().expect("Malformed flow rate");

            let tunnels_start = &tunnels_str[" tunnels lead to ".len()..];
            let tunnels: Vec<UnresolvedTunnel> = if tunnels_start.starts_with("valves") {
                tunnels_start["valves ".len()..]
                    .split(", ")
                    .map(|s| UnresolvedTunnel { dest_valve: s.to_string(), minutes: 1 })
                    .collect()
            } else {
                vec![UnresolvedTunnel {
                    dest_valve: tunnels_start["valve ".len()..].to_string(),
                    minutes: 1,
                }]
            };

            UnresolvedValve { valve_name, flow_rate, tunnels }
        })
        .collect();

    reduce_valves(&mut unresolved_valves);

    let valves = unresolved_valves
        .iter()
        .map(|unresolved| {
            let tunnels = unresolved
                .tunnels
                .iter()
                .map(|tunnel| {
                    let dest_idx = unresolved_valves
                        .iter()
                        .position(|v| v.valve_name == tunnel.dest_valve)
                        .expect("Bad valve reference");
                    Tunnel { dest_idx, minutes: tunnel.minutes }
                })
                .collect();
            Valve { flow_rate: unresolved.flow_rate, tunnels }
        })
        .collect::<Vec<Valve>>();

    let start_idx =
        unresolved_valves.iter().position(|v| v.valve_name == "AA").expect("Start valve missing");

    (valves, start_idx)
}

fn reduce_valves(valves: &mut Vec<UnresolvedValve>) {
    let mut i = 0;
    while i < valves.len() {
        if valves[i].flow_rate != 0 || valves[i].valve_name == "AA" {
            i += 1;
            continue;
        }

        let removed = valves.swap_remove(i);
        for valve in valves.iter_mut() {
            let Some(tunnel_idx) = valve.tunnels.iter().position(|t| t.dest_valve == removed.valve_name) else {
                continue;
            };

            let prev_minutes = valve.tunnels.swap_remove(tunnel_idx).minutes;
            for new_valve in removed.tunnels.iter() {
                if new_valve.dest_valve != valve.valve_name
                    && !valve.tunnels.iter().any(|t| t.dest_valve == new_valve.dest_valve)
                {
                    valve.tunnels.push(UnresolvedTunnel {
                        dest_valve: new_valve.dest_valve.clone(),
                        minutes: prev_minutes + new_valve.minutes,
                    });
                }
            }
        }
    }
}
