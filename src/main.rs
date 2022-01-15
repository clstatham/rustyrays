#![allow(dead_code)]
#![allow(unused_variables)]
mod aabb;
mod camera;
mod color;
mod common;
mod interaction;
mod light;
mod material;
mod matrix;
mod mesh;
mod onb;
mod primitive;
mod quaternion;
mod ray;
mod rng;
mod sampler;
mod scene;
mod shape;
mod sphere;
mod tests;
mod texture;
mod transform;
mod trianglemesh;
mod vector;
mod integrator;

use std::rc::Rc;

use beryllium::{
    event::Event,
    init::{InitFlags, Sdl},
    window::WindowFlags,
};
use camera::{SimpleCamera};
use fermium::keycode;
use integrator::{Integrator, DirectLightingIntegrator};
use light::{ConstantInfiniteLight};
use material::Matte;
use mesh::Mesh;
use pixels::{Pixels, SurfaceTexture};
use primitive::Primitive;
use rng::RngGen;
use scene::Scene;
use sphere::Sphere;
use texture::{ConstantValue, SolidColor};
use transform::Transform;
use zstring::zstr;

use color::*;
use common::*;
use vector::*;

const WIDTH: S = 1280 / 3;
const HEIGHT: S = 720 / 3;
const ASPECT_RATIO: f32 = (WIDTH as f32) / (HEIGHT as f32);

struct World {
    pub scene: Scene,
    pub cam: SimpleCamera,
    pub background: Color3,
    pub integrator: Rc<dyn Integrator>,
    // max_depth: S,
    samples_per_pixel: S,
    rng: RngGen,
}

impl World {
    pub fn new(
        scene: Scene,
        cam: SimpleCamera,
        background: Color3,
        max_depth: S,
        samples_per_pixel: S,
    ) -> Self {
        Self {
            scene,
            cam,
            background,
            // max_depth,
            samples_per_pixel,
            rng: RngGen::new(),
            integrator: Rc::new(DirectLightingIntegrator::new(integrator::LightStrategy::UniformSampleOne, max_depth)),
        }
    }

    /// Fully traces a ray through the world, returning its final color.
    // fn trace(&self, ray: &mut Ray, depth: S) -> Color3 {
    //     let mut rr_factor = 1.0;
    //     let mut out_color = black();
    //     if depth >= self.max_depth {
    //         let rr_stop_prob = 1.0f32.min(0.0625 * depth as F);
    //         if self.rng.sample_0_1() <= rr_stop_prob {
    //             return black();
    //         }
    //         rr_factor = 1.0 / (1.0 - rr_stop_prob);
    //     }
    //     if let Some(mut inter) = self.scene.intersect(ray) {
    //         // ray.origin = inter.p;
    //         // if let Some(ref obj_hit) = inter.primitive {
    //             // if let Some(material) = &obj_hit.material {
    //                 for light in self.scene.lights.iter() {
    //                     let li = light
    //                         .sample_li(Rc::new(inter.clone()), self.rng.uniform_sample_point2());
    //                     if li.col == black() || li.pdf == Some(0.0) || li.pdf.is_none() {
    //                         return black();
    //                     }
    //                     inter.scatter(ray, &self.rng);
                        
    //                     if let Some(ref bsdf) = inter.bsdf {
    //                         if let Some((attenuation, material_pdf, material_wi, _)) =
    //                             bsdf
    //                                 .sample_f(&-ray.direction, &self.rng.uniform_sample_point2(), crate::material::BXDF_REFLECTION | crate::material::BXDF_SPECULAR)
    //                         {
                                
