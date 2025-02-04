use rs_paths::algorithms::combine_segments;
use rs_paths::algorithms::{ellipse_parameters, fit_ellipse};
use rs_paths::geom::*;
use rs_paths::path;
use rs_paths::path::*;

pub fn main() {
    let path = path![
        Line::new((12.00, 20.00), (12.00, 20.04)),
        Line::new((12.00, 20.04), (11.98, 20.09)),
        Line::new((11.98, 20.09), (11.96, 20.13)),
        Line::new((11.96, 20.13), (11.94, 20.17)),
        Line::new((11.94, 20.17), (11.90, 20.22)),
        Line::new((11.90, 20.22), (11.86, 20.26)),
        Line::new((11.86, 20.26), (11.81, 20.30)),
        Line::new((11.81, 20.30), (11.75, 20.34)),
        Line::new((11.75, 20.34), (11.69, 20.38)),
        Line::new((11.69, 20.38), (11.62, 20.41)),
        Line::new((11.62, 20.41), (11.54, 20.45)),
        Line::new((11.54, 20.45), (11.46, 20.48)),
        Line::new((11.46, 20.48), (11.37, 20.51)),
        Line::new((11.37, 20.51), (11.27, 20.54)),
        Line::new((11.27, 20.54), (11.18, 20.57)),
        Line::new((11.18, 20.57), (11.07, 20.59)),
        Line::new((11.07, 20.59), (10.96, 20.61)),
        Line::new((10.96, 20.61), (10.85, 20.63)),
        Line::new((10.85, 20.63), (10.74, 20.65)),
        Line::new((10.74, 20.65), (10.62, 20.67)),
        Line::new((10.62, 20.67), (10.50, 20.68)),
        Line::new((10.50, 20.68), (10.37, 20.69)),
        Line::new((10.37, 20.69), (10.25, 20.69)),
        Line::new((10.25, 20.69), (10.13, 20.70)),
        Line::new((10.13, 20.70), (10.00, 20.70)),
        Line::new((10.00, 20.70), (9.87, 20.70)),
        Line::new((9.87, 20.70), (9.75, 20.69)),
        Line::new((9.75, 20.69), (9.63, 20.69)),
        Line::new((9.63, 20.69), (9.50, 20.68)),
        Line::new((9.50, 20.68), (9.38, 20.67)),
        Line::new((9.38, 20.67), (9.26, 20.65)),
        Line::new((9.26, 20.65), (9.15, 20.63)),
        Line::new((9.15, 20.63), (9.04, 20.61)),
        Line::new((9.04, 20.61), (8.93, 20.59)),
        Line::new((8.93, 20.59), (8.82, 20.57)),
        Line::new((8.82, 20.57), (8.73, 20.54)),
        Line::new((8.73, 20.54), (8.63, 20.51)),
        Line::new((8.63, 20.51), (8.54, 20.48)),
        Line::new((8.54, 20.48), (8.46, 20.45)),
        Line::new((8.46, 20.45), (8.38, 20.41)),
        Line::new((8.38, 20.41), (8.31, 20.38)),
        Line::new((8.31, 20.38), (8.25, 20.34)),
        Line::new((8.25, 20.34), (8.19, 20.30)),
        Line::new((8.19, 20.30), (8.14, 20.26)),
        Line::new((8.14, 20.26), (8.10, 20.22)),
        Line::new((8.10, 20.22), (8.06, 20.17)),
        Line::new((8.06, 20.17), (8.04, 20.13)),
        Line::new((8.04, 20.13), (8.02, 20.09)),
        Line::new((8.02, 20.09), (8.00, 20.04)),
        Line::new((8.00, 20.04), (8.00, 20.00)),
        Line::new((8.00, 20.00), (8.00, 19.96)),
        Line::new((8.00, 19.96), (8.02, 19.91)),
        Line::new((8.02, 19.91), (8.04, 19.87)),
        Line::new((8.04, 19.87), (8.06, 19.83)),
        Line::new((8.06, 19.83), (8.10, 19.78)),
        Line::new((8.10, 19.78), (8.14, 19.74)),
        Line::new((8.14, 19.74), (8.19, 19.70)),
        Line::new((8.19, 19.70), (8.25, 19.66)),
        Line::new((8.25, 19.66), (8.31, 19.62)),
        Line::new((8.31, 19.62), (8.38, 19.59)),
        Line::new((8.38, 19.59), (8.46, 19.55)),
        Line::new((8.46, 19.55), (8.54, 19.52)),
        Line::new((8.54, 19.52), (8.63, 19.49)),
        Line::new((8.63, 19.49), (8.73, 19.46)),
        Line::new((8.73, 19.46), (8.82, 19.43)),
        Line::new((8.82, 19.43), (8.93, 19.41))
    ];

    // Fit ellipse to points
    let a = fit_ellipse(&path.to_points());

    // Extract ellipse parameters
    let (x0, y0, semi_major_axis, semi_minor_axis, theta) = ellipse_parameters(a);

    println!("Center: ({:.2}, {:.2})", x0, y0);
    println!("Semi-major axis: {:.2}", semi_major_axis);
    println!("Semi-minor axis: {:.2}", semi_minor_axis);
    println!("Rotation angle: {:.2} radians", theta);

    // Plot the points (if desired)
}
