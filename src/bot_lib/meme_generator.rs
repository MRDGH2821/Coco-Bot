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

/// Generates a meme by adding text to a template image with intelligent text wrapping and sizing
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

    // Text color (white with black outline)
    let white = Rgba([255u8, 255u8, 255u8, 255u8]);
    let black = Rgba([0u8, 0u8, 0u8, 255u8]);

    // Calculate available areas for text (leaving margins)
    let text_margin = (width as f32 * 0.05) as u32; // 5% margin on each side
    let max_text_width = width - (text_margin * 2);
    let max_text_height = (height as f32 * 0.2) as u32; // 20% of image height for each text area

    // Draw top text
    if !top_text.is_empty() {
        let top_text_upper = top_text.to_uppercase();
        let (wrapped_lines, font_size) = prepare_text_with_wrapping(
            &font,
            &top_text_upper,
            max_text_width,
            max_text_height,
            width,
        );

        let scale = PxScale::from(font_size);
        let line_height = (font_size * 1.2) as i32; // 120% of font size for line spacing
        let start_y = (height as f32 * 0.05) as i32;

        for (i, line) in wrapped_lines.iter().enumerate() {
            let text_width = calculate_text_width(&font, scale, line);
            let x = ((width as f32 - text_width) / 2.0).max(text_margin as f32) as i32;
            let y = start_y + (i as i32 * line_height);

            draw_text_with_outline(&mut rgba_img, &font, scale, line, x, y, white, black);
        }
    }

    // Draw bottom text
    if !bottom_text.is_empty() {
        let bottom_text_upper = bottom_text.to_uppercase();
        let (wrapped_lines, font_size) = prepare_text_with_wrapping(
            &font,
            &bottom_text_upper,
            max_text_width,
            max_text_height,
            width,
        );

        let scale = PxScale::from(font_size);
        let line_height = (font_size * 1.2) as i32; // 120% of font size for line spacing
        let total_text_height = (wrapped_lines.len() as i32 - 1) * line_height;
        let start_y = (height as f32 * 0.85) as i32 - total_text_height;
        /* jscpd:ignore-start */
        for (i, line) in wrapped_lines.iter().enumerate() {
            let text_width = calculate_text_width(&font, scale, line);
            let x = ((width as f32 - text_width) / 2.0).max(text_margin as f32) as i32;
            let y = start_y + (i as i32 * line_height);

            draw_text_with_outline(&mut rgba_img, &font, scale, line, x, y, white, black);
        }
    }
    /* jscpd:ignore-end */
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

/// Calculates the height of text when rendered
fn calculate_text_height(font: &FontRef, scale: PxScale) -> f32 {
    use ab_glyph::{Font, ScaleFont};

    let scaled_font = font.as_scaled(scale);
    scaled_font.ascent() - scaled_font.descent()
}

/// Wraps text to fit within specified dimensions and calculates optimal font size
fn prepare_text_with_wrapping(
    font: &FontRef,
    text: &str,
    max_width: u32,
    max_height: u32,
    image_width: u32,
) -> (Vec<String>, f32) {
    // Start with initial font size based on image width
    let mut font_size = (image_width as f32 * 0.08).max(20.0);
    let min_font_size = 12.0;
    let max_font_size = image_width as f32 * 0.15;

    // Clamp initial font size
    font_size = font_size.min(max_font_size).max(min_font_size);

    loop {
        let scale = PxScale::from(font_size);
        let line_height = calculate_text_height(font, scale) * 1.2; // 120% spacing

        // Try to wrap text with current font size
        let wrapped_lines = wrap_text_to_lines(font, scale, text, max_width as f32);
        let total_height = wrapped_lines.len() as f32 * line_height;

        // Check if text fits within height constraints
        if total_height <= max_height as f32 || font_size <= min_font_size {
            return (wrapped_lines, font_size);
        }

        // Reduce font size and try again
        font_size = (font_size * 0.9).max(min_font_size);
    }
}

/// Wraps text into multiple lines to fit within the specified width
fn wrap_text_to_lines(font: &FontRef, scale: PxScale, text: &str, max_width: f32) -> Vec<String> {
    let words: Vec<&str> = text.split_whitespace().collect();
    if words.is_empty() {
        return vec![String::new()];
    }

    let mut lines = Vec::new();
    let mut current_line = String::new();

    for word in words {
        let test_line = if current_line.is_empty() {
            word.to_string()
        } else {
            format!("{} {}", current_line, word)
        };

        let line_width = calculate_text_width(font, scale, &test_line);

        if line_width <= max_width {
            current_line = test_line;
        } else {
            // Current line is too wide, start a new line
            if !current_line.is_empty() {
                lines.push(current_line);
                current_line = word.to_string();
            } else {
                // Single word is too wide, break it down
                current_line = word.to_string();
            }
        }
    }

    if !current_line.is_empty() {
        lines.push(current_line);
    }

    // If no lines were created, return the original text
    if lines.is_empty() {
        lines.push(text.to_string());
    }

    lines
}

/// Draws text with a black outline for better visibility
fn draw_text_with_outline(
    image: &mut image::ImageBuffer<image::Rgba<u8>, Vec<u8>>,
    font: &FontRef,
    scale: PxScale,
    text: &str,
    x: i32,
    y: i32,
    text_color: Rgba<u8>,
    outline_color: Rgba<u8>,
) {
    // Draw black outline
    for dx in -2..=2 {
        for dy in -2..=2 {
            if dx != 0 || dy != 0 {
                draw_text_mut(image, outline_color, x + dx, y + dy, scale, font, text);
            }
        }
    }
    // Draw main text
    draw_text_mut(image, text_color, x, y, scale, font, text);
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
pub fn save_meme(
    meme: &DynamicImage,
    output_path: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
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
