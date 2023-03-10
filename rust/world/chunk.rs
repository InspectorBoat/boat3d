use core::panic;
use std::hint::black_box;
use std::ops::Sub;
use std::ops::Add;
use crate::{block::{blockstate::BlockState, blockface::{Norm, BlockFace, N}}, util::{gl_helper::{Buffer, log_if_error, log_error}, byte_buffer::ByteBuffer}, BLOCKS};

use super::world::World;

#[derive(Debug)]
pub struct Chunk {
    pub blocks: [u16; 4096],
    pub chunk_pos: Vec3i,
    pub face_count: u32,
    pub buffer: Option<Buffer>
}
impl Chunk {
    fn get_face<const NORMAL: N>(&self, index: usize, world: &World) -> &BlockFace {
        return unsafe { &BLOCKS[*self.blocks.get_unchecked(index) as usize].model[NORMAL] }
    }
    fn get_opposing_face<const NORMAL: N>(&self, index: usize, world: &World) -> &BlockFace {
        match NORMAL {
            N::SOUTH => {
                if index & 0x00f == 0 {
                    if self.chunk_pos.z == 0 {
                        return &BlockFace::NONE;
                    }
                    return &BLOCKS[world.chunks[((self.chunk_pos.x << 10) | (self.chunk_pos.y << 5) | (self.chunk_pos.z.sub(1) << 0)) as usize].blocks[index | 0x00f] as usize].model[N::NORTH];
                }
                return &BLOCKS[self.blocks[index - 0x001] as usize].model[N::NORTH];
            }
            N::WEST => {
                if index & 0xf00 == 0 {
                    if self.chunk_pos.x == 0 {
                        return &BlockFace::NONE;
                    }
                    if (index | 0xf00) >= 4096 { println!("{index}") }
                    return &BLOCKS[world.chunks[((self.chunk_pos.x.sub(1) << 10) | (self.chunk_pos.y << 5) | (self.chunk_pos.z << 0)) as usize].blocks[index | 0xf00] as usize].model[N::EAST];
                }
                return &BLOCKS[self.blocks[index - 0x100] as usize].model[N::EAST];
            }
            N::DOWN => {
                if index & 0x0f0 == 0 {
                    if self.chunk_pos.y == 0 {
                        return &BlockFace::NONE;
                    }
                    return &BLOCKS[world.chunks[(self.chunk_pos.x << 10 | self.chunk_pos.y.sub(1) << 5 | (self.chunk_pos.z) << 0) as usize].blocks[index | 0x0f0] as usize].model[N::UP];
                }
                return &BLOCKS[self.blocks[index - 0x010] as usize].model[N::UP];
            }
            _ => unsafe { std::hint::unreachable_unchecked() }
        }
    }
    fn get_face_pair<const NORMAL: N>(&self, index: usize, world: &World) -> (&BlockFace, &BlockFace) {
        (self.get_face::<NORMAL>(index, world), self.get_opposing_face::<NORMAL>(index, world))
    }

    #[inline]
    const fn get_index(x: u8, y: u8, z: u8) -> usize {
        ((x as usize) << 8) | ((y as usize) << 4) | ((z as usize) << 0)
    }

    pub fn make_terrain(&mut self, noise: &mut Vec<f32>, mut chunk_x: usize, mut chunk_y: usize, mut chunk_z: usize) {
        self.chunk_pos = Vec3i {
            x: chunk_x as i32,
            y: chunk_y as i32,
            z: chunk_z as i32
        };
        
        chunk_x <<= 22;
        chunk_y <<= 13;
        chunk_z <<= 4;
        let mut j = 0;
        for x in 0..16 {
            for y in 0..16 {
                for z in 0..16 {
                    let pos = {
                        (chunk_x) |
                        (x << 18) |
                        (chunk_y) |
                        (y << 9 ) |
                        (chunk_z) |
                        (z << 0 )
                    };
                    let (noise_val, block) = unsafe {(
                        noise.get_unchecked(pos),
                        self.blocks.get_unchecked_mut((x << 8) | (y << 4) | (z << 0))
                    )};
                    *block = match *noise_val {
                        val if val < 0.1 => {
                            1
                        },
                        val if val < 0.2 => {
                            2
                        },
                        val if val < 0.3 => {
                            3
                        },
                        // val if val < 0.4 => {
                            // &BLOCKS[4]
                        // },
                        // val if val < 0.5 => {
                            // &BLOCKS[1]
                        // },
                        _ => 0
                    };
                    // *block = if j % 2 == 0 { 0 } else { 1 };
                    // *block = if x % 2 == 0 { 0 } else { 1 };
                    j += 1;
                }
                j += 1;
            }
            j += 1;
        }
    }

