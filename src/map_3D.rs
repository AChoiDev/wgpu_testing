pub struct Map3D<T: Clone + Default + Copy> {
    data: Vec<T>,
    length: usize,
}

#[allow(dead_code)]
impl<T: Clone + Default + Copy> Map3D<T> {

    pub fn length(&self) 
    -> usize {
        self.length
    }
    pub fn new(length: usize) 
    -> Self {
        Self {
            data: vec![T::default() ; length.pow(3)],
            length,
        }
    }

    pub fn new_with_default(length: usize, default_val: T) 
    -> Self {
        Self {
            data: vec![default_val ; length.pow(3)],
            length
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
    -> T {
        self.data[self.index(coords)]
    }

    pub fn set(&mut self, coords: [usize ; 3], value: T) {
        let i = self.index(coords);
        self.data[i] = value
    }

    pub fn set_all(&mut self, value_fn : &dyn Fn([usize ; 3]) -> T) {
        let length = self.length;
        self.data
        .iter_mut()
        .enumerate()
        .for_each(|(i, m)| *m = value_fn(Self::coords(i, length)));
    }

    pub fn full_slice(&self) 
    -> &[T] {
        &self.data
    }
}


pub struct GenerateContext {
    pub open_simplex: noise::OpenSimplex,
}
use nalgebra as na;
impl super::displaced_chunks::ChunkData for Map3D<u16> {
    fn allocate() -> Self {
        Map3D::new(32)
    }

    fn initialize(&mut self, world_chunk_coords: na::Vector3<i32>) {
        let open_simplex = noise::OpenSimplex::new();
        let generate_context = GenerateContext {
            open_simplex
        };
        // println!("{}", min);
        self.set_all(
            &(|coords| super::fill_voxel(coords, world_chunk_coords, &generate_context))
        );
    }
}