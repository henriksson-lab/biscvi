use bstr::BString;



////////////////////////////////////////////////////////////
/// Camera
pub struct GBrowserCamera {
    pub chr: BString,
    pub from: i64,
    pub to: i64,
}

impl GBrowserCamera {

    ////////////////////////////////////////////////////////////
    /// Convert world (genome) coordinates to screen coordinates          need to think about best coordinate system
    pub fn world2cam(&self, wx: i64, screen_width: f32) -> f32 {
        let delta = self.to - self.from;
        ((wx - self.from) as f32)*screen_width/(delta as f32)
    }


    ////////////////////////////////////////////////////////////
    /// Convert world (genome) coordinates to screen coordinates          need to think about best coordinate system
    pub fn cam2world(&self, cx: f32, screen_width: f32) -> f32 {
        let delta = self.to - self.from;
        (self.from as f32) + (cx as f32)*(delta as f32)/screen_width  //// 
    }


    ////////////////////////////////////////////////////////////
    /// Zoom around middle position
    pub fn zoom(&mut self, scale: f32) {

        let midpos = (self.to + self.from)/2;
        self.zoom_around(scale, midpos);
    }



    ////////////////////////////////////////////////////////////
    /// Zoom around middle position
    pub fn zoom_around(&mut self, scale: f32, midpos: i64) {

        let delta = self.to - self.from;

        let newdelta = ((delta as f32)/scale) as i64;

        self.from = midpos - newdelta/2;
        self.to = midpos + newdelta/2;
    }
}


