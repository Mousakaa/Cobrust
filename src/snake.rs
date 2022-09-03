use rand::random;

use crate::snake;

pub enum GameMode {
    Play,
    Pause,
    Lost
}

pub enum Direction {
    Left,
    Down,
    Up,
    Right
}

pub struct Snake {
    pub coords: Vec<(f64, f64)>,
    pub apple_coords: (f64, f64),
    pub game_mode: GameMode,
    pub direction: Direction,
    pub score: usize
}

impl Snake {
    pub fn new(width: u16, height: u16) -> Snake {
        Snake {
            coords: (vec![
                ((width / 2) as f64, (height / 2) as f64),
                ((width / 2) as f64, (height / 2 - 1) as f64),
                ((width / 2) as f64, (height / 2 - 2) as f64),
                ((width / 2) as f64, (height / 2 - 3) as f64),
                ((width / 2) as f64, (height / 2 - 4) as f64),
                ((width / 2) as f64, (height / 2 - 5) as f64)
            ]),
            apple_coords: (
                (random::<u16>() % (width - 2) + 1) as f64,
                (random::<u16>() % (height - 2) + 1) as f64
            ),
            game_mode: snake::GameMode::Play,
            direction: snake::Direction::Up,
            score: 0
        }
    }

    pub fn forward(&mut self, width: &u16, height: &u16) {
        let mut head = match self.direction {
            snake::Direction::Left => (self.coords[0].0 - 2.0, self.coords[0].1),
            snake::Direction::Down => (self.coords[0].0, self.coords[0].1 - 2.0),
            snake::Direction::Up => (self.coords[0].0, self.coords[0].1 + 2.0),
            snake::Direction::Right => (self.coords[0].0 + 2.0, self.coords[0].1)
        };

        let mut neck = match self.direction {
            snake::Direction::Left => (self.coords[0].0 - 1.0, self.coords[0].1),
            snake::Direction::Down => (self.coords[0].0, self.coords[0].1 - 1.0),
            snake::Direction::Up => (self.coords[0].0, self.coords[0].1 + 1.0),
            snake::Direction::Right => (self.coords[0].0 + 1.0, self.coords[0].1)
        };

        (head, neck) = match (head, neck) {
            ((-1.0, y), _) | (_, (-1.0, y)) => (((*width - 1) as f64, y), (*width as f64, y)),
            ((xh, y), (xn, _)) if xh == (*width + 1) as f64 || xn == (*width + 1) as f64 => ((1.0, y), (0.0, y)),
            ((x, -1.0), _) | (_, (x, -1.0)) => ((x, (*height - 1) as f64), (x, *height as f64)),
            ((x, yh), (_, yn)) if yh == (*height + 1) as f64 || yn == (*height + 1) as f64 => ((x, 1.0), (x, 0.0)),
            t => t
        };

        if self.coords.contains(&head) || self.coords.contains(&neck) { //If snake hits itself
            self.game_mode = snake::GameMode::Lost;
        }

        else if is_around(&head, &self.apple_coords) { //If snake eats apple
            self.coords.push((-1.0, -1.0));
            self.coords.push((-1.0, -1.0));
            self.coords.push((-1.0, -1.0));
            self.coords.push((-1.0, -1.0));
            self.coords.push((-1.0, -1.0));
            self.coords.push((-1.0, -1.0));

            let mut tmp1 = (self.coords[0], self.coords[1]);
            let mut tmp2: ((f64, f64), (f64, f64));

            for i in 1..(self.coords.len() / 2) {
                tmp2 = (self.coords[2 * i], self.coords[2 * i + 1]);
                (self.coords[2 * i], self.coords[2 * i + 1]) = tmp1;
                tmp1 = tmp2;
            }

            self.coords[0] = head;
            self.coords[1] = neck;

            //Move apple
            
            self.apple_coords = (
                (random::<u16>() % (width - 2) + 1) as f64,
                (random::<u16>() % (height - 2) + 1) as f64
            );

            self.score = self.score + 10;
        }

        else { //Just go forward
            let mut tmp1 = (self.coords[0], self.coords[1]);
            let mut tmp2: ((f64, f64), (f64, f64));

            for i in 1..(self.coords.len() / 2) {
                tmp2 = (self.coords[2 * i], self.coords[2 * i + 1]);
                (self.coords[2 * i], self.coords[2 * i + 1]) = tmp1;
                tmp1 = tmp2;
            }

            self.coords[0] = head;
            self.coords[1] = neck;
        }
    }
}

fn is_around(seeker: &(f64, f64), target: &(f64, f64)) -> bool {
    if seeker.0 == target.0 - 1.0 || seeker.0 == target.0 || seeker.0 == target.0 + 1.0 {
        if seeker.1 == target.1 - 1.0 || seeker.1 == target.1 || seeker.1 == target.1 + 1.0 {
            return true;
        }
    }
    return false;
}