    pub fn mesh_north_south(&mut self, buffer: &mut ByteBuffer, world: &World) {
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
            let pos = Chunk::get_index(x, y, z);

            let (face_s, face_n) = self.get_face_pair::<{N::SOUTH}>(pos, world);
            let compare = BlockFace::compare(face_s, face_n);
            'south: {
                if compare.0 {
                    active_run_s = false;
                    break 'south
                }

                // /*
                if active_run_s && same_row_s {
                    if run_s.match_right(&face_s) {
                        run_s.merge_face(buffer, &face_s);
                        break 'south
                    } else {
                        active_run_s = false;
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
                        let (next_face_s, next_face_n) = self.get_face_pair::<{N::SOUTH}>(next_pos, world);
                        let compare = BlockFace::compare(next_face_s, next_face_n);

                        if compare.0 || !Run::match_faces(face_s, next_face_s) {
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
                // break 'north;
                if compare.1 {
                    active_run_n = false;
                    break 'north
                }
                // /*
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
                        let (next_face_s, next_face_n) = self.get_face_pair::<{N::SOUTH}>(next_pos, world);
                        let compare = BlockFace::compare(next_face_s, next_face_n);

                        if compare.1 || !Run::match_faces(face_n, next_face_n) {
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
    }
    
    pub fn mesh_west_east(&mut self, buffer: &mut ByteBuffer, world: &World) {
        let mut row_w: [Run; 16] = Default::default();
        let mut run_w: &mut Run = &mut row_w[0];
        let mut active_run_w: bool = false;
        let mut same_row_w: bool = false;

        let mut row_e: [Run; 16] = Default::default();
        let mut run_e: &mut Run = &mut row_e[0];
        let mut active_run_e: bool = false;
        let mut same_row_e: bool = false;
        
        let mut row_id: u16 = 0;
        
        for x in 0..16_u8 { for y in 0..16_u8 { for z in 0..16_u8 {
            let pos = Chunk::get_index(x, y, z);

            let (face_w, face_e) = self.get_face_pair::<{N::WEST}>(pos, world);
            let compare = BlockFace::compare(face_w, face_e);
            'west: {
                if compare.0 {
                    active_run_w = false;
                    break 'west
                }

                // /*
                if active_run_w && same_row_w {
                    if run_w.match_right(&face_w) {
                        run_w.merge_face(buffer, &face_w);
                        break 'west
                    } else {
                        active_run_w = false;
                    }
                }
                // /*
                if !active_run_w {
                    run_w = &mut row_w[z as usize];
                    if run_w.row + 1 == row_id && run_w.match_top_left(&face_w) {
                        same_row_w = false;
                        active_run_w = true;
                    }
                }
                if active_run_w {
                    if run_w.end == z {
                        if run_w.match_top_right(&face_w) {
                            run_w.pull(buffer, &face_w, z, y, x);
                            active_run_w = false;
                        }
                        else {
                            run_w.pull_partial(buffer, &face_w, z, y, x);
                            same_row_w = true;
                        }
                    }
                    else {
                        let next_pos = pos + 0x001;
                        let (next_face_w, next_face_e) = self.get_face_pair::<{N::WEST}>(next_pos, world);
                        let compare = BlockFace::compare(next_face_w, next_face_e);

                        if compare.0 || !Run::match_faces(face_w, next_face_w) {
                            run_w.pull_partial(buffer, &face_w, z, y, x);
                            active_run_w = false;
                        }
                    }
                    break 'west
                }
                // */
                // */
                run_w = &mut row_w[z as usize];
                same_row_w = true;
                active_run_w = true;
                run_w.begin(buffer, &face_w, z, y, x, row_id);
            }
            'east: {
                // break 'east;
                if compare.1 {
                    active_run_e = false;
                    break 'east
                }
                // /*
                if active_run_e && same_row_e {
                    if run_e.match_right(&face_e) {
                        run_e.merge_face(buffer, &face_e);
                        break 'east
                    } else {
                        active_run_e = false;
                        break 'east
                    }
                }
                // /* 
                if !active_run_e {
                    run_e = &mut row_e[z as usize];
                    if run_e.row + 1 == row_id && run_e.match_top_left(&face_e) {
                        same_row_e = false;
                        active_run_e = true;
                    }
                }

                if active_run_e {
                    if run_e.end == z {
                        if run_e.match_top_right(&face_e) {
                            run_e.pull(buffer, &face_e, z, y, x);
                            active_run_e = false;
                        }
                        else {
                            run_e.pull_partial(buffer, &face_e, z, y, x);
                            same_row_e = true;
                        }
                    }
                    else {
                        let next_pos = pos + 0x001;
                        let (next_face_w, next_face_e) = self.get_face_pair::<{N::WEST}>(next_pos, world);
                        let compare = BlockFace::compare(next_face_w, next_face_e);

                        if compare.1 || !Run::match_faces(face_e, next_face_e) {
                            run_e.pull_partial(buffer, &face_e, z, y, x);
                            active_run_e = false;
                        }
                    }
                    break 'east
                }
                // */
                // */
                run_e = &mut row_e[z as usize];
                active_run_e = true;
                same_row_e = true;
                run_e.begin(buffer, &face_e, z, y, x, row_id);
            }
        } (active_run_w, active_run_e) = (false, false); row_id += 1; } row_id += 16; }
    }

    pub fn mesh_down_up(&mut self, buffer: &mut ByteBuffer, world: &World) {
        let mut row_d: [Run; 16] = Default::default();
        let mut run_d: &mut Run = &mut row_d[0];
        let mut active_run_d: bool = false;
        let mut same_row_d: bool = false;

        let mut row_u: [Run; 16] = Default::default();
        let mut run_u: &mut Run = &mut row_u[0];
        let mut active_run_u: bool = false;
        let mut same_row_u: bool = false;
        
        let mut row_id: u16 = 0;
        
        for y in 0..16_u8 { for x in 0..16_u8 { for z in 0..16_u8 {
            let pos = Chunk::get_index(x, y, z);

            let (face_d, face_u) = self.get_face_pair::<{N::DOWN}>(pos, world);
            let compare = BlockFace::compare(face_d, face_u);
            'down: {
                if compare.0 {
                    active_run_d = false;
                    break 'down
                }

                // /*
                if active_run_d && same_row_d {
                    if run_d.match_right(&face_d) {
                        run_d.merge_face(buffer, &face_d);
                        break 'down
                    } else {
                        active_run_d = false;
                    }
                }
                // /*
                if !active_run_d {
                    run_d = &mut row_d[z as usize];
                    if run_d.row + 1 == row_id && run_d.match_top_left(&face_d) {
                        same_row_d = false;
                        active_run_d = true;
                    }
                }
                if active_run_d {
                    if run_d.end == z {
                        if run_d.match_top_right(&face_d) {
                            run_d.pull(buffer, &face_d, z, x, y);
                            active_run_d = false;
                        }
                        else {
                            run_d.pull_partial(buffer, &face_d, z, x, y);
                            same_row_d = true;
                        }
                    }
                    else {
                        let next_pos = pos + 0x001;
                        let (next_face_d, next_face_u) = self.get_face_pair::<{N::DOWN}>(next_pos, world);
                        let compare = BlockFace::compare(next_face_d, next_face_u);

                        if compare.0 || !Run::match_faces(face_d, next_face_d) {
                            run_d.pull_partial(buffer, &face_d, z, x, y);
                            active_run_d = false;
                        }
                    }
                    break 'down
                }
                // */
                // */
                run_d = &mut row_d[z as usize];
                same_row_d = true;
                active_run_d = true;
                run_d.begin(buffer, &face_d, z, x, y, row_id);
            }
            'up: {
                // break 'up;
                if compare.1 {
                    active_run_u = false;
                    break 'up
                }
                // /*
                if active_run_u && same_row_u {
                    if run_u.match_right(&face_u) {
                        run_u.merge_face(buffer, &face_u);
                        break 'up
                    } else {
                        active_run_u = false;
                        break 'up
                    }
                }
                // /* 
                if !active_run_u {
                    run_u = &mut row_u[z as usize];
                    if run_u.row + 1 == row_id && run_u.match_top_left(&face_u) {
                        same_row_u = false;
                        active_run_u = true;
                    }
                }

                if active_run_u {
                    if run_u.end == z {
                        if run_u.match_top_right(&face_u) {
                            run_u.pull(buffer, &face_u, z, x, y);
                            active_run_u = false;
                        }
                        else {
                            run_u.pull_partial(buffer, &face_u, z, x, y);
                            same_row_u = true;
                        }
                    }
                    else {
                        let next_pos = pos + 0x001;
                        let (next_face_d, next_face_u) = self.get_face_pair::<{N::DOWN}>(next_pos, world);
                        let compare = BlockFace::compare(next_face_d, next_face_u);

                        if compare.1 || !Run::match_faces(face_u, next_face_u) {
                            run_u.pull_partial(buffer, &face_u, z, x, y);
                            active_run_u = false;
                        }
                    }
                    break 'up
                }
                // */
                // */
                run_u = &mut row_u[z as usize];
                active_run_u = true;
                same_row_u = true;
                run_u.begin(buffer, &face_u, z, x, y, row_id);
            }
        } (active_run_d, active_run_u) = (false, false); row_id += 1; } row_id += 16; }
    }
   
