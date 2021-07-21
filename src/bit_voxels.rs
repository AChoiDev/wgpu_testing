#[derive(Clone)]
pub struct BitVoxels
{
    pub dims : [usize ; 3],
    pub data : Vec<u8>
}

use std::ops::{BitAnd, BitAndAssign, BitOrAssign};
// use dot_vox as dv;
use super::dot_vox_wrapper::DotVoxWrapper;


// Data dims represents the extent to which a coordinate can be converted to an index for the data vec
// data_dims = (5, 6, 3) => max_coords = (4, 5, 2)
impl BitVoxels
{
    pub fn new(vox_data : &DotVoxWrapper, model_index : usize)
        -> BitVoxels
    {
        let dims = vox_data.dims(model_index);
        let data_dims = [(dims[0] + 1) / 2, (dims[1] + 1) / 2, (dims[2] + 1) / 2];
        
        let data = vec![0 ; data_dims[0] * data_dims[1] * data_dims[2]];

        let mut b_voxels = BitVoxels {dims, data};


        for vox in vox_data.voxel_slice(model_index)
        {
            b_voxels.set_voxel([vox.x as usize, vox.y as usize, vox.z as usize], true);
        }

        b_voxels
    }

    // Changes an individual bit from the u8
    // the bit represents whether a voxel is present or not
    pub fn set_voxel(&mut self, coords : [usize ; 3], existence : bool)
    {
        let byte_octo_voxel = &mut self.data[BitVoxels::index_from_coords(coords, self.dims)];

        let nth_bit = BitVoxels::bit_pos_from_coords(coords);

        byte_octo_voxel
            .bitand_assign(!(1 << nth_bit)); // clears bit
        byte_octo_voxel
            .bitor_assign((existence as u8) << nth_bit); // sets bit
    }


    // Reads the individual bit that represents whether a voxel is present or not
    pub fn get_voxel(&self, coords : [usize ; 3])
        -> bool
    {
        self.data[BitVoxels::index_from_coords(coords, self.dims)].clone()
            .bitand(1 << BitVoxels::bit_pos_from_coords(coords)) != 0
    }

    pub fn dims(&self)
        -> [usize ; 3]
    {
        self.dims
    }


    fn index_from_coords(coords : [usize ; 3], dims : [usize ; 3])
        -> usize
    {
        let data_dims = [(dims[0] + 1) / 2, (dims[1] + 1) / 2];

        (coords[0] / 2)
        + (coords[1] / 2) * data_dims[0]
        + (coords[2] / 2) * data_dims[0] * data_dims[1]
    }

    // position of a bit in a byte based on boolean coords
    fn bit_pos_from_bool_coords(coords : [bool ; 3])
        -> usize
    {
        (coords[0] as usize) + (coords[1] as usize) * 2 + (coords[2] as usize) * 4
    }
    fn bit_pos_from_coords(coords : [usize ; 3])
        -> usize
    {
        BitVoxels::bit_pos_from_bool_coords(
            [(coords[0] % 2) != 0, (coords[1] % 2) != 0, (coords[2] % 2) != 0])
    }
}
