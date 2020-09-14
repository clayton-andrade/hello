use std::fmt::{self, Display, Formatter, Result};

#[derive(Debug)]
struct MinMax(i64, i64);

impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{x: {}, y: {}}}", self.x, self.y)
    }
}

#[derive(Debug)]
struct Complex {
    real: f64,
    imag: f64
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} + {}i", self.real, self.imag)        
    }
}

#[derive(Debug)]
struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let vec = &self.0;

        write!(f, "[")?;

        for (i, e) in vec.iter().enumerate() {
            if i != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}: {}", i, e)?;
        }

        write!(f, "]")
    }
}

struct City<'city> {
    name: &'city str,
    // Latitude
    lat: f32,
    // Longitude
    lon: f32
}

impl <'city> Display for City<'city> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let lat_c = if self.lat >= 0.0 {'N'} else {'S'};
        let lon_c = if self.lon >= 0.0 {'E'} else {'W'};

        write!(f, "{}: {:.3}° {} {:.3}° {}",
               self.name, self.lat.abs(), lat_c, self.lon.abs(), lon_c)
    }
}

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "RGB({r}, {g}, {b}) 0x{r:02X}{g:02X}{b:02X}",
               r = self.red, g = self.green, b = self.blue)
    }
}

fn main() {
    let minmax = MinMax(0, 14);
    
    println!("Compare structures:");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);

    let big_range = MinMax(-300, 300);
    let small_range = MinMax(-3, 3);

    println!("The big range is {big} and the small range is {small}",
             big = big_range, small = small_range);

    let point = Point2D {x: 3.3, y:7.2};

    println!("Compare points.");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);

    let complex = Complex {real: 3.3, imag: 7.2};

    println!("Compare complex.");
    println!("Display: {}", complex);
    println!("Debug: {:?}", complex);

    let list = List(vec![1, 2, 3, 4, 5]);

    println!("{}", list);

    let cities = [City {name: "Dublin", lat: 53.347778, lon: -6.259722},
                  City {name: "Oslo", lat: 59.95, lon: 10.75},
                  City {name: "Vancouver", lat: 49.25, lon: -123.1}];
    
    for city in &cities {
        println!("{}", city);
    }

    for color in [Color { red: 128, green: 255, blue: 90 },
                  Color { red: 0, green: 3, blue: 254 },
                  Color { red: 0, green: 0, blue: 0 }
    ].iter() {
        println!("{:?}", color);
    }

    for color in [Color { red: 128, green: 255, blue: 90 },
                  Color { red: 0, green: 3, blue: 254 },
                  Color { red: 0, green: 0, blue: 0 }
    ].iter() {
        println!("{}", color);
    }
}