    pub fn new<'a>() -> Chunk {
        return Chunk {
            blocks: [0; 4096],
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
        // if let Some(buffer) = &self.buffer && buffer.valid() {
            // panic!()
        // }
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

    beg: u8,
    end: u8,
    
    tex: u16,
    
    ind: u16,
    
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
        buffer[self.ind + 3] = ((buffer[self.ind + 3] + 0x10) & 0xf0) | face.rig;
        self.end += 1;
        self.rig = face.rig;
    }
    /**
     * Pulls the run up after an incomplete merge
     * min_x, min_y, min_z, and texture are already guaranteed to match
     */
    fn pull_partial(&mut self, buffer: &mut ByteBuffer, face: &BlockFace, u: u8, v: u8, d: u8) {
        let ind = buffer.ind as u16;
        buffer.put_u64(buffer.get_u64(self.ind));
        
        buffer[ind + 1] = v << 4;
        buffer[ind + 2] = (d << 4) | face.rig;
        buffer[ind + 3] = ((u - self.beg) << 4) | face.top;
        buffer[ind + 4] &= 0x0f;
        self.ind = ind;
        self.end = u;
        self.row += 1;
    }
    /**
     * Pulls the run up after a complete merge
     * Only possible change is max_y
     */
    fn pull(&mut self, buffer: &mut ByteBuffer, face: &BlockFace, u: u8, v: u8, d: u8) {
        buffer[self.ind as usize + 4] += 0x10;

        self.top = face.top;
        self.row += 1;
    }
    /**
     * Begins a new run
     */
    fn begin(&mut self, buffer: &mut ByteBuffer, face: &BlockFace, u: u8, v: u8, d: u8, row: u16) {
        self.ind = buffer.ind as u16;
        // Moving the entire face is more efficient
        let offset = ((u as u64) << 4) | ((v as u64) << 12) | ((d as u64) << 20);
        buffer.put_u64(face.as_u64() + offset);

        self.beg = u;
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
            beg: 0,
            end:   0,
            
            lef:   0,
            bot:   0,
            rig:   0,
            top:   0,

            dep:   0,
            tex:   0,
            
            row:  u16::MAX - 1,

            ind: 0,
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