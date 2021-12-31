use itertools::iproduct;
use std::fmt;

#[derive(Debug, Clone, Copy)]
pub struct Octopus {
    energy: u8,
    flashes: u32,
    has_flashed: bool,
}

impl Octopus {
    fn new(initial_energy: u8) -> Octopus {
        Octopus {
            energy: initial_energy,
            flashes: 0,
            has_flashed: true,
        }
    }

    fn set_energy(&mut self, initial_energy: u8) {
        self.energy = initial_energy;
    }

    fn increase_energy(&mut self) {
        self.energy = self.energy + 1;
    }

    fn reset_flash(&mut self) {
        self.has_flashed = false;
    }

    fn flash(&mut self) {
        self.has_flashed = true;
    }

    fn has_flashed(&self) -> bool {
        self.has_flashed
    }

    fn should_flash(&self) -> bool {
        !self.has_flashed && self.energy > 9
    }
}
#[derive(Debug, Clone, Copy)]
pub struct OctopusGrid<const N: usize> {
    octopuses: [[Octopus; N]; N],
    pub(crate) flash_count: u32,
}

impl<const N: usize> OctopusGrid<N> {
    pub(crate) fn new(input: String) -> OctopusGrid<N> {
        let mut grid = OctopusGrid {
            octopuses: [[Octopus {
                energy: 0,
                flashes: 0,
                has_flashed: true,
            }; N]; N],
            flash_count: 0,
        };

        for (i, line) in input.lines().enumerate() {
            for (j, c) in line.chars().enumerate() {
                grid.set_energy_at(i, j, c.to_string().parse::<u8>().ok().unwrap());
            }
        }

        grid
    }

    pub(crate) fn step(&mut self) -> Option<u32> {
        // increase energy
        for i in 0..N {
            for j in 0..N {
                self.octopuses[i][j].reset_flash();
                self.octopuses[i][j].increase_energy();
            }
        }

        // check for flashes
        for i in 0..N {
            for j in 0..N {
                self.flash_if_needed(i, j);
            }
        }

        //
        let mut flash_count: u32 = 0;
        for i in 0..N {
            for j in 0..N {
                if self.octopuses[i][j].has_flashed() {
                    flash_count = flash_count + 1;
                    self.octopuses[i][j].set_energy(0);
                }
            }
        }

        Some(flash_count)
    }

    fn set_energy_at(&mut self, i: usize, j: usize, initial_energy: u8) {
        self.octopuses[i][j].set_energy(initial_energy)
    }

    fn flash_if_needed(&mut self, i: usize, j: usize) {
        // self.octopuses[i][j].set_energy(initial_energy)

        if self.octopuses[i][j].should_flash() {
            self.octopuses[i][j].flash();
            self.flash_count = self.flash_count + 1;
            OctopusGrid::<N>::neighbour_positions(i as i32, j as i32).for_each(|(x, y)| {
                self.octopuses[x as usize][y as usize].increase_energy();
                self.flash_if_needed(x as usize, y as usize);
            });
        }
    }

    ///
    ///
    fn neighbour_positions(i: i32, j: i32) -> impl Iterator<Item = (i32, i32)> {
        iproduct!(-1..=1, -1..=1)
            .filter(|&x| !(x.0 == 0 && x.1 == 0))
            .map(move |(x, y)| (i + x, j + y))
            .filter(|&x| x.0 >= 0 && x.0 < N as i32)
            .filter(|&x| x.1 >= 0 && x.1 < N as i32)
    }
}

impl<const N: usize> fmt::Display for OctopusGrid<N> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..N {
            for j in 0..N {
                write!(f, "{}", self.octopuses[i][j].energy);
            }
            write!(f, "\n");
        }

        Ok(())
    }
}

mod tests {
    use super::*;

    #[test]
    fn masks1() {
        let input = r#"5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526"#;

        let mut grid = OctopusGrid::<10>::new(input.to_string());

        for i in 0..100 {
            grid.step();
            println!("STEP 1\n {}", grid);
        }

        println!("FLASH \n {}", grid.flash_count);

        // .for_each(|inp| println!("{:?} | {:?}", inp.data, inp.digits));
    }
}
