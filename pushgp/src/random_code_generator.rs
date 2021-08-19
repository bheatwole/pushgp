use crate::{Code, RandomType};
use rand::{prelude::SliceRandom, rngs::SmallRng, Rng, SeedableRng};
use rust_decimal::{prelude::FromPrimitive, Decimal};

#[derive(Debug, PartialEq)]
pub struct RandomCodeGenerator {
    rng: SmallRng,
    types: Vec<RandomType>,
    min_random_float: f64,
    max_random_float: f64,
    min_random_int: i64,
    max_random_int: i64,
    max_points_in_random_expressions: usize,
}

impl RandomCodeGenerator {
    pub fn new() -> RandomCodeGenerator {
        RandomCodeGenerator {
            rng: SmallRng::from_entropy(),
            types: vec![],
            min_random_float: -1.0,
            max_random_float: 1.0,
            min_random_int: std::i64::MIN,
            max_random_int: std::i64::MAX,
            max_points_in_random_expressions: 1000,
        }
    }

    pub fn set_seed(&mut self, seed: Option<u64>) {
        self.rng = if let Some(seed) = seed { SmallRng::seed_from_u64(seed) } else { SmallRng::from_entropy() }
    }

    pub fn are_types_defined(&self) -> bool {
        self.types.len() > 0
    }

    pub fn clear_types(&mut self) {
        self.types.clear();
    }

    pub fn append_type(&mut self, rand_type: RandomType, weight: u8) {
        for _ in 0..weight {
            self.types.push(rand_type);
        }
    }

    pub fn generate(&mut self) -> Code {
        let actual_points = self.rng.gen_range(1..=self.max_points_in_random_expressions);
        self.random_code_with_size(actual_points)
    }

    pub fn random_atom_of_type(&mut self, atom_type: RandomType) -> Code {
        match atom_type {
            RandomType::EphemeralBool => Code::LiteralBool(if 0 == self.rng.gen_range(0..=1) { false } else { true }),
            RandomType::EphemeralFloat => {
                let float: f64 = self.rng.gen_range(self.min_random_float..self.max_random_float);
                Code::LiteralFloat(Decimal::from_f64(float).unwrap())
            }
            RandomType::EphemeralInt => {
                Code::LiteralInteger(self.rng.gen_range(self.min_random_int..=self.max_random_int))
            }
            RandomType::EphemeralName => Code::LiteralName(self.rng.gen_range(0..=u64::MAX)),
            RandomType::DefinedName(name) => Code::LiteralName(name),
            RandomType::Instruction(inst) => Code::Instruction(inst),
        }
    }

    fn random_code_with_size(&mut self, points: usize) -> Code {
        if 1 == points {
            let index = self.rng.gen_range(0..self.types.len());
            self.random_atom_of_type(*self.types.get(index).unwrap())
        } else {
            let mut sizes_this_level = self.decompose(points - 1, points - 1);
            sizes_this_level.shuffle(&mut self.rng);
            let mut list = vec![];
            for size in sizes_this_level {
                list.push(self.random_code_with_size(size));
            }
            Code::List(list)
        }
    }

    fn decompose(&mut self, number: usize, max_parts: usize) -> Vec<usize> {
        if 1 == number || 1 == max_parts {
            return vec![1];
        }
        let this_part = self.rng.gen_range(1..=(number - 1));
        let mut result = vec![this_part];
        result.extend_from_slice(&self.decompose(number - this_part, max_parts - 1));
        result
    }
}
