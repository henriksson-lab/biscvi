

////////////////////////////////////////////////////////////
/// Camera
pub struct GBrowserCamera {

    pub chr: String,
    pub from: u64,
    pub to: u64,

}

impl GBrowserCamera {

    ////////////////////////////////////////////////////////////
    /// Convert world (genome) coordinates to screen coordinates          need to think about best coordinate system
    pub fn world2cam(&self, wx: u64, screen_width: f32) -> f32 {
        let delta = self.to - self.from;
        ((wx - self.from) as f32)*screen_width/(delta as f32)
    }


    ////////////////////////////////////////////////////////////
    /// Zoom around middle position
    pub fn zoom(&mut self, scale: f32) {

        let delta = self.to - self.from;
        let midpos = (self.to + self.from)/2;

        let newdelta = ((delta as f32)*scale) as u64;

        self.from = midpos - newdelta/2;
        self.to = midpos + newdelta/2;
    }

}


