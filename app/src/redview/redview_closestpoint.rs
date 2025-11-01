use std::collections::HashMap;

use crate::redview::ReductionViewData;


////////////////////////////////////////////////////////////
/// ID of a bucket
type SectorID = (i32,i32);

////////////////////////////////////////////////////////////
/// Point in index: x,y, ID
type IndexedPoint = (f32,f32,usize); 


////////////////////////////////////////////////////////////
/// Data structure for fast lookup of which point is closest to a given structure.
/// The system is simple: the world is divided up into square buckets, reducing
/// the number of points to be checked. This assumes that points beyond a certain
/// distance are not relevant.
pub struct ClosestPointIndex2D {
    sectors: HashMap<SectorID, Vec<IndexedPoint>>,
    max_dist: f32
}
impl ClosestPointIndex2D {

    ////////////////////////////////////////////////////////////
    /// Construct an empty index
    pub fn new() -> ClosestPointIndex2D {
        ClosestPointIndex2D {
            sectors: HashMap::new(),
            max_dist: 1.0 //do not do 0.0 to avoid division by 0
        }
    }

    ////////////////////////////////////////////////////////////
    /// Remove all points from the index
    pub fn clear(&mut self) {
        self.sectors.clear();

    }

    ////////////////////////////////////////////////////////////
    /// Get sector ID (bucket) for a given point
    pub fn get_sector_id(&self, x: f32, y: f32) -> SectorID {
        (
            ((x as f32)/self.max_dist) as i32,
            ((y as f32)/self.max_dist) as i32,
        )
    }

    ////////////////////////////////////////////////////////////
    /// From a reduction, place all points into their buckets
    pub fn build_point_index(&mut self, umap: &ReductionViewData, max_dist: f32) {
        self.clear();
        self.max_dist = max_dist;

        for i in 0..umap.num_point {
            let x = umap.data[i*2+0];
            let y: f32 = umap.data[i*2+1];

            let sector_id = self.get_sector_id(x,y);

            /*
            possible speedup
            self.sectors.raw_entry_mut()
                .from_key(sector_id)
                .or_insert_with(|| (sector_id, UmapPointIndexTree::new()));
 */

            let sector = self.sectors.get_mut(&sector_id);
            if let Some(sector) = sector {
                sector.push((x,y,i));
            } else {
                let mut sector = Vec::new();
                sector.push((x,y,i));
                self.sectors.insert(sector_id, sector);
            }
        }
    }



    ////////////////////////////////////////////////////////////
    /// Find the point closest to the given point, if any is close enough
    pub fn get_closest_point(&self, x:f32, y:f32) -> Option<usize> {

        //Scan all sectors around mouse for candidate points
        let (sector_mid_x,sector_mid_y) = self.get_sector_id(x,y);
        let mut list_cand = Vec::new();
        for sector_x in (sector_mid_x-1)..(sector_mid_x+2) {   //////////////////////// overflow here. 
            for sector_y in (sector_mid_y-1)..(sector_mid_y+2) {
                //Find closest point in sector
                if let Some(sector) = self.sectors.get(&(sector_x, sector_y)) {
                    let mut iter = sector.iter();

                    //First point
                    let (px,py,i) = iter.next().unwrap();
                    let mut best_i = *i;
                    let mut best_dist = dist2(x,y,  *px,*py);

                    //Remaining points
                    while let Some((px,py,i)) = iter.next() {
                        let this_dist = dist2(x,y,  *px,*py);
                        if this_dist < best_dist {
                            best_dist = this_dist;
                            best_i = *i;
                        }
                    }

                    list_cand.push((best_i, best_dist));
                }
            }
        }

        //If we got candidates...
        if list_cand.len()>0 {
            //Find distance to the nearest candidate
            let mut max=f32::MAX;
            let mut return_i = 0;
            for (cand_i, d2) in list_cand {
                if d2<max {
                    max=d2;
                    return_i=cand_i;
                }                
            }

            //See if this point is close enough
            if max < self.max_dist*self.max_dist {  // can remove this extra test
                Some(return_i)
            } else {
                None
            }

        } else {
            None
        }
    }

}


////////////////////////////////////////////////////////////
/// Compute length^2 of a 2d vector
fn dist2(x1:f32,y1:f32,   x2:f32,y2:f32) -> f32 {
        let dx = x1 - x2;
        let dy = y1 - y2;
        let dist2 = dx*dx + dy*dy;
        dist2
}
