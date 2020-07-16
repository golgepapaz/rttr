mod vec3;
mod ray;
use vec3::Vec3;
use ray::Ray;
type Color = Vec3;

fn ray_color(r: &Ray) -> Color {
   let unit_direction =  r.direction().unit();
   let t = (unit_direction.y + 1.0) * 0.5;
    Color::new(1.0,1.0,1.0) * (1.0-t)  + Color::new(0.5,0.7,0.4) * t
} 
fn main() {

const ASPECT_RATIO: f64 =  16.0 / 9.0;
const IMAGE_WIDTH: i32 = 384;
const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;

let viewport_height = 2.0;
let viewport_width = ASPECT_RATIO * viewport_height;
let focal_length = 1.0;

let origin = Vec3::new(0.,0.,0.);
let horizontal = Vec3::new(viewport_width, 0., 0.);
let vertical = Vec3::new(0., viewport_height, 0.);
let lower_left = origin - horizontal/2. - vertical/2. - Vec3::new(0.,0.,focal_length);

}
