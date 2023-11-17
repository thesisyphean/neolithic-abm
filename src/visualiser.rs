use image::{ImageBuffer, Rgb, RgbImage};

// Needed variables -
// - Number of agents
// - Number of resource patches
// From there we can calculate the number of settlements

// Tracked variables -
// - Settlement positions
// - Resource patch positions

pub struct Visualiser {
    frame: RgbImage,
    i: u32,
    settlements: Vec<(u32, u32)>,
    patches: Vec<(u32, u32)>,
}

impl Visualiser {
    const EMPTY: Rgb<u8> = Rgb([255, 255, 255]);
    const SETTLEMENT: Rgb<u8> = Rgb([0, 0, 0]);
    const CLAIMED: Rgb<u8> = Rgb([128, 128, 128]);

    pub fn new() -> Self {
        Visualiser {
            frame: ImageBuffer::new(500, 500),
            i: 0,
            settlements: vec![],
            patches: vec![],
        }
    }

    pub fn initialise(&mut self) {
        for pixel in self.frame.pixels_mut() {
            *pixel = Self::EMPTY;
        }
    }

    fn paint_pixel(&mut self, pos: (u32, u32), pixel: Rgb<u8>) {
        for x in 0..10 {
            for y in 0..10 {
                self.frame.put_pixel(pos.0 * 10 + x, pos.1 * 10 + y, pixel);
            }
        }
    }

    pub fn update_agents(&mut self, num_agents: u32) {
        let num_settlements = num_agents / 75;

        let current = self.settlements.len() as u32;
        if num_settlements < current {
            for n in 0..(current - num_settlements) {
                let index = rand::random::<u32>() % (current - n);
                let pos = self.settlements.swap_remove(index as usize);
                self.paint_pixel(pos, Self::EMPTY);
            }
        } else if num_settlements > current {
            let difference = num_settlements - current;
            let mut new_settlements = 0;
            let mut count = 0;

            while difference > new_settlements && count < 10_000 {
                let index = rand::random::<u32>() % 2500;
                let pos = Self::index_to_pos(index);

                // TODO: !!!
                if self.frame[pos] == Self::EMPTY || self.frame[pos] == Self::CLAIMED {
                    self.paint_pixel(pos, Self::SETTLEMENT);
                    self.settlements.push(pos);
                    new_settlements += 1;
                }

                count += 1;
            }
        }
    }

    pub fn update_patches(&mut self, num_patches: u32) {
        let current = self.patches.len() as u32;

        if num_patches < current {
            for n in 0..(current - num_patches) {
                let index = rand::random::<u32>() % (current - n);
                let pos = self.patches.swap_remove(index as usize);
                self.paint_pixel(pos, Self::EMPTY);
            }
        } else if num_patches > current {
            let difference = num_patches - current;
            let mut new_patches = 0;
            let mut count = 0;

            while difference > new_patches && count < 10_000 {
                let index = rand::random::<u32>() % 2500;
                let pos = Self::index_to_pos(index);

                // TODO: !!!
                if self.frame[pos] == Self::EMPTY {
                    self.paint_pixel(pos, Self::CLAIMED);
                    self.patches.push(pos);
                    new_patches += 1;
                }

                count += 1;
            }
        }
    }

    pub fn save(&mut self) {
        self.frame
            .save(format!("frames/frame{:0>4}.png", self.i))
            .unwrap();

        self.i += 1;
    }

    fn index_to_pos(index: u32) -> (u32, u32) {
        (index % 50, index / 50)
    }
}
