// use image::{GrayImage, Luma};
// use imageproc::contours::{find_contours, Contour};
// use imageproc::contrast::{threshold, ThresholdType};
// use imageproc::drawing::draw_polygon_mut;
// use imageproc::point::Point;
// use std::path::Path;

#[tauri::command]
pub fn contour(img_path: &str) {
    println!("Hello, {img_path}");
    // let img = image::open(&Path::new(img_path))
    //     .expect("Failed to open image")
    //     .to_luma8();

    // let binary_img = threshold(&img, 128, ThresholdType::BinaryInverted);

    // let contours: Vec<Contour<i32>> = find_contours(&binary_img);

    // for (i, contour) in contours.into_iter().enumerate() {
    //     println!("masking {}", i);
    //     let mut mask = GrayImage::new(binary_img.width(), binary_img.height());

    //     let polygon_points: Vec<Point<i32>> = contour
    //         .points
    //         .iter()
    //         .map(|p| Point::new(p.x, p.y))
    //         .collect();

    //     draw_polygon_mut(&mut mask, &polygon_points, Luma([255]));

    //     let mask_path = format!("output/mask_{:0>3}.png", i);
    //     mask.save(mask_path).expect("Failed to save mask");
    // }

    // println!("complete");
}
