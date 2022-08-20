use bracket_lib::prelude::to_cp437;
use bracket_lib::prelude::BTerm;
use bracket_lib::prelude::BLACK;
use bracket_lib::prelude::YELLOW;
pub struct Player {
    pub x: i32,
    pub y: i32,
    velocity: f32,
}

impl Player {
    pub fn new(x: i32, y: i32) -> Self {
        Player {
            x,
            y,
            velocity: 0.0,
        }
    }

    pub fn render(&mut self, ctx: &mut BTerm) {
        ctx.set(0, self.y, YELLOW, BLACK, to_cp437('@'));
    }

    pub fn gravity_and_move(&mut self) {
        self.velocity += 0.2;

        // terminal velocity is +2.0
        if self.velocity > 2.0 {
            self.velocity = 2.0;
        }

        self.y += self.velocity as i32;
        self.x += 1;

        // Never go in reverse.
        if self.y < 0 {
            self.y = 0;
        }
    }

    pub fn flap(&mut self) {
        self.velocity = -2.0;
    }
}
