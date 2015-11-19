pub struct Grid<T> {
    pub width : u32,
    pub height : u32,
    cells : Vec<Option<T>>
}

impl <T> Grid <T> {
    pub fn new(width: u32, height: u32) -> Grid<T> {
        let mut vec = Vec::new();
        let size = width * height;
        for _ in 0..size {
            vec.push(None);
        }
        Grid {
            width: width,
            height: height,
            cells : vec
        }
    }

    pub fn get(&self, x : u32, y : u32) -> Option<&T> {
        let index = self.width * y + x;
        if (index as usize) < self.cells.len() {
            return self.cells[index as usize].as_ref();
        }
        None
    }

    pub fn get_mut(&mut self, x : u32, y : u32) -> Option<&mut T> {
        let index = self.width * y + x;
        if (index as usize) < self.cells.len() {
            return self.cells[index as usize].as_mut();
        }
        None
    }

    pub fn set(&mut self, x : u32, y: u32, element : T) {
        let index = self.width * y + x;
        if (index as usize) < self.cells.len() {
            self.cells[index as usize] = Some(element);
        }
    }

    pub fn fill<F>(&mut self, x : u32, y: u32, w: u32, h : u32, f : F)
        where F : Fn() -> T {
        for yy in y..(y + h) {
            for xx in x..(x + w) {
                self.set(xx, yy, f());
            }
        }
    }

}
