mod common;
mod vector;
mod color;
mod ray;
mod aabb;
mod matrix;
mod transform;
mod quaternion;
mod interaction;
mod shape;
mod sphere;
mod trianglemesh;
mod camera;
mod tests;
mod mesh;
mod primitive;
mod sampler;
mod material;
mod onb;
mod rng;
mod texture;

use std::{rc::Rc};

use beryllium::{init::{Sdl, InitFlags}, window::WindowFlags, event::Event};
use camera::{Camera, SimpleCamera};
use fermium::keycode;
use material::{Matte, Bxdf};
use mesh::Mesh;
use pixels::{SurfaceTexture, Pixels};
use primitive::Primitive;
use ray::Ray;
use rng::RngGen;
use shape::Shape;
use sphere::Sphere;
use texture::{SolidColor, ConstantValue};
use transform::Transform;
use zstring::zstr;

use common::*;
use vector::*;
use color::*;
use rand_distr::{Uniform, Distribution};

const WIDTH: S = 1280/3;
const HEIGHT: S = 720/3;
const ASPECT_RATIO: f32 = (WIDTH as f32) / (HEIGHT as f32);

struct World {
    pub objs: Vec<Rc<Primitive>>,
    pub cam: SimpleCamera,
    pub background: Color3,
    max_depth: S,
    samples_per_pixel: S,
    rng: RngGen,
}

impl World {
    pub fn new(objs: Vec<Rc<Primitive>>, cam: SimpleCamera, background: Color3, max_depth: S, samples_per_pixel: S) -> Self {
        Self {objs, cam, background, max_depth, samples_per_pixel, rng: RngGen::new() }
    }

    pub fn add(&mut self, obj: Rc<Primitive>) {
        self.objs.push(obj);
    }

    /// Fully traces a ray through the world, returning its final color.
    fn trace(&self, ray: &mut Ray, depth: S) -> Color3 {
        let mut rr_factor = 1.0;
        if depth >= self.max_depth {
            let rr_stop_prob = 1.0f32.min(0.0625 * depth as F);
            if self.rng.sample_0_1() <= rr_stop_prob {
                return black()
            }
            rr_factor = 1.0 / (1.0 - rr_stop_prob);
        }
        let mut interaction = None;
        let mut obj_hit  = None;
        for obj in self.objs.iter() {
            match obj.intersect(ray, false) {
                Some(inter) => {
                    interaction = Some(inter);
                    obj_hit = Some(obj);
                }
                None => {}
            }
        }
        match interaction {
            Some(inter) => {
                match obj_hit.unwrap().material.scatter_ray(ray, &inter, &self.rng) {
                    Some(result) => {
                        ray.origin = inter.p;
                        match result.bxdfs[0].sample_f(&result.wo, &self.rng.uniform_sample_point2()) {
                            Some((attenuation, pdf, wi, _)) => {
                                let scatter_pdf = result.material.scattering_pdf(ray, &inter);
                                ray.direction = onb::Onb::new_from_w(&inter.shading.n).local(&wi);
                                let bounced_color = self.trace(ray, depth+1);
                                let mut out_color = attenuation * scatter_pdf;
                                out_color.x *= bounced_color.x;
                                out_color.y *= bounced_color.y;
                                out_color.z *= bounced_color.z;
                                out_color /= pdf;
                                // out_color /= result.material.scattering_pdf(ray, &inter);
                                out_color * rr_factor
                            },
                            None => {black()}
                        }
                        // out_color = color3(out_color.x * bounced_color.x, out_color.y * bounced_color.y, out_color.z * bounced_color.z);
                    },
                    None => { black() }
                }
            },
            None => self.background * rr_factor
        }
    }

