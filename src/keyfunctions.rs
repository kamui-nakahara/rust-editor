use crate::Data;

impl Data {
    pub fn move_down(&mut self) {
        if self.is_dir {
            if self.y as usize + self.top < self.listdir.len() {
                if self.y < self.height - 5 {
                    self.y += 1;
                } else {
                    self.top += 1;
                }
            }
        } else {
            if self.y + 1 < self.height - 1 && self.y < (self.content.len() - self.top) as u16 {
                self.y += 1;
            } else {
                if self.content.len() >= self.top + self.height as usize - 1 {
                    self.top += 1;
                }
            }
            let length = self.content[self.y as usize - 1 + self.top].len() as u16
                + (self.mode == 'i') as u16;
            if self.x_max || length < self.x {
                self.x = length;
            }
        }
    }

    pub fn move_up(&mut self) {
        if self.y > 1 {
            self.y -= 1
        } else {
            if self.top > 0 {
                self.top -= 1
            }
        }
        if !self.is_dir {
            let length = self.content[self.y as usize - 1 + self.top].len() as u16
                + (self.mode == 'i') as u16;
            if self.x_max || length < self.x {
                self.x = length;
            }
        }
    }

    pub fn move_left(&mut self) {
        if self.x > 1 {
            self.x -= 1;
            self.x_max = false;
        }
    }

    pub fn move_right(&mut self) {
        let length =
            self.content[self.y as usize - 1 + self.top].len() as u16 + (self.mode == 'i') as u16;
        if length < self.x + 1 {
            self.x = length;
        } else {
            if self.x < self.width {
                self.x += 1;
                self.x_max = false;
            }
        }
    }
}
