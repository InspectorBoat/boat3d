use core::panic;
use std::hint::black_box;

use crate::{block::{blockstate::BlockState, blockface::{Normal, BlockFace, N}}, util::{gl_helper::{Buffer, log_if_error, log_error}, buffer::ByteBuffer}, BLOCKS};

#[derive(Debug)]
pub struct Chunk {
    pub blocks: [&'static BlockState; 4096],
    pub chunk_pos: Vec3i,
    pub face_count: u32,
    pub buffer: Option<Buffer>
}
impl Chunk {
    fn get_face<const NORMAL: N>(&self, index: usize) -> &BlockFace {
        return unsafe { &self.blocks.get_unchecked(index).model[NORMAL] }
    }
    fn get_opposing_face<const NORMAL: N>(&self, index: usize) -> &BlockFace {
        if index & NORMAL.offset_mask().0 == NORMAL.offset_mask().1 { return &BlockFace::NONE; }
        let index = index as isize + NORMAL.offset();
        return unsafe { &self.blocks.get_unchecked(index as usize).model[NORMAL.opposite()] }
    }
    fn get_face_pair<const NORMAL: N>(&self, index: usize) -> (&BlockFace, &BlockFace) {
        (self.get_face::<NORMAL>(index), self.get_opposing_face::<NORMAL>(index))
    }

    fn get_index(x: u8, y: u8, z: u8) -> usize {
        ((x as usize) << 8) | ((y as usize) << 4) | ((z as usize) << 0)
    }
    
    fn opposing_south(&self, index: usize) -> &BlockFace {
        if index & 0x00f == 0 { return &BlockFace::NONE }
        let pos: usize = index - 0x001;

        return unsafe { &self.blocks.get_unchecked(pos).model[Normal::NORTH] }
    }
    fn opposing_north(&self, index: usize) -> &BlockFace {
        if index & 0x00f == 0x00f { return &BlockFace::NONE }
        let pos: usize = index + 0x001;
        return unsafe { &self.blocks.get_unchecked(pos).model[Normal::SOUTH] }
    }
    fn opposing_west(&self, index: usize) -> &BlockFace {
        if index & 0xf00 == 0 { return &BlockFace::NONE }
        let pos: usize = index - 0x100;
        return unsafe { &self.blocks.get_unchecked(pos).model[Normal::EAST] }
    }
    fn opposing_down(&self, index: usize) -> &BlockFace {
        if index & 0x0f0 == 0 { return &BlockFace::NONE }
        let pos: usize = index - 0x010;
        return unsafe { &self.blocks.get_unchecked(pos).model[Normal::UP] }
    }
    
    pub fn make_terrain(&mut self, noise: &mut Vec<f32>, mut chunk_x: usize, mut chunk_y: usize, mut chunk_z: usize) {
        self.chunk_pos = Vec3i {
            x: chunk_x as i32,
            y: chunk_y as i32,
            z: chunk_z as i32
        };
        
        chunk_x <<= 20;
        chunk_y <<= 12;
        chunk_z <<= 4;
        let mut j = 0;
        for x in 0..16 {
            for y in 0..16 {
                for z in 0..16 {
                    let pos = {
                        (chunk_x) |
                        (x << 16) |
                        (chunk_y) |
                        (y << 8 ) |
                        (chunk_z) |
                        (z << 0 )
                    };
                    let (noise_val, block) = unsafe {(
                        noise.get_unchecked(pos),
                        self.blocks.get_unchecked_mut((x << 8) | (y << 4) | (z << 0))
                    )};

                    *block = match *noise_val {
                        // val if val < 0.1 => {
                            // &BLOCKS[1]
                        // },
                        // val if val < 0.2 => {
                            // &BLOCKS[2]
                        // },
                        val if val < 0.2 => {
                            &BLOCKS[3]
                        },
                        _ => &BLOCKS[0]
                    };
                    // *block = if j % 2 == 0 { &BLOCKS[0] } else { &BLOCKS[1] };
                    // *block = if x % 2 == 0 { &BLOCKS[0] } else { &BLOCKS[1] };
                    j += 1;
                }
                j += 1;
            }
            j += 1;
        }
    }

