use intcode::{Outcome, VM};

mod intcode;

static INPUT: &str = include_str!("../input.txt");

fn get_thruster_signal(memory: &[i32], phases: &[u8]) -> i32 {
    let mut output = 0;

    for &phase in phases {
        let mut vm = VM::default();
        vm.load_memory(memory);
        vm.put_input(phase as i32);
        vm.put_input(output);
        vm.run_until_terminated();

        output = vm.get_output();
    }

    output
}

fn get_thruster_signal_with_feedback(memory: &[i32], phases: &[u8]) -> i32 {
    let mut vms: Vec<_> = phases
        .iter()
        .map(|&phase| {
            let mut vm = VM::default();
            vm.load_memory(memory);
            vm.put_input(phase as i32);
            vm
        })
        .collect();

    let num_vms = vms.len();

    let mut vm_index = 0;
    let mut final_vm_output = 0;

    let next_vm_index = |index| (index + 1) % num_vms;
    let is_final_vm = |index| index == num_vms - 1;

    vms[0].put_input(0);

    loop {
        let vm = &mut vms[vm_index];

        match vm.run_partial() {
            Outcome::Terminated => {
                if is_final_vm(vm_index) {
                    return final_vm_output;
                }
            }
            Outcome::NeedsInput => panic!("VM starved"),
            Outcome::SentOutput => {
                let output = vm.get_output();

                if is_final_vm(vm_index) {
                    final_vm_output = output;
                }

                let next_vm = &mut vms[next_vm_index(vm_index)];
                next_vm.put_input(output);
            }
        }

        vm_index = next_vm_index(vm_index);
    }
}

fn permutations(values: &[u8]) -> Vec<Vec<u8>> {
    let mut perms = Vec::new();
    permutations_inner(values, &mut |next: Vec<u8>| {
        perms.push(next);
    });
    perms
}

fn permutations_inner(values: &[u8], callback: &mut dyn FnMut(Vec<u8>)) {
    match values.len() {
        0 => {}
        1 => callback(values.to_vec()),
        _ => {
            for i in 0..values.len() {
                let value = values[i];
                let others = &[&values[..i], &values[i + 1..]].concat();

                permutations_inner(&others, &mut |mut buf: Vec<u8>| {
                    buf.push(value);
                    callback(buf);
                });
            }
        }
    }
}

fn part_one() {
    let program = VM::decode_tape(INPUT);

    let best = permutations(&[0, 1, 2, 3, 4])
        .into_iter()
        .map(|phases| {
            let signal = get_thruster_signal(&program, &phases);

            (phases, signal)
        })
        .max_by_key(|(_phases, signal)| *signal)
        .unwrap();

    println!("Part one: {} (permutations {:?})", best.1, best.0);
}

fn part_two() {
    let program = VM::decode_tape(INPUT);

    let best = permutations(&[5, 6, 7, 8, 9])
        .into_iter()
        .map(|phases| {
            let signal = get_thruster_signal_with_feedback(&program, &phases);

            (phases, signal)
        })
        .max_by_key(|(_phases, signal)| *signal)
        .unwrap();

    println!("Part two: {} (permutations {:?})", best.1, best.0);
}

fn main() {
    part_one();
    part_two();
}
