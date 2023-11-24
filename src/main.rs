use image::GenericImageView;

fn main() {
    // Use the open function to load an image from a Path.
    // `open` returns a `DynamicImage` on success.
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("[USAGE] imgfile");
        return;
    }

    let img = image::open(&args[1]).unwrap();


        // The dimensions method returns the images width and height.
        println!("file:{} dimensions:{:?}", imgfile, img.dimensions());
}