    pub fn mesh_north_south(&mut self, buffer: &mut ByteBuffer) {
        let mut row_s: [Run; 16] = Default::default();
        let mut run_s: &mut Run = &mut row_s[0];
        let mut active_run_s: bool = false;
        let mut same_row_s: bool = false;

        let mut row_n: [Run; 16] = Default::default();
        let mut run_n: &mut Run = &mut row_n[0];
        let mut active_run_n: bool = false;
        let mut same_row_n: bool = false;
        
        let mut row_id: u16 = 0;

        for z in 0..16_u8 { for y in 0..16_u8 { for x in 0..16_u8 {
            let pos = ((x as usize) << 8) | ((y as usize) << 4) | ((z as usize) << 0);
            
            let (face_s, face_n) = self.get_face_pair::<{N::SOUTH}>(pos);
            let compare = BlockFace::compare(face_s, face_n);
            
            'south: {
                if compare.0 {
                    active_run_s = false;
                    break 'south
                }

                /*
                if active_run_s && same_row_s {
                    if run_s.match_right(&face_s) {
                        run_s.merge_face(buffer, &face_s);
                        break 'south
                    } else {
                        active_run_s = false;
                        break 'south
                    }
                }
                // /*
                if !active_run_s {
                    run_s = &mut row_s[x as usize];
                    if run_s.row + 1 == row_id && run_s.match_top_left(&face_s) {
                        same_row_s = false;
                        active_run_s = true;
                    }
                }

                if active_run_s {
                    if run_s.end == x {
                        if run_s.match_top_right(&face_s) {
                            run_s.pull(buffer, &face_s, x, y, z);
                            active_run_s = false;
                        }
                        else {
                            run_s.pull_partial(buffer, &face_s, x, y, z);
                            same_row_s = true;
                        }
                    }
                    else {
                        let next_pos = pos + 0x100;
                        let next_face_s = unsafe {
                            &self.blocks.get_unchecked(next_pos).model[N::SOUTH]
                        };
                        
                        if next_face_s.is_none() ||
                            !Run::match_faces(face_s, next_face_s) ||
                            next_face_s.culled_by(self.opposing_south(next_pos)) {
                            run_s.pull_partial(buffer, &face_s, x, y, z);
                            active_run_s = false;
                        }
                    }
                    break 'south
                }
                // */
                // */
                run_s = &mut row_s[x as usize];
                same_row_s = true;
                active_run_s = true;
                run_s.begin(buffer, &face_s, x, y, z, row_id);
            }
            'north: {
                if compare.1 {
                    active_run_n = false;
                    break 'north
                }
                /*
                if active_run_n && same_row_n {
                    if run_n.match_right(&face_n) {
                        run_n.merge_face(buffer, &face_n);
                        break 'north
                    } else {
                        active_run_n = false;
                        break 'north
                    }
                }
                // /* 
                if !active_run_n {
                    run_n = &mut row_n[x as usize];
                    if run_n.row + 1 == row_id && run_n.match_top_left(&face_n) {
                        same_row_n = false;
                        active_run_n = true;
                    }
                }

                if active_run_n {
                    if run_n.end == x {
                        if run_n.match_top_right(&face_n) {
                            run_n.pull(buffer, &face_n, x, y, z);
                            active_run_n = false;
                        }
                        else {
                            run_n.pull_partial(buffer, &face_n, x, y, z);
                            same_row_n = true;
                        }
                    }
                    else {
                        let next_pos = pos + 0x100;
                        let next_face_n = unsafe {
                            &self.blocks.get_unchecked(next_pos).model[Normal::NORTH]
                        };
                        
                        if next_face_n.is_none() ||
                            !Run::match_faces(face_n, next_face_n) ||
                            next_face_n.culled_by(self.opposing_south(next_pos)) {
                            run_n.pull_partial(buffer, &face_n, x, y, z);
                            active_run_n = false;
                        }
                    }
                    break 'north
                }
                // */
                // */
                run_n = &mut row_n[x as usize];
                active_run_n = true;
                same_row_n = true;
                run_n.begin(buffer, &face_n, x, y, z, row_id);
            }
        } (active_run_s, active_run_n) = (false, false); row_id += 1; } row_id += 16; }
        
