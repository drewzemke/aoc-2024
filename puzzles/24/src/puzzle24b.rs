use crate::Device;
use common::puzzle::PuzzlePart;
use itertools::Itertools;

pub struct Puzzle24b {}

// NOTE: I essentially solved this by hand. I wrote a function `tree_for_output` that
// computes the AST that computes a specific output, pretty-printed the outputs
// for each z__ wire, then figured out what wires needed to be swapped by inspection
// and by testing various inputs on the x__ and y__ wires.
impl PuzzlePart for Puzzle24b {
    fn description() -> &'static str {
        "Find the four pairs of gates whose outputs needs to be swapped to turn the device into an integer adder."
    }

    fn solve(input: &str) -> String {
        let mut device = Device::parse(input).unwrap();

        let mut pairs = [
            ["cdj", "z08"],
            ["z32", "gfm"],
            ["dhm", "qjd"],
            ["z16", "mrb"],
        ];
        for [left, right] in pairs {
            device.swap_outputs(left, right);
        }

        // NOTE: uncomment the two blocks below to print
        // the tree data I used to figure out which swaps were required

        // let mut z_wires = device
        //     .wires
        //     .iter()
        //     .filter(|w| w.name.starts_with('z'))
        //     .collect::<Vec<_>>();
        // z_wires.sort_by_cached_key(|w| w.name);

        // for w in z_wires {
        //     let output = w.name;
        //     let mut tree = device.tree_for_output(output);
        //     tree.sort_nodes();
        //     println!("{output} = {tree}")
        // }

        device.compute();

        let (x, y) = device.inputs();
        let z = device.output();

        assert_eq!(x + y, z);

        let all_strs = pairs.as_flattened_mut();
        all_strs.sort_unstable();
        all_strs.iter().join(",")
    }
}
