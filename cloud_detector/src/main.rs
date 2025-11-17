#[cfg(feature = "blosc")]
use hdf5::filters::blosc_set_nthreads;
use hdf5::{File, H5Type, Result as H5Result};
use std::io::Result as MainResult;
use anyhow::Result;
use ndarray::{Array4, s, Axis};

/**
    Implement quick Braaten-Cohen-Yang cloud detector
    author: @acraviogli
**/

// read hd5 file
fn read_hdf5() -> Result<()> {
    let file = File::open("../data/s2_test.h5")?; // open for reading
    
    // access datasets inside .h5 file and print some useful info
    println!("Top-level members:");
    for name in file.member_names()? {
        println!("  {}", name);
        if let Ok(ds) = file.dataset(&name) {
            // print information (useful for info on tiles etc)
            println!("Dataset: {}", name);
            println!("  Shape: {:?}", ds.shape());
            println!("  Type: {:?}", ds.dtype());
        }
    }

    // work on data
    // now work with sample dataset
    let ds = file.dataset("s2_18")?;
        
    // decide batch size for processing and save total number of tiles
    let batch_size = 256;
    let num_tiles = ds.shape()[0];

    // for each batch of tiles extract rgb channels
    println!("--------");
    println!("  Shape: {:?}", ds.shape());
    println!("  Type: {:?}", ds.dtype());
    
    // iterate through slice and do the stuff
    // label each pixel (?)
    // Inspect attributes if present
    // create buffer for 1 slice
    let mut buf: Array4<u8> = Array4::zeros((1, ds.shape()[1], ds.shape()[2], ds.shape()[3]));
    let tile_index = 0;
    let tile: Array4<u8> = ds.read_slice(s![tile_index..tile_index+1, .., .., ..])?;
    // drop the first dimension
    let tile3d = tile.index_axis(Axis(0), 0);
    
    let mut i = 0;
    while i < ds.shape()[1] {
        let mut j = 0;
        while j < ds.shape()[2] {
            let r = tile3d[[i, j, 0]];
            let g = tile3d[[i, j, 1]];
            let b = tile3d[[i, j, 2]];
            println!("{} {} {}", r, g, b);
            j += 1;
        }
        i += 1;
    }

    let orig_shape = ds.attr("orig_shape")?;
    let tile_size = ds.attr("tile_size")?;
    let order = ds.attr("order")?;


    Ok(())
}

fn main() -> std::io::Result<()> {
    println!("We all love low level programming languages");

    let path = env::current_dir()?;
    println!("The current directory is {}", path.display());

    read_hdf5().map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    Ok(())
}