

////////////////////////////////////////////////////////////
/// Camera
pub struct GBrowserCamera {

    pub chr: String,
    pub from: u64,
    pub to: u64,

}

impl GBrowserCamera {

    pub fn world2cam(&self, wx: u64, screen_width: f32) -> f32 {
        let delta = self.to - self.from;
        ((wx - self.from) as f32)*screen_width/(delta as f32)
    }


}


