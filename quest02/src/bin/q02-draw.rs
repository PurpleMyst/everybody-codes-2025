use std::{fs::File, io::Write};

use quest02::{Complex, parse_a};

fn turbo(value: u8) -> [u8; 3] {
    // Account for empirical bounds
    let x = (value as f64 - 24.0) / (99.0 - 24.0);

    // Powers
    let x2 = x * x;
    let x3 = x2 * x;
    let x4 = x2 * x2;
    let x5 = x3 * x2;

    // Coefficients from the GLSL reference implementation
    let r = 0.13572138 + 4.61539260 * x - 42.66032258 * x2 + 132.13108234 * x3 - 152.94239396 * x4 + 59.28637943 * x5;

    let g = 0.09140261 + 2.19418839 * x + 4.84296658 * x2 - 14.18503333 * x3 + 4.27729857 * x4 + 2.82956604 * x5;

    let b = 0.10667330 + 12.64194608 * x - 60.58204836 * x2 + 110.36276771 * x3 - 89.90310912 * x4 + 27.34824973 * x5;

    [
        (r.clamp(0.0, 1.0) * 255.0).round() as u8,
        (g.clamp(0.0, 1.0) * 255.0).round() as u8,
        (b.clamp(0.0, 1.0) * 255.0).round() as u8,
    ]
}

fn main() {
    draw(include_str!("../part2.txt"), "p2.pbm", 10);
    draw(include_str!("../part3.txt"), "p3.pbm", 1);
}

fn draw(input: &'static str, out_path: &'static str, step: i64) {
    let a = parse_a(input);
    let mut f = File::create(out_path).unwrap();
    writeln!(f, "P6").unwrap();
    writeln!(f, "# q02-draw generated").unwrap();
    let side = 1000 / step + 1;
    writeln!(f, "{0} {0}", side).unwrap();
    writeln!(f, "255").unwrap();

    let mut points = Vec::new();

    for y in (0..=1000).step_by(step as usize) {
        'cell: for x in (0..=1000).step_by(step as usize) {
            let b = Complex::new(a.real + x, a.imag + y);

            let mut check_result = Complex::zero();
            let divisor = Complex::new(100000, 100000);
            for step in 0..100 {
                check_result = (check_result * check_result) / divisor + b;
                if check_result.real.abs() > 1000000 || check_result.imag.abs() > 1000000 {
                    points.push(Some(step));
                    continue 'cell;
                }
            }
            points.push(None);
        }
    }

    eprintln!("Drawing to {}", out_path);
    eprintln!("Total points: {}", points.len());
    eprintln!("Engraved points: {}", points.iter().filter(|p| p.is_none()).count());
    eprintln!(
        "Minimum step for escaped points: {}",
        points.iter().filter_map(|p| *p).min().unwrap_or(0)
    );
    eprintln!(
        "Maximum step for escaped points: {}",
        points.iter().filter_map(|p| *p).max().unwrap_or(0)
    );

    for maybe_step in points {
        f.write_all(&maybe_step.map_or([0, 0, 0], turbo)).unwrap();
    }
}