        // buffer.convert();
    }

    pub fn new<'a>() -> Chunk {
        return Chunk {
            blocks: [&BLOCKS[0]; 4096],
            chunk_pos: Vec3i {x: 0, y: 0, z: 0},
            buffer: None,
            face_count: 0
        };
    }
    pub fn create_buffer(&mut self) {
        if let Some(_) = &self.buffer {
            panic!();
        }
        self.buffer = Some(Buffer::create());
    }
    pub fn kill_buffer(&mut self) {
        let buffer = self.buffer.take();
        if let Some(buffer) = buffer {
            buffer.kill();
        }
    }
}
impl Default for Chunk {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for Chunk {
    fn drop(&mut self) {
        if let Some(buffer) = &self.buffer && buffer.valid() {
            panic!()
        }
    }
}

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct Run {
    lef: u8,
    bot: u8,
    rig: u8,
    top: u8,
    
    dep: u8,

    start: u8,
    end: u8,
    
    tex: u16,
    
    pointer: u16,
    
    row: u16,
}

impl Run {
    /**
     * Matches the top and bottom right corners of the run with the top and bottom left corners of the face
     * Used to immediately merge a face
     */
    fn match_right(&self, face: &BlockFace) -> bool {
        return {
            self.tex == face.tex &&
            self.rig == face.lef &&
            self.bot == face.bot &&
            self.top == face.top
        }
    }
    /**
     * Matches the top left corner of the run with the bottom left corner of the face
     * Used to begin a merge
     */
    fn match_top_left(&self, face: &BlockFace) -> bool {
        return {
            self.tex == face.tex &&
            self.top == face.bot &&
            self.lef == face.lef
        }
    }
    /**
     * Matches the top right corner of the run with the bottom right corner of the face
     * Used to finalize a merge
     */
    fn match_top_right(&self, face: &BlockFace) -> bool {
        return {
            face.bot == 0 &&
            self.rig == face.rig
        }
    }
    /**
     * Merges a face into the run horizontally
     * Extends the run's end position and updates the end x
     * End y is already guaranteed to match
     */
    fn merge_face(&mut self, buffer: &mut ByteBuffer, face: &BlockFace) {
        buffer[self.pointer as usize + 4] += face.rig + 1;
        self.end += 1;
        self.rig = face.lef + face.rig;
    }
    /**
     * Pulls the run up after an incomplete merge
     * min_x, min_y, min_z, and texture are already guaranteed to match
     */
    fn pull_partial(&mut self, buffer: &mut ByteBuffer, face: &BlockFace, u: u8, v: u8, d: u8) {
        self.pointer = buffer.pos as u16;
        buffer.put(self.start * 16 + self.lef);

        buffer.put(v * 16);
        buffer.put(d * 16);
        buffer.put(face.nor.into());

        buffer.put(16 * (u - self.start) + face.rig);
        buffer.put(face.top);
        buffer.put(0);
        buffer.put(face.top as u8);

        self.end = u;
        self.row += 1;
    }
    /**
     * Pulls the run up after a complete merge
     * Only possible change is max_y
     */
    fn pull(&mut self, buffer: &mut ByteBuffer, face: &BlockFace, u: u8, v: u8, d: u8) {
        buffer[self.pointer as usize + 5] += face.top + 1;

        self.top = face.bot + face.top;
        self.row += 1;
    }
    /**
     * Begins a new run
     */
    fn begin(&mut self, buffer: &mut ByteBuffer, face: &BlockFace, u: u8, v: u8, d: u8, row: u16) {
        self.pointer = buffer.pos as u16;

        // Moving the entire face is more efficient
        let offset = ((u as u64) << 4) | ((v as u64) << 12) | ((d as u64) << 20);
        buffer.put_u64(face.as_u64() + offset);

        self.start = u;
        self.end = u;
        self.tex = face.tex;

        self.lef = face.lef;
        self.bot = face.bot;

        self.rig = face.rig;
        self.top = face.top;

        self.row = row;
    }
    /**
     * Matches the top and bottom right corners of the first face with the top and bottom left corners of the next face
     */
    fn match_faces(face: &BlockFace, next: &BlockFace) -> bool {
        return {
            face.tex == next.tex &&
            face.rig == next.lef &&
            face.bot == next.bot &&
            face.top == next.top
        }
    }

    fn new() -> Run {
        Run {
            start: 0,
            end:   0,
            
            lef:   0,
            bot:   0,
            rig:   0,
            top:   0,

            dep:   0,
            tex:   0,
            
            row:  u16::MAX,

            pointer: 0,
        }
    }
}
impl Default for Run {
    fn default() -> Self {
        Run::new()
    }
}

#[derive(Debug)]
pub struct Vec3i {
    pub x: i32, pub y: i32, pub z: i32
}