use bstr::BString;
use my_web_app::gbrowser_struct::GBrowserAttributeValue;
use my_web_app::gbrowser_struct::GBrowserPhase;
use my_web_app::gbrowser_struct::GBrowserRecordBuf;
use my_web_app::gbrowser_struct::GBrowserStrand;
use noodles::gff::feature::RecordBuf;
use noodles::gff::feature::record::Attributes;
use noodles::gff::feature::record::Phase;
use noodles::gff::feature::record::Strand;
use std::collections::HashMap;
use std::ops::Deref;


// Noodles do not have Serde serialization for the records. We convert to our own data structure
// to enable this. Having our own structures also means that Noodles need not be included on the
// client side, reducing size

////////////////////////////////////////////////////////////
/// Convert Noodles Phase
fn convert_phase(s: Phase) -> GBrowserPhase {
    match s {
        Phase::Zero => GBrowserPhase::Zero,
        Phase::One => GBrowserPhase::One,
        Phase::Two => GBrowserPhase::Two,
    }
}

////////////////////////////////////////////////////////////
///Convert Noodles Strand
fn convert_strand(s: Strand) -> GBrowserStrand {
    match s {
        Strand::None => GBrowserStrand::None,
        Strand::Forward => GBrowserStrand::Forward,
        Strand::Reverse => GBrowserStrand::Reverse,
        Strand::Unknown => GBrowserStrand::Unknown,
    }
}


////////////////////////////////////////////////////////////
/// Convert Noodles record
pub fn convert_record(r: &RecordBuf) -> GBrowserRecordBuf {

    let mut attr:HashMap<BString, GBrowserAttributeValue> = HashMap::new();

    for a in r.attributes().iter() {
        let (k,v)=a.unwrap();
        let k = k.deref();
        if let Some(v) = v.as_string() {
            attr.insert(k.into(), GBrowserAttributeValue::String(v.into()));
        }
    }
    
    GBrowserRecordBuf {
        reference_sequence_name: r.reference_sequence_name().into(), 
        ty: r.ty().into(),
        start: r.start().get() as u64,
        end: r.end().get() as u64,
        strand: convert_strand(r.strand()),
        phase: r.phase().map(|x| convert_phase(x)),
        attributes: attr,
    }
}

