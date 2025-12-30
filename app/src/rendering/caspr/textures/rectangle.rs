#[derive(Clone)]
pub struct Rectangle<T> {
    pub width: u32,
    pub height: u32,

    pub data: Vec<T>,
}

impl<T> Rectangle<T> {
    pub fn new(width: u32, height: u32, data: Vec<T>) -> Self {
        Self { width, height, data }
    }

    pub fn empty() -> Self {
        Self {
            width: 0,
            height: 0,
            data: Vec::new(),
        }
    }
}

impl Rectangle<u8> {
    pub fn mip_map_count(&self) -> u32 {
        Self::mip_map_count_inner(self.width, self.height, 0)
    }

    fn mip_map_count_inner(width: u32, height: u32, acc: u32) -> u32 {
        if width.min(height) <= 1 || width % 2 != 0 || height % 2 != 0 {
            return acc + 1;
        }
        Self::mip_map_count_inner(width / 2, height / 2, acc + 1)
    }

    pub fn generate_mipmaps(self) -> Vec<Self> {
        let mut acc = Vec::new();
        self.generate_mipmaps_inner(&mut acc);
        acc
    }

    fn generate_mipmaps_inner(self, acc: &mut Vec<Self>) {
        if self.width.min(self.height) <= 1 || self.width % 2 != 0 || self.height % 2 != 0 {
            acc.push(self);
            return;
        }

        let next_width = self.width / 2;
        let next_height = self.height / 2;
        let mut next_data = Vec::with_capacity((next_width * next_height) as usize);

        for y in 0..next_height {
            for x in 0..next_width {
                let row_top = 2 * y;
                let row_bot = 2 * y + 1;
                let col_left = 2 * x;
                let col_right = 2 * x + 1;

                let i_tl = (row_top * self.width + col_left) as usize;
                let i_tr = (row_top * self.width + col_right) as usize;
                let i_bl = (row_bot * self.width + col_left) as usize;
                let i_br = (row_bot * self.width + col_right) as usize;

                let sum = self.data[i_tl] as u16 + self.data[i_tr] as u16 + self.data[i_bl] as u16 + self.data[i_br] as u16;
                // Round to nearest integer, not down
                let avg = (sum + 2) / 4;

                next_data.push(avg as u8);
            }
        }

        acc.push(self);

        let next_rectangle = Self::new(next_width, next_height, next_data);
        next_rectangle.generate_mipmaps_inner(acc);
    }
}
