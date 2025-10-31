use my_web_app::CountFileMetaColumnData;


////////////////////////////////////////////////////////////
/// Histogram for continuous data
#[derive(Debug, Clone)]
pub struct ContinuousFeatureHistogram {
    pub bin: Vec<f32>,
    pub count: Vec<u64>,
    pub total: u64,
    pub min: f32,
    pub max: f32,
    pub max_count: u64,
}


////////////////////////////////////////////////////////////
/// Histogram for categorical data
#[derive(Debug, Clone)]
pub struct CategoricalFeatureHistogram {
    pub category: Vec<String>,
    pub count: Vec<u64>,
    pub total: u64
}


////////////////////////////////////////////////////////////
/// Histogram for any type of data
#[derive(Debug, Clone)]
pub enum FeatureHistogram {
    ContinuousFeatureHistogram(ContinuousFeatureHistogram),
    CategoricalFeatureHistogram(CategoricalFeatureHistogram),
}
impl FeatureHistogram {


    ////////////////////////////////////////////////////////////
    /// Generate a histogram for any type of data
    pub fn build(inp: &CountFileMetaColumnData) -> FeatureHistogram {
        match inp {
            ///// Categorial data
            CountFileMetaColumnData::Categorical(list_data, list_cats) => {
                
                //Allocate an empty vector for counting
                let mut count: Vec<u64> = Vec::new();
                count.resize(list_cats.len(), 0);

                //Count all entries
                for v in list_data.iter() {
                    count[*v as usize] += 1;
                }
                
                FeatureHistogram::CategoricalFeatureHistogram(CategoricalFeatureHistogram {
                    category: list_cats.clone(),
                    count: count,
                    total: list_data.len() as u64,
                })
            },

            ///// Dense numeric array of data
            CountFileMetaColumnData::Numeric(list_data) => {
                make_histo_continuous_data(list_data)
            },

            ///// Sparse numeric array or data
            CountFileMetaColumnData::SparseNumeric(_list_indices, list_data) => {
                make_histo_continuous_data(list_data)
            },
        }
    }

}



////////////////////////////////////////////////////////////
/// Make a histogram of continuous data
fn make_histo_continuous_data(list_data: &Vec<f32>) -> FeatureHistogram { /////////////////////// for sparse: need to give total number of elements. store total size in the type!! 

    //log::debug!("compute hist {:?}", list_data);

    let num_bins = 30;
    
    //Figure out range of histogram
    let (minval, maxval) = make_safe_minmax(&list_data);
    //log::debug!("compute hist range {} {}", minval, maxval);


    //TODO: for sparse data, set minval=0 (or clamp!!)


    //Range and bin width
    let span = maxval - minval;
    let delta = span / (num_bins as f32);

    //Compute bin positions
    let mut bin: Vec<f32> = Vec::new();
    for i in 0..num_bins {
        bin.push((i as f32)*delta);                    
    }

    //Allocate an empty vector for counting
    let mut count: Vec<u64> = Vec::new();
    count.resize(num_bins, 0);

    //Fill bins
    let maxbin = (num_bins-1) as i32;
    let maxbin_f = maxbin as f32;
    for v in list_data.iter() {
        let binpos = (*v as f32 - minval)*maxbin_f/span;
        let binpos = binpos as i32;
        let binpos = binpos.clamp(0, maxbin);
        count[binpos as usize] += 1;
    }

    
    //Figure out peak height
    let max_count = *count.iter().max().unwrap();

    FeatureHistogram::ContinuousFeatureHistogram(ContinuousFeatureHistogram {
        bin: bin,
        count: count,
        total: list_data.len() as u64,
        min: minval,
        max: maxval,
        max_count: max_count,
    })
}



////////////////////////////////////////////////////////////
/// Find min and max values of a list of floats, even if list is empty
pub fn make_safe_minmax(list_data: &Vec<f32>) -> (f32,f32) {
    if list_data.is_empty() {
        (0.0,0.0)
    } else {
        let mut it = list_data.iter();
        let firstval = *it.next().unwrap();
        let mut minval=firstval;
        let mut maxval=firstval;
        for v in it {
            if *v < minval {
                minval = *v;
            }
            if *v > maxval {
                maxval = *v;
            }
        }
        (minval, maxval)
    }
}