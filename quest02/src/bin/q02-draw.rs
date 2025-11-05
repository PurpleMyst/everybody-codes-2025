use std::fs::File;
use std::io::Write;

use quest02::{Complex, parse_a};

fn turbo(value: u8) -> [u8 ; 3] {
    let x = value as f64 / 99.0;

    // Polynomial approximation of Turbo colormap (by Anton Mikhailov, Google)
    let r = (34.61 + x * (1172.33 + x * (-10793.56 + x * (33300.12 + x * (-38394.49 + x * 14825.05))))).clamp(0.0, 255.0);
    let g = (23.31 + x * (557.33 + x * (1225.33 + x * (-3574.96 + x * (4479.12 + x * (-1747.72)))))).clamp(0.0, 255.0);
    let b = (27.2 + x * (3211.1 + x * (-15327.97 + x * (27814.0 + x * (-22569.18 + x * 6832.95))))).clamp(0.0, 255.0);

    [r as u8, g as u8, b as u8]
}

fn main() {
    let a = parse_a(include_str!("../part3.txt"));
    let mut f = File::create("part3_image.pbm").unwrap();
    writeln!(f, "P6").unwrap();
    writeln!(f, "# q02-draw generated").unwrap();
    writeln!(f, "1001 1001").unwrap();
    writeln!(f, "255").unwrap();

    for y in 0..=1000 {
        'cell: for x in 0..=1000 {
            let b = Complex::new(a.real + x, a.imag + y);

            let mut check_result = Complex::zero();
            let divisor = Complex::new(100000, 100000);
            for step in 0..100 {
                check_result = (check_result * check_result) / divisor + b;
                if check_result.real.abs() > 1000000 || check_result.imag.abs() > 1000000 {
                    let color = turbo(step);
                    f.write_all(&color).unwrap();
                    continue 'cell;
                }
            }
            f.write_all(&[0, 0, 0]).unwrap();
        }
    }
}
