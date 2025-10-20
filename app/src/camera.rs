
////////////////////////////////////////////////////////////
/// x
#[derive(Debug, PartialEq)]
pub struct Camera2D {
    pub x: f32,
    pub y: f32,
    pub zoom_x: f32,
    pub zoom_y: f32,
}
impl Camera2D {

    ////////////////////////////////////////////////////////////
    /// x
    pub fn new() -> Camera2D {
        Camera2D {
            x: 0.0,
            y: 0.0,
            zoom_x: 1.0,
            zoom_y: 1.0,
        }
    }

    ////////////////////////////////////////////////////////////
    /// x
    pub fn cam2world(&self, cx: f32, cy:f32) -> (f32,f32) {
        (
            cx/self.zoom_x + self.x,
            cy/self.zoom_y + self.y
        )
    }


    ////////////////////////////////////////////////////////////
    /// x
    pub fn world2cam(&self, wx: f32, wy:f32) -> (f32,f32) {
        (
            (wx-self.x)*self.zoom_x,
            (wy-self.y)*self.zoom_y
        )
    }


    /* 
    ////////////////////////////////////////////////////////////
    /// x
    pub fn fit_umap(&mut self, umap: &UmapData) {
        self.x = (umap.min_x + umap.max_x)/2.0;
        self.y = (umap.min_y + umap.max_y)/2.0;

        let world_dx = umap.max_x - umap.min_x;
        let world_dy = umap.max_y - umap.min_y;

        let margin = 0.9;
        self.zoom_x = margin/(world_dx/2.0);
        self.zoom_y = margin/(world_dy/2.0);
    }

    */

    ////////////////////////////////////////////////////////////
    /// Zoom around this position.
    /// i.e. it should be in the same position in camera coordinates after zoom has been applied
    /// 
    /// world2cam(mouse_pos, zoom1) = world2cam(mouse_pos, zoom2)
    /// for: world2cam(wx,zoom_x) = (wx-cam_x)*zoom_x
    /// 
    /// Derivation:
    /// (wx-cam_x1)*zoom1 = (wx-cam_x2)*zoom2
    /// (wx-cam_x1)*zoom1/zoom2 = wx - cam_x2
    /// cam_x2 = wx - (wx-cam_x1)*zoom1/zoom2
    pub fn zoom_around(&mut self, wx: f32, wy: f32, scale: f32) {
        let zoom1_x = self.zoom_x;
        let zoom1_y = self.zoom_y;

        //Apply zoom
        self.zoom_x *= scale;
        self.zoom_y *= scale;

        //Correct position
        self.x = wx - (wx-self.x)*zoom1_x/self.zoom_x;
        self.y = wy - (wy-self.y)*zoom1_y/self.zoom_y;
    }

}




////////////////////////////////////////////////////////////
/// x
#[derive(Debug, PartialEq)]
pub struct Rectangle2D {
    pub x1: f32,
    pub x2: f32,
    pub y1: f32,
    pub y2: f32
}
impl Rectangle2D {
    pub fn range_x(&self) -> (f32, f32) {
        if self.x1<self.x2 {
            (self.x1,self.x2)
        } else {
            (self.x2,self.x1)
        }
    }

    pub fn range_y(&self) -> (f32, f32) {
        if self.y1<self.y2 {
            (self.y1,self.y2)
        } else {
            (self.y2,self.y1)
        }
    }
}