    //                             if let Some(ref shading) = inter.shading {
    //                                 // ray.direction = onb::Onb::new_from_w(&inter.n.unwrap()).local(&material_wi);
    //                                 let bounced_color = self.trace(ray, depth+1);
    //                                 let mut f = attenuation + bounced_color;
    //                                 if li.vis.unwrap().unoccluded(&self.scene) {
    //                                 // if true {
    //                                     f.x *= li.col.x;
    //                                     f.y *= li.col.y;
    //                                     f.z *= li.col.z;
    //                                     if let Some(light_wi) = li.wi {
    //                                         f *= light_wi.dot(&shading.n).abs();
    //                                     }
    //                                     if let Some(light_pdf) = li.pdf {
    //                                         f /= light_pdf;
    //                                     }
    //                                     f *= material_wi.dot(&shading.n).abs();
    //                                     f /= material_pdf;
    //                                     out_color += f;
    //                                 }
    //                             } // if let shading
    //                         } // if let attenuation
    //                     } // if let bsdf
    //                 }
    //             // } // if let material
    //         // } // if let obj_hit
    //         out_color * rr_factor
    //     } else {
    //         // if let inter
    //         for light in self.scene.lights.iter() {
    //             // out_color += self.background * rr_factor; // TODO: add emmissive background lights
    //             out_color += light.le(ray);
    //         }
    //         // out_color += self.background;
    //         out_color * rr_factor
    //     }
        // out_color
    // }

    pub fn render_pixel(&self, frame: &mut [u8], pixel_idx: S, x: S, y: S) {
        // if x == WIDTH/2 && y == HEIGHT/2 {
        //     println!("At Center");
        // }
        let mut out_col = black();
        for _ in 0..self.samples_per_pixel {
            let mut ray = self.cam.get_ray(point2(
                x as F + self.rng.sample_neg1_1(),
                y as F + self.rng.sample_neg1_1(),
            ));
            let col = self.integrator.li(&mut ray, &self.scene, 0, &self.rng);
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

    let sky = color3(0.7, 0.8, 1.0);

    let mut objs: Scene = Scene {
        objs: vec![
            // Box::new(Sphere::new(
            //     false, 20.0, -1.0, 1.0, 2.0*PI)),
            Primitive::new(
                Rc::new(Sphere::new(false, 100.0)),
                Transform::new_translate(vec3(0.0, -100.01, 0.0)),
                Rc::new(Matte {
                    kd: Rc::new(SolidColor {
                        color: color3(0.1, 0.1, 0.1),
                    }),
                    bump_map: None,
                    sigma: Some(Rc::new(ConstantValue { val: 0.0 })),
                }),
                None,
            ),
            Primitive::new(
                Rc::new(Sphere::new(false, 3.0)),
                Transform::new_translate(vec3(0.0, 3.0, 0.0)),
                Rc::new(Matte {
                    kd: Rc::new(SolidColor {
                        color: color3(1.0, 0.1, 0.1),
                    }),
                    bump_map: None,
                    sigma: Some(Rc::new(ConstantValue { val: 0.0 })),
                }),
                None,
            ),
            Primitive::new(
                Mesh::load_obj("./cube.obj".to_string()).expect("Error loading model!"),
                Transform::new_translate(vec3(0.0, 2.0, 4.0)),
                Rc::new(Matte {
                    kd: Rc::new(SolidColor {
                        color: color3(0.1, 0.1, 1.0),
                    }),
                    bump_map: None,
                    sigma: Some(Rc::new(ConstantValue { val: 0.0 })),
                }),
                None,
            ),
        ],
        lights: vec![
            //     Box::new(PointLight::new(
            //     Transform::new_translate(vec3(0.0, 8.0, 5.0)),
            //     color3(1.0, 1.0, 1.0) * 5.0,
            // )),
            Box::new(ConstantInfiniteLight::new(
                Transform::new_identity(),
                sky * 10.0,
            ))
        ],
    };
    objs.preprocess();
    // let cam = Camera::new(

    //     Transform::new_lookat(
    //         point3(10.0,10.0,10.0),
    //         point3(0.0, 0.1,0.0),
    //         vec3(0.0, 1.0, 0.0)),// * Transform::new_translate(point3(10.0, 10.0, 10.0)),

    //         // * Transform::new_translate(vec3(10.0,10.0,10.0)),
    //     90.0
    // );
    let cam = SimpleCamera::new(point3(10.0, 10.0, 10.0), point3(0.0, 0.0, 0.0), 40.0);

    
    let world = World::new(objs, cam, sky, 5, 10);

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
        if current_frame % (HEIGHT * 5) as S == 0 {
            pixels.render()?;
            println!("Rendered scanline {}/{}", y, HEIGHT);
        }

        current_frame += 1;
    }

    Ok(())
}
