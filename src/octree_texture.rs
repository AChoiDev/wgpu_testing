use std::u16;

use super::map_3D::Map3D;
pub struct OctreeTexture {
    data: Map3D<Octant>,
    magnitude: usize,
    count: usize,
}

pub const NULL_CHILD: u16 = u16::MAX;

#[derive(Copy, Clone)]
pub struct Octant(u16);
impl Default for Octant {
    fn default() -> Self { Octant(NULL_CHILD) }
}

pub const OCTUPLE_DATA_MAP_SIZE: usize = 18;

impl OctreeTexture {
    pub fn new(magnitude: usize)
    -> Self {
        Self {
            data: Map3D::new(OCTUPLE_DATA_MAP_SIZE),
            magnitude,
            count: 1,
        }
    }

    pub fn new_from_map(map: &Map3D<u8>, magnitude: usize) 
    -> Self {
        let mut oct_text = Self::new(magnitude);

        for x in 0..map.length() {
        for y in 0..map.length() {
        for z in 0..map.length() {
            let coords = [x, y, z];
            if map.get(coords) != 255 {
                oct_text.insert(coords);
            }
        }
        }
        }

        oct_text
    }

    // pub fn initialize(map: &Map3D<u8>, magnitude: usize)
    // -> Self {
        //count
    // }
    
    pub fn full_slice(&self) 
    -> &[Octant] {
        self.data.full_slice()
    }

    pub fn u16_format_to_usize_coords(format: u16) 
    -> [usize ; 3] {
        let f_usize = format as usize;
        let five_bit_mask = (1 << 5) - 1;
        [
            f_usize & five_bit_mask,
            (f_usize >> 5) & five_bit_mask,
            (f_usize >> 10) & five_bit_mask,
        ]
    }
    fn d_coords(&self, octuple_coords: [usize; 3], child_index: usize) 
    -> [usize ; 3] {
        [
            (octuple_coords[0] << 1) | ((child_index >> 0) & 1),
            (octuple_coords[1] << 1) | ((child_index >> 1) & 1),
            (octuple_coords[2] << 1) | ((child_index >> 2) & 1),
        ]
    }

    fn d_get_child(&self, octuple_coords: [usize; 3], child_index: usize) 
    -> u16 {
        self.data.get(self.d_coords(octuple_coords, child_index)).0
    }

    fn d_set_child(&mut self, octuple_coords: [usize; 3], child_index: usize, child: u16) {
        self.data.set(self.d_coords(octuple_coords, child_index), Octant(child));
    }

    fn d_is_child_null(&self, octuple_coords: [usize; 3], child_index: usize) 
    -> bool {
        self.d_get_child(octuple_coords, child_index) == NULL_CHILD
    }

    fn d_get_child_usize(&self, octuple_coords: [usize; 3], child_index: usize) 
    -> [usize ; 3] {
        Self::u16_format_to_usize_coords(self.d_get_child(octuple_coords, child_index))
    }

    

    fn d_set_child_volume(&mut self, octuple_coords: [usize; 3], child_index: usize, sub_child_index: usize) {
        let mut base_child = self.d_get_child(octuple_coords, child_index);
        if base_child == NULL_CHILD {
            base_child = 0;
        }
        base_child |= 1 << sub_child_index;

        self.d_set_child(octuple_coords, child_index, base_child);
    }

    
    fn d_get_child_volume(&self, octuple_coords: [usize; 3], child_index: usize, sub_child_index: usize) 
    -> bool {
        let child = self.d_get_child(octuple_coords, child_index);
        if child == NULL_CHILD {
            return false;
        }

        ((child >> sub_child_index) & 1) == 1
    }

    // returns location of octuple in u16 format
    fn d_allocate_octuple(&mut self) 
    -> u16 {
        let s = OCTUPLE_DATA_MAP_SIZE / 2;
        let new_location = [
            (self.count % s),
            ((self.count / s) % s),
            (self.count / (s * s)),
        ];

        self.count += 1;

        (new_location[0] | 
        (new_location[1] << 5) |
        (new_location[2] << 10)) as u16
    }

    pub fn insert(&mut self, coords: [usize ; 3]) {
        let mut octuple_coords = [0usize ; 3];

        for depth in (2usize..self.magnitude).rev() {
            let child_index = Self::infer_child_index(coords, depth);

            if self.d_is_child_null(octuple_coords, child_index) {
                let new_location = self.d_allocate_octuple();
                self.d_set_child(octuple_coords, child_index, new_location);
            }

            octuple_coords = self.d_get_child_usize(octuple_coords, child_index);
        }

        let child_index = Self::infer_child_index(coords, 1);
        let sub_child_index = Self::infer_child_index(coords, 0);

        self.d_set_child_volume(octuple_coords, child_index, sub_child_index);
    }

    pub fn get(&self, coords: [usize ; 3]) 
    -> bool {
        let mut octuple_coords = [0usize ; 3];

        for depth in (2usize..self.magnitude).rev() {
            let child_index = Self::infer_child_index(coords, depth);
            if self.d_is_child_null(octuple_coords, child_index) {
                return false;
            }

            octuple_coords = self.d_get_child_usize(octuple_coords, child_index);
        }

        let child_index = Self::infer_child_index(coords, 1);
        let sub_child_index = Self::infer_child_index(coords, 0);

        self.d_get_child_volume(octuple_coords, child_index, sub_child_index)
    }

    pub fn total_nodes(&self)
    -> usize {self.count}

    // takes the nth bit of each coordinate
    // and maps each bit to a bit of a new integer
    pub fn infer_child_index(coords: [usize ; 3], depth: usize) 
    -> usize {
        ((coords[0] >> depth) & 0b001) | 
        (((coords[1] >> depth) << 1) & 0b010) | 
        (((coords[2] >> depth) << 2) & 0b100)
    }
}


unsafe impl bytemuck::Pod for Octant {

}
unsafe impl bytemuck::Zeroable for Octant {
    
}