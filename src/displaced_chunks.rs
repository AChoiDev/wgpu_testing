use std::collections::HashSet;

use nalgebra as na;

type VectorInt = na::Vector3<i32>;

struct Chunk<T: ChunkData> {
    data: T,
    displacement: VectorInt,
    initialized: bool,
}


// Displaced Chunks are chunks of map data
// based on their integer displacement from the chunk the viewer resides in.
// All chunks are accessed via a unique chunk index.
    // All chunks are assigned a displacement from the viewer's current chunk,
    // e.g., the chunk directly above the player's chunk is (0, 1, 0)
pub struct DisplacedChunks<T : ChunkData>
{
    chunks : Vec<Chunk<T>>,

    // The set of all possible chunk displacements
    displacement_set : HashSet<VectorInt>,
}


impl<T : ChunkData> DisplacedChunks<T>
{
    pub fn new(displacement_set : HashSet<na::Vector3<i32>>)
        -> DisplacedChunks<T>
    {
        let num = displacement_set.len();

        let chunks = 
            displacement_set
            .iter()
            .map(|&displacement| Chunk { data: T::allocate(), displacement, initialized: false})
            .collect();

        DisplacedChunks 
        {
            chunks,
            displacement_set,
        }
    }
    


    pub fn len(&self)
        -> usize
    {
        self.chunks.len()
    }

    fn closest_uninitialized_chunk_index(&self)
        -> Option<usize>
    {
        (0..self.len())
        .filter(|&index| 
            !self.chunks[index].initialized)
        .min_by_key(|&index| 
            Self::mag_squared(self.chunks[index].displacement))
    }
    
    fn try_initialize(&mut self, world_chunk_coords: VectorInt) {
        if let Some(index) = self.closest_uninitialized_chunk_index() {
            let chunk = &mut self.chunks[index];
            chunk.data.initialize(world_chunk_coords + chunk.displacement);
        }
    }

    // gets the squared magnitude of an i32 na vector
    fn mag_squared(disp: VectorInt)
        -> i32
    {
        disp.iter().fold(0, |acc, i| acc + i * i)
    }

    // All chunks are shifted and assigned new displacements.
    // If this new displacement is NOT within the set,
    // then it will be flagged as unused and given a different, unoccupied displacement

    // Invalid chunk indices will be returned
    pub fn displace(&mut self, displacement : na::Vector3<i32>)
        -> Vec<usize>
    {
        let mut occupied_displacement_set = HashSet::new();
        let mut invalid_indices = Vec::new();

        // The chunks are displaced in this loop
        // Invalid displacements are recorded for the next loop
        for index in 0..self.len()
        {
            self.chunks[index].displacement += displacement;

            occupied_displacement_set.insert(self.chunks[index].displacement.clone());

            // new displacement is not part of the set
            // the index is now invalid
            if !self.displacement_set.contains(&self.chunks[index].displacement)
            {
                invalid_indices.push(index);
            }
        }

        let mut unoccupied_displacement_iter =
            self.displacement_set.difference(&occupied_displacement_set).cloned();

        // Invalid chunk updating (flagging, new valid displacement)
        for &invalid_index in &invalid_indices
        {
            self.chunks[invalid_index].initialized = false;
            
            self.chunks[invalid_index].displacement = unoccupied_displacement_iter.next().unwrap();
        }

        invalid_indices
    }


    pub fn get_displacement(&self, index : usize)
        -> na::Vector3<i32>
    {
        self.chunks[index].displacement
    }
}

pub trait ChunkData {
    fn initialize(&mut self, world_chunk_coord: VectorInt);
    fn allocate() -> Self;
}



pub fn radius_displacement_set(view_radius : usize)
        -> HashSet<na::Vector3<i32>>
    {
        let mut displacement_set = HashSet::new();

        let r = view_radius as i32;
        let axis_iter = (1 - r)..r;

        let view_radius = (view_radius - 1) as f32;

        for x in axis_iter.clone() {
        for y in axis_iter.clone() {
        for z in axis_iter.clone() {

            let displacement = na::Vector3::new(x, y, z);

            if displacement_in_length(displacement, view_radius)
            {
                displacement_set.insert(displacement);
            }

        }}}

        displacement_set
    }


    fn displacement_in_length(displacement : na::Vector3<i32>, length : f32)
        -> bool
    {
        // The magnitude of the view coordinate is compared with the radius
        displacement.iter()
        .fold(0f32, |acc, c| acc + (c * c) as f32)
        .sqrt() 
            <= length
    }