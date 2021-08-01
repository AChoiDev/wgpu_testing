use std::u16;

use crate::map_3D::Map3D;

use super::bit_voxels::BitVoxels;
use super::dot_vox_wrapper::DotVoxWrapper;

pub struct StandardVoxelPrefab
{
    dims : [usize ; 3],
    bit_voxels : BitVoxels,
    pub palette_volume : Map3D<u16>,
    pub palette : [u32 ; 256]
}


impl StandardVoxelPrefab
{
    pub fn new(vox_file_path : &str)
        -> StandardVoxelPrefab
    {
        let vox_data_wrap = DotVoxWrapper::new(vox_file_path);

        let bit_voxels = BitVoxels::new(&vox_data_wrap, 0);

        let dims = vox_data_wrap.dims(0);

        if dims[0] != 32 || dims[1] != 32 || dims[2] != 32
        {
            panic!("Must have appropriately dimensioned model!");
        }

        let palette_volume = 
        {
            let mut pal_vol = Map3D::new_with_default(32, u16::MAX);
            for voxel in vox_data_wrap.voxel_slice(0)
            {
                pal_vol.set([voxel.x as usize, voxel.y as usize, voxel.z as usize], (voxel.i) as u16);
                // pal_vol[index] = voxel.i as u16;
            }
            pal_vol
        };

        let palette = vox_data_wrap.palette();

        StandardVoxelPrefab {dims, bit_voxels, palette_volume, palette}
    }
}
