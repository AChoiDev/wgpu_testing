use dot_vox as dv;
use dv::DotVoxData;
use std::mem;
pub struct DotVoxWrapper
{
    vox_data : DotVoxData
}

impl DotVoxWrapper
{

    pub fn new(file : &str)
        -> DotVoxWrapper
    {
        let mut vox_data = dv::load(file).unwrap();

        // change orientation (switches y and z)
        for model in &mut vox_data.models
        {
            for voxel in &mut model.voxels
            {
                mem::swap(&mut voxel.y, &mut voxel.z);
            }

            mem::swap(&mut model.size.y, &mut model.size.z);
        }
        

        DotVoxWrapper {vox_data}
    }
    pub fn get_voxel(&self, coords : [usize ; 3], model_index : usize)
        -> Option<&dv::Voxel>
    {
        let model = &self.vox_data.models[model_index];

        model.voxels.iter().find(|v| (v.x as usize, v.y as usize, v.z as usize) == (coords[0], coords[1], coords[2]))
    }

    pub fn get_voxel_color(&self, coords : [usize ; 3], model_index : usize)
        -> Option<u32>
    {
        match self.get_voxel(coords, model_index)
        {
            Some(voxel) => Some(self.vox_data.palette[voxel.i as usize]),
            _ => None
        }
    }

    pub fn palette(&self)
        -> [u32 ; 256]
    {
        debug_assert!(self.vox_data.palette.len() == 256);

        let mut array = [0 ; 256];
        array.copy_from_slice(&self.vox_data.palette.as_slice()[..256]);

        return array;
    }

    
    pub fn voxel_slice(&self, model_index : usize)
        -> &[dv::Voxel]
    {
        self.vox_data.models[model_index].voxels.as_slice()
    }

    pub fn dims(&self, model_index : usize)
        -> [usize ; 3]
    {
        let size = self.vox_data.models[model_index].size;

        [size.x  as usize, size.y as usize, size.z as usize]
    }
}