# Image Slicer

## Description
Image Slicer is a Rust-based application that allows users to slice an image into smaller pixel-sized images. Each pixel is saved as an individual image file, making it easy to analyze or manipulate pixel data. The application leverages multi-threading for efficient processing, allowing it to handle large images quickly.

## Features
- **Parallel Processing**: Utilizes the Rayon library to process pixel slicing concurrently, significantly reducing processing time.
- **Image Info Logging**: Provides detailed information about the input image, including dimensions, color type, and total pixel count.
- **Customizable Pixel Size**: Users can adjust the size of the pixels to be sliced by modifying the `PIXEL_SIZE` constant.
- **Output Directory**: Automatically creates an output directory for sliced images, ensuring organized storage.
- **Progress Tracking**: Displays a progress bar in the console, indicating the percentage of images processed and the current count of saved images.

## Requirements
- Rust programming language
- Image crate for image manipulation
- Rayon crate for parallel processing