    pub fn render_pixel(&self, frame: &mut [u8], pixel_idx: S, x: S, y: S) {
        // if x == WIDTH/2 && y == HEIGHT/2 {
        //     println!("At Center");
        // }
        // let rng = Uniform::new(-1.0, 1.0);
        let mut out_col = black();
        for _ in 0..self.samples_per_pixel {
            let mut ray = self.cam.get_ray(point2(x as F + self.rng.sample_neg1_1(), y as F + self.rng.sample_neg1_1()));
            let col = self.trace(&mut ray, 1);
            out_col += col / self.samples_per_pixel as F;
        }
        
        let pix = color_to_pixel(out_col);
        // let out_idx = xy.y as S * WIDTH as S + xy.x as S;
        // let out_idx = pixel_idx;
        frame[4 * pixel_idx] = pix[0];
        frame[4 * pixel_idx + 1] = pix[1];
        frame[4 * pixel_idx + 2] = pix[2];
        frame[4 * pixel_idx + 3] = pix[3];
    }
}



fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let sdl = Sdl::init(InitFlags::VIDEO | InitFlags::EVENTS)?;
    let window = sdl.create_vk_window(
        zstr!("RustyRays"),
        None,
        (WIDTH as i32, HEIGHT as i32),
        WindowFlags::ALLOW_HIGHDPI,
    )?;

    let mut pixels = {
        let surface_texture = SurfaceTexture::new(WIDTH as u32, HEIGHT as u32, &*window);
        Pixels::new(WIDTH as u32, HEIGHT as u32, surface_texture)?
    };

    let objs: Vec<Rc<Primitive>> = vec![
        // Box::new(Sphere::new(
        //     false, 20.0, -1.0, 1.0, 2.0*PI)),
        Rc::new(Primitive::new(
            Rc::new(Sphere::new(false, 100.0)),
            Transform::new_translate(vec3(0.0,-100.01,0.0)),
            Rc::new(Matte { 
                kd: Rc::new(SolidColor { color:  color3(0.1, 0.1, 0.1) }),
                bump_map: None,
                sigma: Some(Rc::new(ConstantValue { val: 0.0 })),
            })
        )),
        Rc::new(Primitive::new(
            Rc::new(Sphere::new(false, 3.0)),
            Transform::new_translate(vec3(0.0,3.0,0.0)),
            Rc::new(Matte { 
                kd: Rc::new(SolidColor { color:  color3(1.0, 0.1, 0.1) }),
                bump_map: None,
                sigma: Some(Rc::new(ConstantValue { val: 0.0 })),
            })
        )),
        // Rc::new(Primitive::new(
        //     Mesh::load_obj("./cube.obj".to_string()).expect("Error loading model!"),
        //     Transform::new_translate(vec3(0.0, 2.0, 0.0)),
        //     Rc::new(Lambertian { color: color3(0.8, 0.8, 0.8) })
        // )),
    ];
    // let cam = Camera::new(
        
    //     Transform::new_lookat(
    //         point3(10.0,10.0,10.0), 
    //         point3(0.0, 0.1,0.0), 
    //         vec3(0.0, 1.0, 0.0)),// * Transform::new_translate(point3(10.0, 10.0, 10.0)),

    //         // * Transform::new_translate(vec3(10.0,10.0,10.0)),
    //     90.0
    // );
    let cam = SimpleCamera::new(
        point3(10.0,10.0,10.0),
        point3(0.0,0.0,0.0),
        40.0
    );

    let world = World::new(objs, cam, color3(1.0, 1.0, 1.0), 5, 100);

    let mut current_frame = 0;
    'game_loop: loop {
        while let Some(event) = sdl.poll_event() {
            match event {
                // Close events
                Event::Quit { .. } => break 'game_loop,
                Event::Keyboard { keycode: key, .. } if key == keycode::SDLK_ESCAPE => {
                    break 'game_loop
                }
                Event::Keyboard {
                    // scancode,
                    // is_pressed,
                    ..
                } => {
                }
                _ => (),
            }
        }

        // Draw the current frame
        let frame = pixels.get_frame();
        let i = current_frame % frame.chunks_exact_mut(4).len();

        let x = i % WIDTH as S;
        let y = i / WIDTH as S;

        // world.draw_next_pixel(i, x, y, frame);
        world.render_pixel(frame, i, x, y);
        if current_frame % (HEIGHT*5) as S == 0 {
            pixels.render()?;
            println!("Rendered scanline {}/{}", y, HEIGHT);
        }

        current_frame += 1;
    }

    Ok(())
}
