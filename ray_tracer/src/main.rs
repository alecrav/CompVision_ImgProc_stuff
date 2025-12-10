mod ppm;

fn main() {
    println!("Creating PPM image");
    ppm::create_ppm(400, 600);
}
