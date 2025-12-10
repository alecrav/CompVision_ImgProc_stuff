use std::fs::OpenOptions;
use std::io::Write;

/// This function creates a ppm image, writing it into an appropriate file
pub fn create_ppm(width: i32, height: i32) {
    let mut f = match OpenOptions::new()
    .write(true)
    .append(true)
    .create(true)
    .open("./images/first.ppm") {
        Ok(f) => f,
        Err(e) => {
            eprintln!("could not create file: {}", e);
            return
        }
    };

    println!("This image has a resolution of {} x {} pixels", &width, &height);
    let mut r = 0.0;
    let mut g = 0.0;
    let mut b = 0.0;

    writeln!(f, "P3").unwrap();
    writeln!(f, "{} {}", width, height).unwrap();
    writeln!(f, "255").unwrap();
    for i in 0..height {
        for j in 0..width {
            r = i as f64 / (width  as f64 - 1.0) * 255.99;
            g = j as f64 / (height as f64 - 1.0) * 255.99;
            b = 0.0;


            
            if let Err(e) = writeln!(f, "{} {} {}", r, g, b) {
                eprintln!("couldn't write to file: {}", e)
            };
        }
    }
    println!("Wrote image");
}