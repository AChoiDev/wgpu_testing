use super::bit_voxels::BitVoxels;
use super::dot_vox_wrapper::DotVoxWrapper;
use nalgebra as na;

pub struct StandardVoxelPrefab
{
    dims : [usize ; 3],
    bit_voxels : BitVoxels,
    palette_volume : Vec<u8>,
    palette : [u32 ; 256]
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
            let mut pal_vol = vec![0 ; dims[0] * dims[1] * dims[2]];
            for voxel in vox_data_wrap.voxel_slice(0)
            {
                let index = 
                    voxel.x as usize 
                    + voxel.y as usize * dims[0] 
                    + voxel.z as usize * dims[0] * dims[1];
                pal_vol[index] = voxel.i;
            }
            pal_vol
        };

        let palette = vox_data_wrap.palette();

        StandardVoxelPrefab {dims, bit_voxels, palette_volume, palette}
    }
}
