use std::path::PathBuf;

use anyhow::{anyhow, Result};
use image::GenericImageView;
use mozjpeg::Compress;

pub fn optimize(entry: &PathBuf, output: &PathBuf) -> Result<()> {
    let is_jpeg = entry.extension() == Some("jpg".as_ref());
    if !is_jpeg {
        unimplemented!()
    }

    let source = image::open(entry)?;

    let rgb_source = source
        .to_rgb8()
        .pixels()
        .flat_map(|x| x.0.to_vec())
        .collect::<Vec<_>>();

    let (width, height) = source.dimensions();

    let mut encoder = Compress::new(mozjpeg::ColorSpace::JCS_RGB);
    encoder.set_size(width as usize, height as usize);

    encoder.set_progressive_mode();
    encoder.set_scan_optimization_mode(mozjpeg::ScanMode::AllComponentsTogether);
    encoder.set_optimize_scans(true);
    encoder.set_optimize_coding(true);
    encoder.set_quality(88.);
    // encoder.dct_method(DctMethod::IntegerSlow); // patched [custom] crated, tried all variations
    encoder.set_mem_dest();
    encoder.start_compress();

    encoder
        .write_scanlines(rgb_source.as_slice())
        .then_some(())
        .ok_or(anyhow!("Failed to write all lines from source image"))?;

    encoder.finish_compress();

    let data = encoder.data_to_vec().unwrap();
    std::fs::write(output, data).unwrap();

    Ok(())
}
