use core::panic;
use crate::types::Rect;

pub struct Animation {
    pub(crate) time_per: Vec<usize>,
    pub(crate) frames:Vec<Rect>,
    looping:bool
    // You'll want to know the frames involved and the timing for each frame
    // But then there's also dynamic data, which might live in this struct or might live somewhere else
    // An Animation/AnimationState split could be fine, if AnimationState holds the start time and the present frame (or just the start time) and possibly a reference to the Animation
    // but there are lots of designs that will work!
}

impl Animation {
    pub fn new(time_per:Vec<usize>, frames:Vec<Rect>, looping:bool) -> Self {
        Self {
            time_per,
            frames,
            looping
        }
    }
    pub fn get_frame(&self, current_time:usize)->Rect{
        let net_duration:usize = self.time_per.iter().sum();
        let mut acum_time = 0;
        //let t = current_time-net_duration;
        let current_time = if self.looping { 
            current_time % net_duration 
        } else { 
            current_time.min(net_duration-1) 
        };
        for (i,j) in self.time_per.iter().zip(self.frames.iter()){
            acum_time += i;
            if  current_time< acum_time{
                return *j;
            }
        }
        panic!();
    }
}

