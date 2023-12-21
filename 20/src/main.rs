use std::collections::{HashMap, VecDeque, HashSet};

const INPUT: &str = "%zl -> zp, cl
%vp -> dj, vr
%cc -> xp
&dj -> lq, mb, dc, ns, gz
%md -> ts, zp
%fc -> zp
%px -> zx
&nx -> gl, br, pr, xf, vd, gj, kd
%tf -> lt, dj
%fj -> pc
%mb -> xx
%cl -> mj
%pm -> fj
%dc -> dj, vp
%jc -> bz, xm
&vd -> zh
%pz -> sr, nx
&ns -> zh
%sr -> nx
%gl -> pr
%xx -> nt, dj
%gp -> md
%hb -> jl, nx
&zh -> rx
%rb -> gz, dj
%xm -> bz
&zp -> px, gp, cl, bh, fn, ls, hs
&bz -> pm, pc, bv, dl, jp, fj, cc
%nl -> bz, pm
&bh -> zh
%hq -> gj, nx
%bv -> bz, nl
%bj -> jp, bz
%gj -> mx
%xp -> bz, bj
%vr -> dj, mb
&dl -> zh
%pr -> hb
%nt -> dj, lq
%mx -> gl, nx
%kd -> hq
%fn -> px
%jp -> xc
%zx -> zl, zp
%br -> nx, xf
%lt -> dj
%df -> dj, tf
%ts -> zp, fc
%jl -> nx, pz
%xc -> jc, bz
%xf -> kd
%lq -> rb
%gz -> df
%pc -> cc
%hs -> fn
broadcaster -> ls, bv, dc, br
%mj -> zp, gp
%ls -> hs, zp";

type Pulse = bool;
type GateId = String;

#[derive(Debug, Clone)]
struct Broadcaster {
    targets: Vec<GateId>
}

#[derive(Debug, Clone)]
struct Flipflop {
    targets: Vec<GateId>,
    state: Pulse,
}

#[derive(Debug, Clone)]
struct Conjunction {
    targets: Vec<GateId>,
    inputs: HashMap<GateId, Pulse>,
}

#[derive(Debug, Clone)]
enum Gate {
    Broadcaster(Broadcaster),
    Flipflop(Flipflop),
    Conjunction(Conjunction)
}
impl Gate {
    fn targets(&self) -> &Vec<String> {
        match self {
            Self::Broadcaster(state) => &state.targets,
            Self::Flipflop(state) => &state.targets,
            Self::Conjunction(state) => &state.targets
        }
    }

    fn update(&mut self, id: &GateId, pulse: Pulse, source: &Option<GateId>, queue: &mut VecDeque<(GateId, Pulse, Option<GateId>)>) -> Option<bool> {
        match self {
            Self::Broadcaster(state) => {
                for t in &state.targets {
                    queue.push_back((t.clone(), pulse, Some(id.clone())));
                }
                Some(pulse)
            },
            Self::Flipflop(state) => {
                if !pulse {
                    state.state = !state.state;
                    for t in &state.targets {
                        queue.push_back((t.clone(), state.state, Some(id.clone())));
                    }
                    Some(state.state)
                } else {
                    None
                }
            },
            Self::Conjunction(state) => {
                if let Some(source) = source {
                    *state.inputs.get_mut(source).unwrap() = pulse;
                }
                let out = !state.inputs.iter().all(|(_, p)| *p);
                for t in &state.targets {
                    queue.push_back((t.clone(), out, Some(id.clone())))
                }
                Some(out)
            }
        }
    }
}

fn parse(input: &str) -> HashMap<GateId, Gate> {
    let mut connections = input.lines().map(|line| {
        let (name, targets) = line.split_once(" -> ").unwrap();
        let targets = targets.split(", ").map(|s| s.to_string()).collect();
        let gate = match &name[0..1] {
            "%" => (name[1..].to_string(), Gate::Flipflop(Flipflop { targets, state: false })),
            "&" => (name[1..].to_string(), Gate::Conjunction(Conjunction { targets, inputs: HashMap::new() })),
            _ => (name.to_string(), Gate::Broadcaster(Broadcaster { targets })),
        };
        gate
    }).collect::<HashMap<_, _>>();
    
    let conj_connections = connections.iter().filter(|(k, v)|
        matches!(v, Gate::Conjunction(_))
    ).map(|(k, _)| (k.clone(), 
                    connections.iter()
                    .filter(|(_, v)|
                        v.targets().contains(k)
                    ).map(|(k, _)| k.clone())
                    .collect::<Vec<_>>()
    )).collect::<Vec<_>>();
    for (k, c) in conj_connections {
        if let Some(Gate::Conjunction(gate)) = connections.get_mut(&k) {
            for c in c {
                gate.inputs.insert(c, false);
            }
        }
    }
    // println!("{connections:?}");
    connections
}

fn press(connections: &mut HashMap<GateId, Gate>, mut hit_detectors: Option<&mut HashMap<String, usize>>, i: usize) -> (usize, usize) {
    let mut queue: VecDeque<(GateId, Pulse, Option<GateId>)> = VecDeque::from([("broadcaster".to_string(), false, None)]);
    let mut low_count = 1_usize;
    let mut high_count = 0_usize;

    while let Some((dest, pulse, src)) = queue.pop_front() {
        if let Some(ref src) = src {
            // println!("{src:?} {pulse}-> {dest:?}");
            if pulse { high_count += 1; } else { low_count += 1; }
        }

        if let Some(gate) = connections.get_mut(&dest) {
            let last_pulse = gate.update(&dest, pulse, &src, &mut queue);

            if let Some(ref mut d) = hit_detectors {
                if let Some(c) = d.get_mut(&dest) {
                    if matches!(last_pulse, Some(true)) && *c == 0 {
                        // println!("{dest} {i}");
                        *c = i;
                    }
                }
            }

        }
    }
    (low_count, high_count)
}

// too lazy to combine both into single solution
fn solve_a(connections: &mut HashMap<GateId, Gate>) -> usize {
    let solution = (0..1000).fold((0, 0), |acc, i| {
        let (low, high) = press(connections, None, i);
        (acc.0 + low, acc.1 + high)
    });
    solution.0 * solution.1
}

fn lcm(arr: impl Iterator<Item = usize>) -> usize {
    fn gcd(a: usize, b: usize) -> usize {
        if a * b == 0 {
            a.max(b)
        } else {
            gcd(a.max(b) % a.min(b), a.min(b))
        }
    }
    arr.reduce(|acc, v| acc * v / gcd(acc, v)).unwrap()
}

// rx is parented by conjunction (zh)
// all inputs to zh must be high to output low
// lcm of all inputs parented to zh when they are outputting high
fn solve_b(connections: &mut HashMap<GateId, Gate>) -> usize {
    let rx_parent = connections.iter().find(|c| c.1.targets().contains(&"rx".to_string())).unwrap().0.clone();
    let mut rx_parent_parents = connections.iter().filter(|c| c.1.targets().contains(&rx_parent)).map(|c| (c.0.clone(), 0_usize)).collect::<HashMap<_, _>>();
    let mut counter = 1;
    while rx_parent_parents.iter().find(|(_, c)| **c == 0).is_some() {
        press(connections, Some(&mut rx_parent_parents), counter);
        counter += 1;
    }
    lcm(rx_parent_parents.iter().map(|(_, v)| *v))
}

fn main() {
    let mut connections = parse(INPUT);
    let solution = solve_a(&mut connections);
    println!("{solution:?}");
    let mut connections = parse(INPUT);
    let solution = solve_b(&mut connections);
    println!("{solution:?}");
}
