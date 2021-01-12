pub struct ByteGrid {
    data: Vec<u8>,
    length: usize,
}

#[allow(dead_code)]
impl ByteGrid {

    pub fn length(&self) 
    -> usize {
        self.length
    }
    pub fn new(length: usize) 
    -> Self {
        Self {
            data: vec![0 ; length.pow(3)],
            length,
        }
    }

    pub fn index(&self, coords: [usize ; 3]) 
    -> usize {
        coords[0] + 
        coords[1] * self.length +
        coords[2] * self.length.pow(2)
    }

    pub fn coords(index: usize, length: usize) 
    -> [usize ; 3] {
        [
            index % length,
            (index / length) % length,
            index / length.pow(2),
        ]
    }

    pub fn get(&self, coords: [usize ; 3]) 
    -> u8 {
        self.data[self.index(coords)]
    }

    pub fn set(&mut self, coords: [usize ; 3], value: u8) {
        let i = self.index(coords);
        self.data[i] = value
    }

    pub fn set_all(&mut self, value_fn : &dyn Fn([usize ; 3]) -> u8) {
        let length = self.length;
        self.data
        .iter_mut()
        .enumerate()
        .for_each(|(i, m)| *m = value_fn(Self::coords(i, length)));
    }

    pub fn full_slice(&self) 
    -> &[u8] {
        &self.data
    }
}

use nalgebra as na;
impl super::displaced_chunks::ChunkData for ByteGrid {
    fn allocate() -> Self {
        ByteGrid::new(32)
    }


    fn initialize(&mut self, world_chunk_coords: na::Vector3<i32>) {
        self.set_all(
            &(|coords| super::fill_voxel(coords, 0))
        );
    }
}