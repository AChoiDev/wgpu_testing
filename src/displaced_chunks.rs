const VIEW_HEIGHT: usize = 5;
const VIEW_DEPTH: usize = 15;

pub fn radius_displacement_set()
        -> HashSet<na::Vector3<i32>>
{
    let mut displacement_set = HashSet::new();

    let cd = VIEW_DEPTH as i32 + 2;
    let horizontal_iter = (1 - cd)..cd;
    let ch = VIEW_HEIGHT as i32 + 2;
    let vertical_iter = (1 - ch)..ch;

    for x in horizontal_iter.clone() {
    for y in vertical_iter.clone() {
    for z in horizontal_iter.clone() {

        let displacement = na::Vector3::new(x, y, z);

        if displacement_valid(displacement)
        {
            displacement_set.insert(displacement);
        }

    }}}

    println!("possible displacement total: {}", displacement_set.len());

    displacement_set
}


fn displacement_valid(displacement : na::Vector3<i32>)
    -> bool
{
    let ratios = [
        displacement.x as f32 / VIEW_DEPTH as f32, 
        displacement.y as f32 / VIEW_HEIGHT as f32,
        displacement.z as f32 / VIEW_DEPTH as f32
    ];

    ratios.iter().fold(0f32, |acc, r| acc + r * r) <= 1f32
}

use std::collections::HashSet;

use nalgebra as na;

use crate::render::resources::ChunkIDVariant;

type VectorInt = na::Vector3<i32>;

struct Chunk<T: ChunkData> {
    data: T,
    partition_coords: VectorInt,
    initialized: bool,
    dirty: bool,
}

pub struct DisplacedChunks<T : ChunkData>
{
    chunks : Vec<Chunk<T>>,
    // partition coordinates near the view
    view_partition_coords: VectorInt,

    // The set of all possible partition displacements from view_partition_coords
    // this is constant
    displacement_set : HashSet<VectorInt>,
    
    
}

pub trait ChunkData {
    fn initialize(&mut self, world_chunk_coord: VectorInt);
    fn allocate() -> Self;
}

const DISPLACEMENT_MAP_DIMS: [usize ; 3] = [45, 15, 45];

impl<T: ChunkData>  DisplacedChunks<T> {
    pub fn new(view_partition_coords: VectorInt)
        -> DisplacedChunks<T>
    {
        let displacement_set = radius_displacement_set();


        let chunks = 
            displacement_set
            .iter()
            .map(|&displacement| Chunk { data: T::allocate(), partition_coords: displacement + view_partition_coords, initialized: false, dirty: false})
            .collect();

        DisplacedChunks 
        {
            chunks,
            view_partition_coords,
            displacement_set,
        }
    }
    fn closest_uninitialized_chunk_index(&self)
        -> Option<usize>
    {
        (0..self.len())
        .filter(|&index| 
            !self.chunks[index].initialized)
        .min_by_key(|&index| 
            Self::mag_squared(self.chunks[index].partition_coords - self.view_partition_coords))
    }
    fn mag_squared(disp: VectorInt)
        -> i32
    {
        disp.iter().fold(0, |acc, i| acc + i * i)
    }
    
    pub fn try_initialize(&mut self) {
        if let Some(index) = self.closest_uninitialized_chunk_index() {
            let chunk = &mut self.chunks[index];
            chunk.data.initialize(chunk.partition_coords);
            chunk.initialized = true;
            chunk.dirty = true;
        }
    }

    // obtains mutable reference to dirty chunks
    fn get_mut_dirty_chunks(&mut self) -> Vec<(usize, &mut Chunk<T>)>{
        self.chunks
        .iter_mut()
        .enumerate()
        .filter_map(|(i, c)|
            if c.dirty {Some((i, c))}
            else {None}
        )
        .collect()
    }


    pub fn clean_dirty_chunks(&mut self) -> Vec<(usize, &T)> {
        let mut dirty_chunks = self.get_mut_dirty_chunks();

        dirty_chunks
        .iter_mut()
        .for_each(|(_, m)|
            m.dirty = false
        );

        dirty_chunks
        .into_iter()
        .map(|(i, m)| (i, &m.data))
        .collect()
    }

    pub fn len(&self)
        -> usize
    {
        self.chunks.len()
    }

    // set field and update chunk partition coords
    pub fn set_view_partition_coords(&mut self, coords: VectorInt) {
        self.view_partition_coords = coords;

        let mut invalid_partition_ids: Vec<usize> = Vec::new();
        let mut filled_displacements: HashSet<VectorInt> = HashSet::new();

        // loop through chunks to extract information and uninitialize chunks
        for (partition_id, chunk) in self.chunks.iter_mut().enumerate() {
            let partition_displacement = chunk.partition_coords - self.view_partition_coords;
            if !displacement_valid(partition_displacement) {
                chunk.initialized = false;
                invalid_partition_ids.push(partition_id);
            } else {
                filled_displacements.insert(partition_displacement);
            }
        }

        // assign new partition coordinate to every invalidated partition
        for unfilled_displacement in self.displacement_set.difference(&filled_displacements) {
            let invalid_partition_id = invalid_partition_ids.pop().expect("Not enough invalid partition ids!");
            self.chunks[invalid_partition_id].partition_coords = unfilled_displacement + self.view_partition_coords;
        }

        assert!(invalid_partition_ids.len() == 0);
    }

    pub fn get_index_map(&self) -> Vec<u16> {
        let map_dims = DISPLACEMENT_MAP_DIMS;
        assert!(map_dims[0] % 2 == 1 && map_dims[1] % 2 == 1 && map_dims[2] % 2 == 1);
        let volume = map_dims[0] * map_dims[1] * map_dims[2];
        let map_origin: VectorInt = na::Vector3::new((map_dims[0] / 2) as i32, (map_dims[1] / 2) as i32, (map_dims[2] / 2) as i32);
        let mut map_vec = vec![u16::MAX ; volume];
        for (i, chunk) in self.chunks.iter().enumerate().filter(|(_, c)| c.initialized) {
            assert!(i < u16::MAX as usize);
            let map_coords = chunk.partition_coords - self.view_partition_coords + map_origin;


            // check for irregular map coords
            if map_coords.x < 0 || map_coords.y < 0 || map_coords.z < 0
                || map_coords.x >= map_dims[0] as i32 || map_coords.y >= map_dims[1] as i32 || map_coords.z >= map_dims[2] as i32 {
                panic!("map coords is outside map dims");
            }

            let map_vec_index = (map_coords.x + (map_coords.y * map_dims[0] as i32) + (map_coords.z * map_dims[0] as i32 * map_dims[1] as i32)) as usize;
            map_vec[map_vec_index] = (super::render::resources::chunk_id_variant_to_id(ChunkIDVariant::PartitionID(i as u32))) as u16;
        }
        
        // println!("index map created in {} ms", start_instant.elapsed().as_secs_f32() * 1000.);

        return map_vec;
    }   

}