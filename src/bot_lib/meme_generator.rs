use ab_glyph::{FontRef, PxScale};
use image::{DynamicImage, Rgba};
use imageproc::drawing::draw_text_mut;
use std::fs;
use std::io;
use std::path::Path;

/// Returns a list of image file names from the meme_templates directory
///
/// # Returns
///
/// A `Result` containing a vector of file names (as `String`) on success,
/// or an `io::Error` if the directory cannot be read.
///
/// # Examples
///
/// ```
/// let template_files = get_meme_template_files().unwrap();
/// for file in template_files {
///     println!("Found template: {}", file);
/// }
/// ```
pub fn get_meme_template_files() -> io::Result<Vec<String>> {
    let templates_dir = Path::new("src/assets/meme_templates");

    // Check if directory exists
    if !templates_dir.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Meme templates directory not found",
        ));
    }

    let mut image_files = Vec::new();

    // Read directory entries
    let entries = fs::read_dir(templates_dir)?;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        // Only include files (not directories)
        if path.is_file() {
            if let Some(file_name) = path.file_name() {
                if let Some(name_str) = file_name.to_str() {
                    // Filter for common image file extensions
                    let name_lower = name_str.to_lowercase();
                    if name_lower.ends_with(".jpg")
                        || name_lower.ends_with(".jpeg")
                        || name_lower.ends_with(".png")
                        || name_lower.ends_with(".gif")
                        || name_lower.ends_with(".bmp")
                        || name_lower.ends_with(".webp")
                    {
                        image_files.push(name_str.to_string());
                    }
                }
            }
        }
    }

    // Sort the files alphabetically for consistent ordering
    image_files.sort();

    Ok(image_files)
}

/// Returns the full path to a specific meme template file
///
/// # Arguments
///
/// * `filename` - The name of the template file
///
/// # Returns
///
/// A `String` containing the full path to the template file
pub fn get_meme_template_path(filename: &str) -> String {
    format!("src/assets/meme_templates/{}", filename)
}

/// Generates a meme by adding text to a template image
///
/// # Arguments
///
/// * `template_filename` - The name of the template file to use
/// * `top_text` - Text to display at the top of the meme
/// * `bottom_text` - Text to display at the bottom of the meme
///
/// # Returns
///
/// A `Result` containing the generated meme as a `DynamicImage` on success,
/// or an error if the meme generation fails.
///
/// # Examples
///
/// ```
/// let meme = generate_meme("template.jpg", "TOP TEXT", "BOTTOM TEXT").unwrap();
/// ```
pub fn generate_meme(
    template_filename: &str,
    top_text: &str,
    bottom_text: &str,
) -> Result<DynamicImage, Box<dyn std::error::Error + Send + Sync>> {
    // Load the template image
    let template_path = get_meme_template_path(template_filename);
    let img = image::open(&template_path)?;

    // Convert to RGBA for text rendering
    let mut rgba_img = img.to_rgba8();
    let (width, height) = rgba_img.dimensions();

    // Load Impact font for classic meme style
    let font_data = include_bytes!("../assets/fonts/unicode-impact.ttf");
    let font = FontRef::try_from_slice(font_data).expect("Failed to load Impact font");

    // Calculate font size based on image width
    let font_size = (width as f32 * 0.08).max(20.0); // Minimum 20px, scale with image
    let scale = PxScale::from(font_size);

    // Text color (white with black outline)
    let white = Rgba([255u8, 255u8, 255u8, 255u8]);
    let black = Rgba([0u8, 0u8, 0u8, 255u8]);

    // Draw top text
    if !top_text.is_empty() {
        let top_text_upper = top_text.to_uppercase();
        let text_width = calculate_text_width(&font, scale, &top_text_upper);
        let x = ((width as f32 - text_width) / 2.0).max(0.0) as i32;
        let y = (height as f32 * 0.05) as i32;

        // Draw black outline
        for dx in -2..=2 {
            for dy in -2..=2 {
                if dx != 0 || dy != 0 {
                    draw_text_mut(
                        &mut rgba_img,
                        black,
                        x + dx,
                        y + dy,
                        scale,
                        &font,
                        &top_text_upper,
                    );
                }
            }
        }
        // Draw white text
        draw_text_mut(&mut rgba_img, white, x, y, scale, &font, &top_text_upper);
    }

    // Draw bottom text
    if !bottom_text.is_empty() {
        let bottom_text_upper = bottom_text.to_uppercase();
        let text_width = calculate_text_width(&font, scale, &bottom_text_upper);
        let x = ((width as f32 - text_width) / 2.0).max(0.0) as i32;
        let y = (height as f32 * 0.85) as i32;

        // Draw black outline
        for dx in -2..=2 {
            for dy in -2..=2 {
                if dx != 0 || dy != 0 {
                    draw_text_mut(
                        &mut rgba_img,
                        black,
                        x + dx,
                        y + dy,
                        scale,
                        &font,
                        &bottom_text_upper,
                    );
                }
            }
        }
        // Draw white text
        draw_text_mut(&mut rgba_img, white, x, y, scale, &font, &bottom_text_upper);
    }

    Ok(DynamicImage::ImageRgba8(rgba_img))
}

/// Calculates the approximate width of text when rendered
fn calculate_text_width(font: &FontRef, scale: PxScale, text: &str) -> f32 {
    use ab_glyph::{Font, ScaleFont};

    let scaled_font = font.as_scaled(scale);
    let mut width = 0.0;

    for c in text.chars() {
        let glyph = scaled_font.scaled_glyph(c);
        width += scaled_font.h_advance(glyph.id);
    }

    width
}

/// Saves a generated meme to a file
///
/// # Arguments
///
/// * `meme` - The generated meme image
/// * `output_path` - Path where the meme should be saved
///
/// # Returns
///
/// A `Result` indicating success or failure
pub fn save_meme(meme: &DynamicImage, output_path: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    meme.save(output_path)?;
    Ok(())
}

/// Generates a meme and saves it to a temporary file
///
/// # Arguments
///
/// * `template_filename` - The name of the template file to use
/// * `top_text` - Text to display at the top of the meme
/// * `bottom_text` - Text to display at the bottom of the meme
///
/// # Returns
///
/// A `Result` containing the file path to the generated meme on success,
/// or an error if the meme generation fails.
///
/// # Examples
///
/// ```
/// let meme_path = generate_meme_to_file("template.jpg", "TOP TEXT", "BOTTOM TEXT").unwrap();
/// ```
pub fn generate_meme_as_file_path(
    template_filename: &str,
    top_text: &str,
    bottom_text: &str,
) -> Result<std::path::PathBuf, Box<dyn std::error::Error + Send + Sync>> {
    // Generate the meme image
    let meme_image = generate_meme(template_filename, top_text, bottom_text)?;

    // Create a unique temporary file path
    let temp_dir = std::env::temp_dir();
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_secs();
    let temp_file_path = temp_dir.join(format!("meme_{}_{}.png", timestamp, std::process::id()));

    // Save the meme to the temporary file
    save_meme(&meme_image, temp_file_path.to_str().unwrap())?;

    Ok(temp_file_path)
}
