use rand::rngs::OsRng;
use rand::Rng;

use crate::{BODY1_BASE_SPINE, BODY1_HEAD, BODY2_BASE_SPINE, BODY2_HEAD};

pub fn spawn_dancer_mock() -> Result<(), Box<dyn std::error::Error>> {
    std::thread::spawn(move || {
        let mut rng = OsRng::default();
        loop {
            let mut cur_pos = BODY1_BASE_SPINE.lock();
            let mut rand_dir = [0.0, 0.0, 0.0];
            if cur_pos[0] > 1.0 {
                rand_dir[0] = rng.gen_range(-1.0..0.0);
            } else if cur_pos[0] < -1.0 {
                rand_dir[0] = rng.gen_range(0.0..1.0);
            } else {
                rand_dir[0] = rng.gen_range(-1.0..1.0);
            }
            if cur_pos[1] > 0.2 {
                rand_dir[1] = rng.gen_range(-0.2..0.2);
            } else if cur_pos[1] < -0.2 {
                rand_dir[1] = rng.gen_range(0.0..0.2);
            } else {
                rand_dir[1] = rng.gen_range(-0.2..0.2);
            }
            if cur_pos[2] < -3.0 {
                rand_dir[2] = rng.gen_range(0.0..1.0);
            } else if cur_pos[2] > -1.0 {
                rand_dir[2] = rng.gen_range(-1.0..0.0);
            } else {
                rand_dir[2] = rng.gen_range(-1.0..1.0);
            }
            cur_pos[0] = cur_pos[0] + rand_dir[0] * 0.01;
            cur_pos[1] = cur_pos[1] + rand_dir[1] * 0.01;
            cur_pos[2] = cur_pos[2] + rand_dir[2] * 0.01;
            let body_pos = *cur_pos;
            std::mem::drop(cur_pos);
            let mut head_pos = BODY1_HEAD.lock();
            *head_pos = [body_pos[0], body_pos[1] + 0.2, body_pos[2]];

            let mut cur_pos = BODY2_BASE_SPINE.lock();
            let mut rand_dir = [0.0, 0.0, 0.0];
            if cur_pos[0] > 1.0 {
                rand_dir[0] = rng.gen_range(-1.0..0.0);
            } else if cur_pos[0] < -1.0 {
                rand_dir[0] = rng.gen_range(0.0..1.0);
            } else {
                rand_dir[0] = rng.gen_range(-1.0..1.0);
            }
            if cur_pos[1] > 0.2 {
                rand_dir[1] = rng.gen_range(-0.2..0.2);
            } else if cur_pos[1] < -0.2 {
                rand_dir[1] = rng.gen_range(0.0..0.2);
            } else {
                rand_dir[1] = rng.gen_range(-0.2..0.2);
            }
            if cur_pos[2] < -3.0 {
                rand_dir[2] = rng.gen_range(0.0..1.0);
            } else if cur_pos[2] > -1.0 {
                rand_dir[2] = rng.gen_range(-1.0..0.0);
            } else {
                rand_dir[2] = rng.gen_range(-1.0..1.0);
            }
            cur_pos[0] = cur_pos[0] + rand_dir[0] * 0.01;
            cur_pos[1] = cur_pos[1] + rand_dir[1] * 0.01;
            cur_pos[2] = cur_pos[2] + rand_dir[2] * 0.01;
            let body_pos = *cur_pos;
            std::mem::drop(cur_pos);
            let mut head_pos = BODY2_HEAD.lock();
            *head_pos = [body_pos[0], body_pos[1] + 0.2, body_pos[2]];
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
    });
    Ok(())
}