use flate2::read::GzDecoder;
use my_web_app::gbrowser_struct::GBrowserGFF;
use my_web_app::gbrowser_struct::GBrowserRecordBuf;
use noodles::gff::feature::RecordBuf;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use noodles::gtf;
use noodles::gff;

use crate::gbrowser_noodles::convert_record;



pub struct GFFparseSettings {
}



//TODO take GBrowserGFF as argument, with tracks set up



////////////////////////////////////////////////////////////
/// 
pub struct FeatureCollection {
}
impl FeatureCollection {



    ////////////////////////////////////////////////////////////
    /// x
    pub fn make_default_gff() -> GBrowserGFF {
        let mut gff = GBrowserGFF::new();
        gff.add_track(1000000);  //1 mb
        gff.add_track(10000000); //10 mb
        gff
    }    

    ////////////////////////////////////////////////////////////
    /// For GFF/GTF reading, process one record
    fn add_gene_record(gff: &mut GBrowserGFF, _params: &GFFparseSettings, record: &RecordBuf) {

        let newrec: GBrowserRecordBuf = convert_record(record);
        gff.add_record(newrec);

    }

    ////////////////////////////////////////////////////////////
    /// Read a GFF file - from a reader
    fn read_gff_from_reader<R>(
        gff: &mut GBrowserGFF,
        reader: &mut gff::io::Reader<R>,
        params: &GFFparseSettings,
    ) -> anyhow::Result<()>
    where
        R: std::io::BufRead,
    {
        //let mut gff = GBrowserGFF::new();
        for result in reader.record_bufs() {
            let record = result.expect("Could not read a GFF record; is it actually a GTF?");
            Self::add_gene_record(gff, params, &record);
        }
        anyhow::Ok(())
    }

    ////////////////////////////////////////////////////////////
    /// Read a GTF file - from a reader
    ///
    fn read_gtf_from_reader<R>(
        gff: &mut GBrowserGFF,
        reader: &mut gtf::io::Reader<R>,
        params: &GFFparseSettings,
    ) -> anyhow::Result<()>
    where
        R: std::io::BufRead,
    {
        //let mut gff = GBrowserGFF::new();
        for result in reader.record_bufs() {
            let record = result.expect("Could not read a GFF record; is it actually a GTF?");
            Self::add_gene_record(gff, params, &record);
        }
        anyhow::Ok(())
    }

    ////////////////////////////////////////////////////////////
    /// Read a GFF-like file
    ///
    pub fn read_file(
        gff: &mut GBrowserGFF,
        path_gff: &PathBuf,
        params: &GFFparseSettings,
    ) -> anyhow::Result<()> {
        let spath = path_gff.to_string_lossy();

        if spath.ends_with("gff.gz") {
            println!("Reading gzipped GFF: {:?}", path_gff);
            let mut reader = File::open(&path_gff)
                .map(GzDecoder::new)
                .map(BufReader::new)
                .map(gff::io::Reader::new)?;
            Self::read_gff_from_reader(gff, &mut reader, params)
        } else if spath.ends_with("gff") {
            println!("Reading flat GFF: {:?}", path_gff);
            let mut reader = File::open(&path_gff)
                .map(BufReader::new)
                .map(gff::io::Reader::new)?;
            Self::read_gff_from_reader(gff, &mut reader, params)
        } else if spath.ends_with("gtf.gz") {
            println!("Reading gzipped GTF: {:?}", path_gff);
            let mut reader = File::open(&path_gff)
                .map(GzDecoder::new)
                .map(BufReader::new)
                .map(gtf::io::Reader::new)?;
            Self::read_gtf_from_reader(gff, &mut reader, params)
        } else if spath.ends_with("gtf") {
            println!("Reading gzipped GTF: {:?}", path_gff);
            let mut reader = File::open(&path_gff)
                .map(BufReader::new)
                .map(gtf::io::Reader::new)?;
            Self::read_gtf_from_reader(gff, &mut reader, params)
        } else {
            anyhow::bail!("Could not tell file format for GFF/GTF file {:?}", path_gff);
        }?;


        //See how well it worked
        let track_lens = gff.tracks.iter().map(|t| t.records.len()).collect::<Vec<_>>();
        println!("Done reading GFF; number of chunks in each track: {:?};  remainder bin features: {}", track_lens, gff.remainder.len());
        
        anyhow::Ok(())
    }

    /*
    https://gmod.org/wiki/GFF3

    OUR GFF
    NC_006153.2	RefSeq	gene	56826	58085	.	+	.	ID=gene-YPTB_RS21810;Name=yscD;gbkey=Gene;gene=yscD;gene_biotype=protein_coding;locus_tag=YPTB_RS21810;old_locus_tag=pYV0080
    NC_006153.2	Protein Homology	CDS	56826	58085	.	+	0	ID=cds-WP_002212919.1;Parent=gene-YPTB_RS21810;Dbxref=GenBank:WP_002212919.1;Name=WP_002212919.1;gbkey=CDS;gene=yscD;inference=COORDINATES: similar to AA sequence:RefSeq:WP_002212919.1;locus_tag=YPTB_RS21810;product=SctD family type III secretion system inner membrane ring subunit YscD;protein_id=WP_002212919.1;transl_table=11

    BASIC GFF
    ctg123 . mRNA            1300  9000  .  +  .  ID=mrna0001;Name=sonichedgehog
    ctg123 . exon            1300  1500  .  +  .  Parent=mrna0001
    */

    /*
        use noodles_gtf as gtf;
    let reader = gtf::io::Reader::new(io::empty());
    let _ = reader.get_ref();
    */
}
