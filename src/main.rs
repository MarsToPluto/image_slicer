use image::{DynamicImage, GenericImageView, ImageBuffer, Rgba};
use rayon::prelude::*;
use std::collections::HashMap;
use std::fs;
use std::io::Write; // Import the Write trait
use std::path::{Path, PathBuf};
use std::sync::{Arc, atomic::{AtomicU32, Ordering}};

// Constants
const PIXEL_SIZE: u32 = 16; // Size of the pixel images to be saved

fn rgba_to_hex(color: Rgba<u8>) -> String {
    format!("#{:02X}{:02X}{:02X}", color[0], color[1], color[2])
}

fn save_pixel_as_image(pixel: Rgba<u8>, x: u32, y: u32, output_dir: &Path) {
    let img = ImageBuffer::from_fn(PIXEL_SIZE, PIXEL_SIZE, |_, _| pixel);
    let output_path = output_dir.join(format!("pixel_{}_{}.png", x, y));
    img.save(output_path).expect("Failed to save pixel image");
}

fn slice_image_into_pixels(img_path: &str) {
    // Load the image
    let img = image::open(img_path).expect("Failed to open image");

    // Extract image dimensions and other properties
    let (width, height) = img.dimensions();
    let color_type = img.color();
    
    // Print debugging information about the image
    println!("Image Dimensions: {} x {}", width, height);
    println!("Color Type: {:?}", color_type);
    println!("Total Pixels: {}", width * height);
    
    // Create the output directory if it doesn't exist
    let output_dir = Path::new("src/sliced");
    fs::create_dir_all(output_dir).expect("Failed to create output directory");

    // Calculate the number of pixel images based on PIXEL_SIZE
    let num_x_images = width / PIXEL_SIZE;
    let num_y_images = height / PIXEL_SIZE;
    let total_pixel_images = (num_x_images * num_y_images) as u32;

    // Print the total number of pixel images to be processed
    println!("Total pixel images to be processed: {}", total_pixel_images);

    // Create a vector to hold pixel coordinates
    let pixel_coords: Vec<(u32, u32)> = (0..num_y_images)
        .flat_map(|y| (0..num_x_images).map(move |x| (x * PIXEL_SIZE, y * PIXEL_SIZE)))
        .collect();

    // Create an atomic counter to track progress
    let counter = Arc::new(AtomicU32::new(0));

    // Start processing pixels in parallel
    pixel_coords.par_iter().for_each(|&(x, y)| {
        let pixel = img.get_pixel(x, y).clone(); // Clone the pixel
        save_pixel_as_image(pixel, x / PIXEL_SIZE, y / PIXEL_SIZE, &output_dir);

        // Update progress
        let current_saved_count = counter.fetch_add(1, Ordering::SeqCst) + 1; // Increment and get current count

        // Calculate progress and display it
        let progress = (current_saved_count as f64) / total_pixel_images as f64; // Calculate progress
        let percentage = (progress * 100.0).round() as u32;

        // Simple progress bar logic
        let bar_length = 40; // Length of the progress bar
        let filled_length = (bar_length as f64 * progress).round() as usize;
        let bar = "█".repeat(filled_length) + &" ".repeat(bar_length - filled_length);

        // Print progress with the count of images saved
        print!("\rProgress: [{}] {}% - {} of {} images saved", bar, percentage, current_saved_count, total_pixel_images);
        std::io::stdout().flush().unwrap(); // Ensure the output is flushed
    });

    // Print the completion message with total files saved
    println!("\nSlicing completed, saved {} pixel images to {:?}", total_pixel_images, output_dir);
}

fn find_top_x_colors(img_path: &str) {
    let top_x = 5;
    let img = image::open(Path::new(img_path)).expect("Failed to open image");
    let mut color_counts: HashMap<Rgba<u8>, u32> = HashMap::new();
    
    for (_, _, pixel) in img.pixels() {
        *color_counts.entry(pixel).or_insert(0) += 1;
    }
    
    let mut sorted_colors: Vec<(Rgba<u8>, u32)> = color_counts.into_iter().collect();
    sorted_colors.sort_by(|a, b| b.1.cmp(&a.1)); // Sort by count, descending
    
    println!("Top {} colors:", top_x);
    for (color, count) in sorted_colors.iter().take(top_x) {
        let hex = rgba_to_hex(*color);
        println!("Color: {}, Count: {}", hex, count);
    }
}

fn main() {
    let img_path = "src/2.webp"; // Change this to your image path
    slice_image_into_pixels(img_path);
    // Uncomment below line to find top colors after slicing
    // find_top_x_colors(img_path); 
}
