use std::{ops::{IndexMut, Index, Add}, hint::{unreachable_unchecked, black_box}, mem};
use crate::{block::blockface::{BlockFace, Normal}, world::chunk::Chunk};
#[repr(C, align(8))]
pub struct StagingBuffer {
    pub index: usize,
    pub buffer: Box<Buffer>,
}

#[repr(C, align(8))]
pub struct Buffer(pub [u8; 262144]);

impl StagingBuffer {
    pub fn put(&mut self, val: u8) {
        // self.arr[self.ind] = val;
        unsafe {
            *self.buffer.0.get_unchecked_mut(self.index) = val;
        }
        self.index += 1;
    }
    pub fn put_u32(&mut self, val: u32) {
        unsafe {
            let loc = self.buffer.0.as_mut_ptr().byte_add(self.index) as *mut u32;
            *loc = val;
        }
        self.index += 4;
    }
    pub fn put_u64(&mut self, val: u64) {
        unsafe {
            let loc = self.buffer.0.as_mut_ptr().byte_add(self.index) as *mut u64;
            *loc = val;
        }
        self.index += 8;
    }
    pub fn get_u64<T: Into<usize>>(&self, pos: T) -> u64 {
        unsafe {
            let loc = self.buffer.0.as_ptr().byte_add(pos.into()) as *mut u64;
            return *loc
        }
    }
    pub fn new() -> StagingBuffer {
        return StagingBuffer {
            index: 0,
            buffer: Box::new(Buffer([0; 262144])),
        }
    }
    pub fn put_face(&mut self, face: &BlockFace, pos: usize) {
        unsafe {
            let loc = self.buffer.0.as_mut_ptr().byte_add(self.index) as *mut u64;
            *loc = face.as_u64() + Chunk::INDICES_ZYX[pos] as u64;
        }
        self.index += 8;
    }
    
    /** 
     *   0f0   00f
     *   ---------
     * 0 ure ~ lef
     * 1 ven ~ bot
     * 2 dep ~ rig
     * 3 wid ~ top
     * 4 hei ~ dep
     * 5 nor ~ nor
     * 6 tex ~ tex
     * 7 tex ~ tex
     * 
     * 
     * Converts a quad from left-bottom-right-top format to u-v-d-w-h format
     */
    #[allow(unused_parens)]
    pub fn format_quads(&mut self) { unsafe {
        for i in (0..self.index).step_by(8) {
            let face = &mut self.buffer.0[i..i+8];
            let ure = (face[0]);
            let ven = (face[1]);
            let dep = (face[2] & 0xf0) | (0x0 + (face[4] & 0x0f));
            let nor = (face[5]);
            let wid = (face[3] & 0xf0) | (0xf - (face[2] & 0x0f) - (face[0] & 0x0f));
            let hei = (face[4] & 0xf0) | (0xf - (face[3] & 0x0f) - (face[1] & 0x0f));
            let tex = (face[6], face[7]);
            // face[0] = ure;
            // face[1] = ven;
            face[2] = dep;
            face[3] = nor;
            face[4] = wid;
            face[5] = hei;
            // arr[6] = tex.0;
            // arr[7] = tex.1;
            /*
            let nor = mem::transmute::<u8, Normal>(nor);
            match nor {
                Normal::SOUTH => {
                    let start_x = ure / 16;
                    let end_x = (ure + wid + 1) / 16;

                    let start_y = ven / 16;
                    let end_y = (ven + hei + 1) / 16;

                    let z = (dep + 1) / 16;
                    for x in start_x..end_x {
                        for y in start_y..end_y {
                            self.light_index += 1;
                            self.light[self.light_index] = chunk.light[((x as usize) << 8) | ((y as usize) << 4) | ((z as usize) << 0)];
                        }
                    }
                }
                Normal::NORTH => {
                    let start_x = ure / 16;
                    let end_x = (ure + wid + 1) / 16;

                    let start_y = ven / 16;
                    let end_y = (ven + hei + 1) / 16;

                    let z = dep / 16;
                    for x in start_x..end_x {
                        for y in start_y..end_y {
                            self.light_index += 1;
                            self.light[self.light_index] = chunk.light[((x as usize) << 8) | ((y as usize) << 4) | ((z as usize) << 0)];
                        }
                    }
                }
                Normal::WEST => {
                    let x = dep / 16;

                    let start_y = ven / 16;
                    let end_y = (ven + hei + 1) / 16;

                    let start_z = ure / 16;
                    let end_z = (ure + wid + 1) / 16;
                    
                    for y in start_y..end_y {
                        for z in start_z..end_z {
                            self.light_index += 1;
                            self.light[self.light_index] = chunk.light[((x as usize) << 8) | ((y as usize) << 4) | ((z as usize) << 0)];
                        }
                    }
                }
                Normal::EAST => {
                    let x = (dep + 1) / 16;

                    let start_y = ven / 16;
                    let end_y = (ven + hei + 1) / 16;

                    let start_z = ure / 16;
                    let end_z = (ure + wid + 1) / 16;
                    
                    for y in start_y..end_y {
                        for z in start_z..end_z {
                            self.light_index += 1;
                            self.light[self.light_index] = chunk.light[((x as usize) << 8) | ((y as usize) << 4) | ((z as usize) << 0)];
                        }
                    }
                }
                Normal::DOWN => {
                    let start_x = ven / 16;
                    let end_x = (ven + hei + 1) / 16;

                    let y = dep / 16;

                    let start_z = ure / 16;
                    let end_z = (ure + wid + 1) / 16;
                    
                    for x in start_x..end_x {
                        for z in start_z..end_z {
                            self.light_index += 1;
                            self.light[self.light_index] = chunk.light[((x as usize) << 8) | ((y as usize) << 4) | ((z as usize) << 0)];
                        }
                    }
                }
                Normal::UP => {
                    let start_x = ven / 16;
                    let end_x = (ven + hei + 1) / 16;

                    let y = (dep + 1) / 16;

                    let start_z = ure / 16;
                    let end_z = (ure + wid + 1) / 16;
                    
                    for x in start_x..end_x {
                        for z in start_z..end_z {
                            self.light_index += 1;
                            self.light[self.light_index] = chunk.light[((x as usize) << 8) | ((y as usize) << 4) | ((z as usize) << 0)];
                        }
                    }
                }
                _ => unreachable_unchecked()
            }
            // */
        }
    } }
    pub fn reset(&mut self) {
        self.index = 0;
    }
}

impl<T: Into<usize>> Index<T> for StagingBuffer {
    type Output = u8;

    fn index(&self, index: T) -> &Self::Output {
        // return &self.arr[index.into()];
        return unsafe { self.buffer.0.get_unchecked(index.into()) }
    }
}

impl<T: Into<usize>> IndexMut<T> for StagingBuffer {
    fn index_mut(&mut self, index: T) -> &mut Self::Output {
        // return &mut self.arr[index.into()];
        return unsafe { self.buffer.0.get_unchecked_mut(index.into()) }
    }
}