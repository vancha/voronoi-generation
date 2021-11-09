use rand::Rng;
use std::cmp::Ordering;

const WIDTH: i32 = 800;
const HEIGHT: i32 = 600;

//the point struct that associates a color with a given coordinate in 2d space
#[derive(Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
    color: image::Rgb<u8>,
}

//implementing some equality operators for the struct, so that they can be compared and ordered
impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Eq for Point {}
impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        if other.x == self.x && other.y == self.y {
            Ordering::Equal
        } else if other.x < self.x && other.y < self.y {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    }
    fn max(self, other: Self) -> Self {
        match self.x > other.x && self.y > other.y {
            true => self,
            false => other,
        }
    }
    fn min(self, other: Self) -> Self {
        match self.x < other.x && self.y < other.y {
            true => self,
            false => other,
        }
    }
}

//adding some custom functions to the struct, like object oriented class methods
impl Point {
    //construct a new point, initializeing it's x and y to a random spot on the 2d plane, and give
    //it a random color
    fn new() -> Self {
        let mut random_numer_generator = rand::thread_rng();
        let grayscalevalue = random_numer_generator.gen_range(0..254) as u8;
        let color = image::Rgb([
            grayscalevalue,
            grayscalevalue,
            grayscalevalue,
            //random_numer_generator.gen_range(0..254) as u8,
            //random_numer_generator.gen_range(0..254) as u8,
            //random_numer_generator.gen_range(0..254) as u8,
        ]);
        let x: i32 = random_numer_generator.gen_range(0..(WIDTH - 1));
        let y: i32 = random_numer_generator.gen_range(0..(HEIGHT - 1));
        Point {
            x: x,
            y: y,
            color: color,
        }
    }
    //get this points x coordinate
    fn get_x(&self) -> i32 {
        self.x
    }
    //get this points y coordinate
    fn get_y(&self) -> i32 {
        self.y
    }
    //get distance from this point to some other x,y coordinate on this plane
    fn get_distance_from(&self, x: i32, y: i32) -> f32 {
        ((x - self.get_x()) as f32).powf(2.0) + ((y - self.get_y()) as f32).powf(2.0)
    }
    //return the color associated with this specific point
    fn get_color(&self) -> image::Rgb<u8> {
        self.color
    }
}

fn main() {
    //create the image buffer that we will eventually write all the color data to
    let mut img_buffer = image::ImageBuffer::new(WIDTH as u32, HEIGHT as u32);
    //let mut random_numer_generator = rand::thread_rng();
    let points = (0..100).map(|_| (Point::new())).collect::<Vec<Point>>();
    //iterate over every pixel in the image buffer mutably
    for (x, y, pixel) in img_buffer.enumerate_pixels_mut() {
        //initiate a temporary "smallest" variable, that will be used to decide which point is
        //closes to the specific pixel that is currently being iterated over 
        let mut smallest = 10000000.0;
        //initiates a temporary color value set to black
        let mut color = image::Rgb([0 as u8, 0 as u8, 0 as u8]);
        //loop over every point in the points list
        for point in &points {
            //if this point is closest to the pixel we are currently iterating over, save it's
            //distance and color in the above declared temporary variables
            if point.get_distance_from(x as i32, y as i32) <= smallest as f32 {
                color = point.get_color();
                smallest = point.get_distance_from(x as i32, y as i32);
            }
        }
        //set the pixel color to the color value of the closest point
        *pixel = color;
    }
    //done, save the buffer to a file and be done with it, voronoi thingy created. yay!
    img_buffer.save("voronoi.png").unwrap();
}
