use std::collections::BTreeMap;

static INPUT: &str = include_str!("../input.txt");

type FormulaeBook = BTreeMap<&'static str, (u32, Vec<(u32, &'static str)>)>;

fn div_round_up(a: u32, b: u32) -> u32 {
    (a + b - 1) / b
}

fn parse_chemical(value: &str) -> (u32, &str) {
    let value = value.trim();
    let mut pieces = value.split(" ");
    let quantity = pieces.next().unwrap().parse().unwrap();
    let name = pieces.next().unwrap();

    assert_eq!(value, &format!("{} {}", quantity, name));

    (quantity, name)
}

fn parse_formulae(input: &'static str) -> FormulaeBook {
    let mut formulae = BTreeMap::new();

    for line in input.lines() {
        let mut halves = line.split("=>");
        let input_str = halves.next().unwrap();
        let output_str = halves.next().unwrap();

        let (output_amt, output_name) = parse_chemical(output_str);
        let inputs = input_str.split(',').map(parse_chemical).collect();

        formulae.insert(output_name, (output_amt, inputs));
    }

    formulae
}

#[derive(Debug, Clone, Default)]
struct Vat {
    store: BTreeMap<&'static str, u32>,
}

impl Vat {
    fn get(&self, chem: &str) -> u32 {
        self.store.get(chem).copied().unwrap_or(0)
    }

    fn increase(&mut self, chem: &'static str, amt: u32) {
        self.adjust(chem, amt as i32);
    }

    fn decrease(&mut self, chem: &'static str, amt: u32) {
        self.adjust(chem, -(amt as i32));
    }

    fn adjust(&mut self, chem: &'static str, amt: i32) {
        let current = self.get(chem);
        let next = (current as i32) + amt;

        if next == 0 {
            self.store.remove(chem);
        } else {
            self.store.insert(chem, next as u32);
        }
    }

    fn iter(&self) -> impl Iterator<Item = (&'static str, u32)> + '_ {
        self.store.iter().map(|(&k, &v)| (k, v))
    }
}

struct State {
    formulae: FormulaeBook,
    need: Vat,
    used: Vat,
    available: Vat,
}

impl State {
    fn new(formulae: FormulaeBook) -> Self {
        Self {
            formulae,
            need: Vat::default(),
            used: Vat::default(),
            available: Vat::default(),
        }
    }

    fn complete(&self) -> bool {
        self.need.store.is_empty()
    }

    fn produce(&mut self, name: &'static str, amt: u32) -> u32 {
        log::trace!("Trying to produce {} {}", amt, name);

        let (amt_per_cycle, reqs) = &self.formulae[name];

        let cycles_needed = div_round_up(amt, *amt_per_cycle);

        for _ in 0..cycles_needed {
            for &(req_qty, req_name) in reqs {
                let available = self.available.get(req_name);

                if available >= req_qty {
                    self.available.decrease(req_name, req_qty);
                } else {
                    self.need.increase(req_name, req_qty - available);
                }
            }
        }

        let amt_produced = amt_per_cycle * cycles_needed;

        log::trace!("Produced {} {}", amt_produced, name);

        self.available.increase(name, amt_produced);
        amt_produced
    }

    fn simplify(&mut self) -> bool {
        for (need_name, need_amt) in self.need.clone().iter() {
            if need_name != "ORE" {
                self.produce(need_name, need_amt);
                self.available.decrease(need_name, need_amt);
            }

            self.need.decrease(need_name, need_amt);
            self.used.increase(need_name, need_amt);
        }

        self.complete()
    }

    fn show(&self) {
        log::trace!("Need: {:?}", self.need);
        log::trace!("Used: {:?}", self.used);
        log::trace!("Available: {:?}", self.available);
    }
}

fn main() {
    env_logger::init();

    let formulae = parse_formulae(INPUT);

    let mut state = State::new(formulae);
    state.need.adjust("FUEL", 1);

    while !state.simplify() {
        state.show();
        log::trace!("");
    }

    println!("Part one: {}", state.used.get("ORE"));
}
