use crate::{block::{blockstate::BlockState, blockface::{Normal, BlockFace}}, util::{gl_helper::{Buffer, log_if_error, log_error}, buffer::ByteBuffer}, BLOCKS};

use super::world::AxisOrder;

#[derive(Debug)]
pub struct Chunk<'a> {
    pub blocks: [&'a BlockState; 4096],
    pub chunk_pos: Vec3i,
    pub face_count: u32,
    pub buffer: Option<Buffer>
}

impl Chunk<'_> {
    fn opposing_south(&self, x: usize, y: usize, z: usize) -> &BlockFace {
        if z == 0 { return &BlockFace::NONE }
        let pos: usize = (x << 8) | (y << 4) | ((z - 1) << 0);
        return &self.blocks[pos].model[Normal::NORTH]
    }
    fn opposing_north(&self, x: usize, y: usize, z: usize) -> &BlockFace {
        if z == 15 { return &BlockFace::NONE }
        let pos: usize = (x << 8) | (y << 4) | ((z + 1) << 0);
        return &self.blocks[pos].model[Normal::SOUTH]
    }
    fn opposing_west(&self, x: usize, y: usize, z: usize) -> &BlockFace {
        if x == 0 { return &BlockFace::NONE }
        let pos: usize = ((x - 1) << 8) | (y << 4) | (z << 0);
        return &self.blocks[pos].model[Normal::EAST]
    }
    fn opposing_down(&self, x: usize, y: usize, z: usize) -> &BlockFace {
        if y == 0 { return &BlockFace::NONE }
        let pos: usize = (x << 8) | ((y - 1) << 4) | (z << 0);
        return &self.blocks[pos].model[Normal::UP]
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
        
        for x in 0..16 {
            for y in 0..16 {
                for z in 0..16 {
                    let pos = (chunk_x) |
                            (x << 16) |
                            (chunk_y) |
                            (y << 8 ) |
                            (chunk_z) |
                            (z << 0 );
                    let (noise_val, block) = unsafe {(
                        noise.get_unchecked(pos),
                        self.blocks.get_unchecked_mut((x << 8) | (y << 4) | (z << 0))
                    )};

                    *block = match *noise_val {
                        val if val < 0.2 => {
                            &BLOCKS[1]
                        },
                        _ => &BLOCKS[0]
                    };
                    // *block = &BLOCKS[1]
                }
            }
        }
    }

    pub fn mesh_south(&mut self, buffer: &mut ByteBuffer) {
        let mut row: [Run<{AxisOrder::ZYX}>; 16] = Default::default();

        let mut run: &mut Run<{AxisOrder::ZYX}> = &mut row[0];
        let mut active_run: bool = false;
        let mut same_row: bool = false;

        for z in 0..16 {
            for y in 0..16 {
                for x in 0..16 {
                    let pos = (x << 8) | (y << 4) | (z << 0);
                    
                    let face: &BlockFace = match &self.blocks[pos].model[Normal::SOUTH] {
                        face if
                            !face.is_none() &&
                            (self.opposing_south(x, y, z).is_none() ||
                            face.not_culled_by_zyx(self.opposing_south(x, y, z))) => {
                            // print!("\n{x:0>2} {y:0>2} {z:0>2}");
                            face
                        },
                        _face => {
                            /*
                            if active_run {
                                print!("\n{x:0>2} {y:0>2} {z:0>2} | end run");
                                if _face.is_none() {print!("missing")}
                                else {print!("culled")}
                                print!("\n");
                            }
                            // */
                            active_run = false;
                            continue
                        }
                    };
                    if active_run && same_row {
                        if run.match_right(&face) {
                            // print!(" | merge face");
                            run.merge_face(buffer, &face);
                            continue
                        } else {
                            // print!(" | end run");
                            active_run = false;
                        }
                    }
                    
                    if !active_run {
                        run = &mut row[x];
                        if run.row == (z * 16 + y).wrapping_sub(1) as u8 && run.match_top_left(&face) {
                            same_row = false;
                            active_run = true;
                            // print!(" | begin merge {}-{}", run.start, run.end);
                        }
                    }

                    if active_run {
                        if run.end == x as u8 {
                            if run.match_top_right(&face) {
                                run.pull(buffer, &face, x as u8, y as u8, z as u8);
                                active_run = false;
                                // print!(" | do merge");
                            }
                            else {
                                run.pull_partial(buffer, &face, x as u8, y as u8, z as u8);
                                same_row = true;
                                // print!(" | fail merge");
                            }
                        }
                        else {
                            let next = ((x + 1) << 8) | (y << 4) | (z << 0);
                            let next_face = &self.blocks[next].model[Normal::SOUTH];

                            
                            if next_face.is_none() ||
                                !face.match_right_zyx(next_face) ||
                                (!self.opposing_south(x + 1, y, z).is_none() && next_face.culled_by_zyx(self.opposing_south(x + 1, y, z))) {
                                /*
                                print!(" | abort");
                                if next_face.is_none() {
                                    print!(":missing {:?}", next_face)
                                } else if !face.match_right(next_face) {
                                    print!(":mismatch")
                                } if !self.opposing(Normal::SOUTH, x, y, z).is_none() && face.culled_by(self.opposing(Normal::SOUTH, x, y, z)) {
                                    print!(":culled")
                                }
                                */
                                run.pull_partial(buffer, &face, x as u8, y as u8, z as u8);
                                active_run = false;
                            }
                            else {
                                // print!(" | continue");
                            }
                        }
                        continue
                    }
                    // print!(" | new run");
                    run = &mut row[x];
                    same_row = true;
                    active_run = true;
                    run.begin(buffer, &face, x as u8, y as u8, z as u8);
                }
                active_run = false;
                // print!(" | end row {y}");
            }
            // print!(" | end face {z}");
        }

        // self.face_count = (buffer.pos as u32) / 8;
        
        /*
        if self.face_count == 0 { 
            // gl_buffer.kill();
            return;
        }
        let gl_buffer: Buffer;
        unsafe {
            gl_buffer = self.buffer.take().unwrap_unchecked();
        }
        
        gl_buffer.storage((buffer.pos + 16) as isize, gl::DYNAMIC_STORAGE_BIT | gl::MAP_WRITE_BIT);
        gl_buffer.upload_slice(&[self.chunk_pos.x, self.chunk_pos.y, self.chunk_pos.z, 0], 0, 16);
        gl_buffer.upload_slice(&buffer.arr.as_slice(), 16, buffer.pos as isize);

        self.buffer = Some(gl_buffer);
        // */
    }

    pub fn mesh_north(&mut self, buffer: &mut ByteBuffer) {
        let mut row: [Run<{AxisOrder::ZYX}>; 16] = Default::default();

        let mut run: &mut Run<{AxisOrder::ZYX}> = &mut row[0];
        let mut active_run: bool = false;
        let mut same_row: bool = false;

        for z in 0..16 {
            for y in 0..16 {
                for x in 0..16 {
                    let pos = (x << 8) | (y << 4) | (z << 0);
                    
                    let face: &BlockFace = match &self.blocks[pos].model[Normal::NORTH] {
                        face if
                            !face.is_none() &&
                            (self.opposing_north(x, y, z).is_none() ||
                            face.not_culled_by_zyx(self.opposing_north(x, y, z))) => {
                            // print!("\n{x:0>2} {y:0>2} {z:0>2}");
                            face
                        },
                        _face => {
                            /*
                            if active_run {
                                print!("\n{x:0>2} {y:0>2} {z:0>2} | end run");
                                if _face.is_none() {print!("missing")}
                                else {print!("culled")}
                                print!("\n");
                            }
                            // */
                            active_run = false;
                            continue
                        }
                    };
                    if active_run && same_row {
                        if run.match_right(&face) {
                            // print!(" | merge face");
                            run.merge_face(buffer, &face);
                            continue
                        } else {
                            // print!(" | end run");
                            active_run = false;
                        }
                    }
                    
                    if !active_run {
                        run = &mut row[x];
                        if run.row == (z * 16 + y).wrapping_sub(1) as u8 && run.match_top_left(&face) {
                            same_row = false;
                            active_run = true;
                            // print!(" | begin merge {}-{}", run.start, run.end);
                        }
                    }

                    if active_run {
                        if run.end == x as u8 {
                            if run.match_top_right(&face) {
                                run.pull(buffer, &face, x as u8, y as u8, z as u8);
                                active_run = false;
                                // print!(" | do merge");
                            }
                            else {
                                run.pull_partial(buffer, &face, x as u8, y as u8, z as u8);
                                same_row = true;
                                // print!(" | fail merge");
                            }
                        }
                        else {
                            let next = ((x + 1) << 8) | (y << 4) | (z << 0);
                            let next_face = &self.blocks[next].model[Normal::NORTH];

                            
                            if next_face.is_none() ||
                                !face.match_right_zyx(next_face) ||
                                (!self.opposing_north(x + 1, y, z).is_none() && next_face.culled_by_zyx(self.opposing_north(x + 1, y, z))) {
                                /*
                                print!(" | abort");
                                if next_face.is_none() {
                                    print!(":missing {:?}", next_face)
                                } else if !face.match_right(next_face) {
                                    print!(":mismatch")
                                } if !self.opposing(Normal::NORTH, x, y, z).is_none() && face.culled_by(self.opposing(Normal::NORTH, x, y, z)) {
                                    print!(":culled")
                                }
                                */
                                run.pull_partial(buffer, &face, x as u8, y as u8, z as u8);
                                active_run = false;
                            }
                            else {
                                // print!(" | continue");
                            }
                        }
                        continue
                    }
                    // print!(" | new run");
                    run = &mut row[x];
                    same_row = true;
                    active_run = true;
                    run.begin(buffer, &face, x as u8, y as u8, z as u8);
                }
                active_run = false;
                // print!(" | end row {y}");
            }
            // print!(" | end face {z}");
        }

        // self.face_count = (buffer.pos as u32) / 8;
        
        /*
        if self.face_count == 0 { 
            // gl_buffer.kill();
            return;
        }
        let gl_buffer: Buffer;
        unsafe {
            gl_buffer = self.buffer.take().unwrap_unchecked();
        }
        
        gl_buffer.storage((buffer.pos + 16) as isize, gl::DYNAMIC_STORAGE_BIT | gl::MAP_WRITE_BIT);
        gl_buffer.upload_slice(&[self.chunk_pos.x, self.chunk_pos.y, self.chunk_pos.z, 0], 0, 16);
        gl_buffer.upload_slice(&buffer.arr.as_slice(), 16, buffer.pos as isize);

        self.buffer = Some(gl_buffer);
        // */
    }

    pub fn mesh_west(&mut self, buffer: &mut ByteBuffer) {
        let mut row: [Run<{AxisOrder::XYZ}>; 16] = Default::default();

        let mut run: &mut Run<{AxisOrder::XYZ}> = &mut row[0];
        let mut active_run: bool = false;
        let mut same_row: bool = false;

        for x in 0..16 {
            for y in 0..16 {
                for z in 0..16 {
                    let pos = (x << 8) | (y << 4) | (z << 0);
                    
                    let face: &BlockFace = match &self.blocks[pos].model[Normal::WEST] {
                        face if
                            !face.is_none()  &&
                            (self.opposing_west(x, y, z).is_none() ||
                            face.not_culled_by_zyx(self.opposing_west(x, y, z))) => {
                            // print!("\n{x:0>2} {y:0>2} {z:0>2}");
                            face
                        },
                        _face => {
                            /*
                            if active_run {
                                print!("\n{x:0>2} {y:0>2} {z:0>2} | end run");
                                if _face.is_none() {print!("missing")}
                                else {print!("culled")}
                                print!("\n");
                            }
                            // */
                            active_run = false;
                            continue
                        }
                    };
                    // /* 
                    if active_run && same_row {
                        if run.match_right(&face) {
                            // print!(" | merge face");
                            run.merge_face(buffer, &face);
                            continue
                        } else {
                            // print!(" | end run");
                            active_run = false;
                        }
                    }
                    // */
                    // /*
                    if !active_run {
                        run = &mut row[z];
                        if run.row == (x * 16 + y).wrapping_sub(1) as u8 && run.match_top_left(&face) {
                            same_row = false;
                            active_run = true;
                            // print!(" | begin merge {}-{}", run.start, run.end);
                        }
                    }

                    if active_run {
                        if run.end == z as u8 {
                            if run.match_top_right(&face) {
                                run.pull(buffer, &face, x as u8, y as u8, z as u8);
                                active_run = false;
                                // print!(" | do merge");
                            }
                            else {
                                run.pull_partial(buffer, &face, x as u8, y as u8, z as u8);
                                same_row = true;
                                // print!(" | fail merge");
                            }
                        }
                        else {
                            let next = (x << 8) | (y << 4) | ((z + 1) << 0);
                            let next_face = &self.blocks[next].model[Normal::WEST];

                            
                            if next_face.is_none() ||
                                !face.match_right_xyz(next_face) ||
                                (false && !self.opposing_west(x, y, z + 1).is_none() && next_face.culled_by_zyx(self.opposing_west(x, y, z + 1))) {
                                /*
                                print!(" | abort");
                                if next_face.is_none() {
                                    print!(":missing {:?}", next_face)
                                } else if !face.match_right(next_face) {
                                    print!(":mismatch")
                                } if !self.opposing(Normal::NORTH, x, y, z).is_none() && face.culled_by(self.opposing(Normal::NORTH, x, y, z)) {
                                    print!(":culled")
                                }
                                */
                                run.pull_partial(buffer, &face, x as u8, y as u8, z as u8);
                                active_run = false;
                            }
                            else {
                                // print!(" | continue");
                            }
                        }
                        continue
                    }
                    // */
                    // print!(" | new run");
                    run = &mut row[z];
                    same_row = true;
                    active_run = true;
                    run.begin(buffer, &face, x as u8, y as u8, z as u8);
                }
                active_run = false;
                // print!(" | end row {y}");
            }
            // print!(" | end face {z}");
        }

        // self.face_count = (buffer.pos as u32) / 8;
        
        /*
        if self.face_count == 0 { 
            // gl_buffer.kill();
            return;
        }
        let gl_buffer: Buffer;
        unsafe {
            gl_buffer = self.buffer.take().unwrap_unchecked();
        }
        
        gl_buffer.storage((buffer.pos + 16) as isize, gl::DYNAMIC_STORAGE_BIT | gl::MAP_WRITE_BIT);
        gl_buffer.upload_slice(&[self.chunk_pos.x, self.chunk_pos.y, self.chunk_pos.z, 0], 0, 16);
        gl_buffer.upload_slice(&buffer.arr.as_slice(), 16, buffer.pos as isize);

        self.buffer = Some(gl_buffer);
        // */
    }

    pub fn mesh_down(&mut self, buffer: &mut ByteBuffer) {
        let mut row: [Run<{AxisOrder::YXZ}>; 16] = Default::default();

        let mut run: &mut Run<{AxisOrder::YXZ}> = &mut row[0];
        let mut active_run: bool = false;
        let mut same_row: bool = false;

        for y in 0..16 {
            for x in 0..16 {
                for z in 0..16 {
                    let pos = (x << 8) | (y << 4) | (z << 0);
                    
                    let face: &BlockFace = match &self.blocks[pos].model[Normal::DOWN] {
                        face if
                            !face.is_none()  &&
                            (self.opposing_down(x, y, z).is_none() ||
                            face.not_culled_by_zyx(self.opposing_down(x, y, z))) => {
                            // print!("\n{x:0>2} {y:0>2} {z:0>2}");
                            face
                        },
                        _face => {
                            /*
                            if active_run {
                                print!("\n{x:0>2} {y:0>2} {z:0>2} | end run");
                                if _face.is_none() {print!("missing")}
                                else {print!("culled")}
                                print!("\n");
                            }
                            // */
                            active_run = false;
                            continue
                        }
                    };
                    // /* 
                    if active_run && same_row {
                        if run.match_right(&face) {
                            // print!(" | merge face");
                            run.merge_face(buffer, &face);
                            continue
                        } else {
                            // print!(" | end run");
                            active_run = false;
                        }
                    }
                    // */
                    /*
                    if !active_run {
                        run = &mut row[z];
                        if run.row == (y * 16 + x).wrapping_sub(1) as u8 && run.match_top_left(&face) {
                            same_row = false;
                            active_run = true;
                            // print!(" | begin merge {}-{}", run.start, run.end);
                        }
                    }

                    if active_run {
                        if run.end == z as u8 {
                            if run.match_top_right(&face) {
                                run.pull(buffer, &face, x as u8, y as u8, z as u8);
                                active_run = false;
                                // print!(" | do merge");
                            }
                            else {
                                run.pull_partial(buffer, &face, x as u8, y as u8, z as u8);
                                same_row = true;
                                // print!(" | fail merge");
                            }
                        }
                        else {
                            let next = (x << 8) | (y << 4) | ((z + 1) << 0);
                            let next_face = &self.blocks[next].model[Normal::DOWN];

                            
                            if next_face.is_none() ||
                                !face.match_right_yxz(next_face) ||
                                (false && !self.opposing_down(x, y, z + 1).is_none() && next_face.culled_by(self.opposing_down(x, y, z + 1))) {
                                /*
                                print!(" | abort");
                                if next_face.is_none() {
                                    print!(":missing {:?}", next_face)
                                } else if !face.match_right(next_face) {
                                    print!(":mismatch")
                                } if !self.opposing(Normal::NORTH, x, y, z).is_none() && face.culled_by(self.opposing(Normal::NORTH, x, y, z)) {
                                    print!(":culled")
                                }
                                */
                                run.pull_partial(buffer, &face, x as u8, y as u8, z as u8);
                                active_run = false;
                            }
                            else {
                                // print!(" | continue");
                            }
                        }
                        continue
                    }
                    // */
                    // print!(" | new run");
                    run = &mut row[z];
                    same_row = true;
                    active_run = true;
                    run.begin(buffer, &face, x as u8, y as u8, z as u8);
                }
                active_run = false;
                // print!(" | end row {y}");
            }
            // print!(" | end face {z}");
        }

        // self.face_count = (buffer.pos as u32) / 8;
        
        /*
        if self.face_count == 0 { 
            // gl_buffer.kill();
            return;
        }
        let gl_buffer: Buffer;
        unsafe {
            gl_buffer = self.buffer.take().unwrap_unchecked();
        }
        
        gl_buffer.storage((buffer.pos + 16) as isize, gl::DYNAMIC_STORAGE_BIT | gl::MAP_WRITE_BIT);
        gl_buffer.upload_slice(&[self.chunk_pos.x, self.chunk_pos.y, self.chunk_pos.z, 0], 0, 16);
        gl_buffer.upload_slice(&buffer.arr.as_slice(), 16, buffer.pos as isize);

        self.buffer = Some(gl_buffer);
        // */
    }

    pub fn new<'a>() -> Chunk<'a> {
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

impl Default for Chunk<'_> {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for Chunk<'_> {
    fn drop(&mut self) {
        // if let Some(buffer) = &self.buffer && buffer.valid() {
            // panic!()
        // }
    }
}

#[derive(Clone, Debug)]
#[allow(dead_code)]
struct Run<const ORDER: AxisOrder> {
    end: u8,
    start: u8,
    row: u8,
    pointer: usize,
    texture: u16,
    min_x: u8,
    min_y: u8,
    min_z: u8,
    max_x: u8,
    max_y: u8,
}

impl Run<{ AxisOrder::ZYX }> {
    fn match_right(&self, face: &BlockFace) -> bool {
        return self.texture == face.t &&
        self.max_x == 15 &&
        face.x == 0 &&
        self.min_y == face.y &&
        self.max_y - self.min_y == face.h;
    }
    fn match_top_right(&self, face: &BlockFace) -> bool {
        return face.y == 0 &&
        self.max_x == face.x + face.w;
    }
    fn match_top_left(&self, face: &BlockFace) -> bool {
        // print!(" | {} {} {} {}", self.texture == face.t, self.max_y == 15, face.y == 0, self.min_x == face.x);
        return self.texture == face.t &&
        self.max_y == 15 &&
        face.y == 0 &&
        self.min_x == face.x
    }
    fn merge_face(&mut self, buffer: &mut ByteBuffer, face: &BlockFace) {
        buffer[self.pointer + 4] += face.w + 1;
        self.end += 1;
        self.max_y = face.y + face.h;
    }
    fn pull_partial(&mut self, buffer: &mut ByteBuffer, face: &BlockFace, x: u8, y: u8, z: u8) {
        self.pointer = buffer.pos;
        buffer.put(self.start * 16 + self.min_x);
        buffer.put(y * 16);
        buffer.put(z * 16);
        buffer.put(face.n.into());

        buffer.put(face.w + 16 * (x - self.start));
        buffer.put(face.h);
        buffer.put(0);
        buffer.put(face.t as u8);

        self.end = x;
        self.row += 1;
    }
    fn pull(&mut self, buffer: &mut ByteBuffer, face: &BlockFace, x: u8, y: u8, z: u8) {
        buffer[self.pointer + 5] += face.h + 1;

        self.row = z * 16 + y;
        self.max_y = face.y + face.h;
    }
    fn begin(&mut self, buffer: &mut ByteBuffer, face: &BlockFace, x: u8, y: u8, z: u8) {
        self.pointer = buffer.pos;
        
        // buffer.put_u64(face.as_u64());
        // buffer[self.pointer]     += x * 16;
        // buffer[self.pointer + 1] += y * 16;
        // buffer[self.pointer + 2] += z * 16;


        buffer.put(face.x + x * 16);
        buffer.put(face.y + y * 16);
        buffer.put(face.z + z * 16);
        buffer.put(face.n.0);

        buffer.put(face.w);
        buffer.put(face.h);
        buffer.put(0);
        buffer.put(face.t as u8);

        self.start = x;
        self.end = x;
        self.texture = face.t;

        self.min_x = face.x;
        self.min_y = face.y;
        self.max_x = face.x + face.w;
        self.max_y = face.y + face.h;

        self.row = z * 16 + y

    }
    fn new() -> Run<{AxisOrder::ZYX}> {
        Run {
            min_z: 0,
            start: 0,
            end: 0,
            min_x: 0,
            min_y: 0,
            max_x: 0,
            max_y: 0,
            texture: 0,
            row: 0,
            pointer: 0,
        }
    }
}
impl Default for Run<{AxisOrder::ZYX}> {
    fn default() -> Self {
        Run::<{AxisOrder::ZYX}>::new()
    }
}

impl Run<{ AxisOrder::XYZ }> {
    fn match_right(&self, face: &BlockFace) -> bool {
        return self.texture == face.t &&
        self.max_x == 15 &&
        face.z == 0 &&
        self.min_y == face.y &&
        self.max_y - self.min_y == face.h;
    }
    fn match_top_right(&self, face: &BlockFace) -> bool {
        return face.y == 0 &&
        self.max_x == face.z + face.w;
    }
    fn match_top_left(&self, face: &BlockFace) -> bool {
        // print!(" | {} {} {} {}", self.texture == face.t, self.max_y == 15, face.y == 0, self.min_x == face.x);
        return self.texture == face.t &&
        self.max_y == 15 &&
        face.y == 0 &&
        self.min_x == face.z
    }
    fn merge_face(&mut self, buffer: &mut ByteBuffer, face: &BlockFace) {
        buffer[self.pointer + 4] += face.w + 1;
        self.end += 1;
        self.max_y = face.y + face.h;
    }
    fn pull_partial(&mut self, buffer: &mut ByteBuffer, face: &BlockFace, x: u8, y: u8, z: u8) {
        self.pointer = buffer.pos;
        buffer.put(x * 16);
        buffer.put(y * 16);
        buffer.put(self.start * 16 + self.min_x);
        buffer.put(face.n.into());

        buffer.put(face.w + 16 * (z - self.start));
        buffer.put(face.h);
        buffer.put(0);
        buffer.put(face.t as u8);

        self.end = z;
        self.row += 1;
    }
    fn pull(&mut self, buffer: &mut ByteBuffer, face: &BlockFace, x: u8, y: u8, z: u8) {
        buffer[self.pointer + 5] += face.h + 1;

        self.row = x * 16 + y;
        self.max_y = face.y + face.h;
    }
    fn begin(&mut self, buffer: &mut ByteBuffer, face: &BlockFace, x: u8, y: u8, z: u8) {
        self.pointer = buffer.pos;
        
        // buffer.put_u64(face.as_u64());
        // buffer[self.pointer]     += x * 16;
        // buffer[self.pointer + 1] += y * 16;
        // buffer[self.pointer + 2] += z * 16;


        buffer.put(face.x + x * 16);
        buffer.put(face.y + y * 16);
        buffer.put(face.z + z * 16);
        buffer.put(face.n.0);

        buffer.put(face.w);
        buffer.put(face.h);
        buffer.put(0);
        buffer.put(face.t as u8);

        self.start = z;
        self.end = z;
        self.texture = face.t;

        self.min_x = face.z;
        self.min_y = face.y;
        self.max_x = face.z + face.w;
        self.max_y = face.y + face.h;

        self.row = x * 16 + y

    }
    fn new() -> Run<{AxisOrder::XYZ}> {
        Run {
            min_z: 0,
            start: 0,
            end: 0,
            min_x: 0,
            min_y: 0,
            max_x: 0,
            max_y: 0,
            texture: 0,
            row: 0,
            pointer: 0,
        }
    }
}
impl Default for Run<{AxisOrder::XYZ}> {
    fn default() -> Self {
        Run::<{AxisOrder::XYZ}>::new()
    }
}

impl Run<{ AxisOrder::YXZ }> {
    fn match_right(&self, face: &BlockFace) -> bool {
        return self.texture == face.t &&
        self.max_x == 15 &&
        face.z == 0 &&
        self.min_y == face.y &&
        self.max_y - self.min_y == face.h;
    }
    fn match_top_right(&self, face: &BlockFace) -> bool {
        return face.x == 0 &&
        self.max_x == face.z + face.w;
    }
    fn match_top_left(&self, face: &BlockFace) -> bool {
        // print!(" | {} {} {} {}", self.texture == face.t, self.max_y == 15, face.y == 0, self.min_x == face.x);
        return self.texture == face.t &&
        self.max_y == 15 &&
        face.x == 0 &&
        self.min_x == face.z
    }
    fn merge_face(&mut self, buffer: &mut ByteBuffer, face: &BlockFace) {
        buffer[self.pointer + 5] += face.h + 1;
        self.end += 1;
        self.max_y = face.x + face.h;
    }
    fn pull_partial(&mut self, buffer: &mut ByteBuffer, face: &BlockFace, x: u8, y: u8, z: u8) {
        self.pointer = buffer.pos;
        buffer.put(x * 16);
        buffer.put(y * 16);
        buffer.put(self.start * 16 + self.min_x);
        buffer.put(face.n.into());

        buffer.put(face.w);
        buffer.put(face.h + 16 * (z - self.start));
        buffer.put(0);
        buffer.put(face.t as u8);

        self.end = z;
        self.row += 1;
    }
    fn pull(&mut self, buffer: &mut ByteBuffer, face: &BlockFace, x: u8, y: u8, z: u8) {
        buffer[self.pointer + 4] += face.w + 1;

        self.row = y * 16 + x;
        self.max_y = face.x + face.w;
    }
    fn begin(&mut self, buffer: &mut ByteBuffer, face: &BlockFace, x: u8, y: u8, z: u8) {
        self.pointer = buffer.pos;
        
        // buffer.put_u64(face.as_u64());
        // buffer[self.pointer]     += x * 16;
        // buffer[self.pointer + 1] += y * 16;
        // buffer[self.pointer + 2] += z * 16;


        buffer.put(face.x + x * 16);
        buffer.put(face.y + y * 16);
        buffer.put(face.z + z * 16);
        buffer.put(face.n.0);

        buffer.put(face.w);
        buffer.put(face.h);
        buffer.put(0);
        buffer.put(face.t as u8);

        self.start = z;
        self.end = z;
        self.texture = face.t;

        self.min_x = face.z;
        self.min_y = face.y;
        self.max_x = face.z + face.h;
        self.max_y = face.x + face.w;

        self.row = y * 16 + x

    }
    fn new() -> Run<{AxisOrder::YXZ}> {
        Run {
            min_z: 0,
            start: 0,
            end: 0,
            min_x: 0,
            min_y: 0,
            max_x: 0,
            max_y: 0,
            texture: 0,
            row: 0,
            pointer: 0,
        }
    }
}
impl Default for Run<{AxisOrder::YXZ}> {
    fn default() -> Self {
        Run::<{AxisOrder::YXZ}>::new()
    }
}

// impl Default for Run<{AxisOrder::YXZ}> {
    // fn default() -> Self {
        // Run::<{AxisOrder::YXZ}>::new()
    // }
// }


#[derive(Debug)]
#[allow(dead_code)]
pub struct Vec3i {
    pub x: i32, pub y: i32, pub z: i32
}