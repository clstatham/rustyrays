#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(clippy::too_many_arguments)]
mod aabb;
mod camera;
mod color;
mod common;
mod integrator;
mod interaction;
mod light;
mod material;
mod matrix;
// mod mesh;
mod onb;
mod primitive;
mod quaternion;
mod ray;
mod rng;
mod distributions;
mod scene;
mod shape;
mod sphere;
mod texture;
mod transform;
mod vector;
mod media;


use rayon::{prelude::*};
use winit::{event_loop::{EventLoop, ControlFlow}, dpi::LogicalSize, window::WindowBuilder, event::VirtualKeyCode};
use winit_input_helper::WinitInputHelper;

use std::{sync::Arc, time::Instant};

use camera::SimpleCamera;
use integrator::{Integrator, PathIntegrator};
use light::{ConstantInfiniteLight};
use material::Matte;
use media::MediumInterface;
use pixels::{Pixels, SurfaceTexture};
use primitive::Primitive;
use rayon::{iter::{IndexedParallelIterator, ParallelIterator}};
use rng::RngGen;
use scene::Scene;
use sphere::Sphere;
use texture::{ConstantValue, SolidColor};
use transform::Transform;

use color::*;
use common::*;
use vector::*;



const WIDTH: S = 640;
const HEIGHT: S = 400;
const ASPECT_RATIO: F = (WIDTH as F) / (HEIGHT as F);

struct World where Self: Send + Sync {
    pub scene: Scene,
    pub cam: SimpleCamera,
    pub integrator: PathIntegrator,
    // max_depth: S,
    samples_per_pixel: S,
    rng: RngGen,
}

impl World {
    pub fn new(scene: Scene, cam: SimpleCamera, max_depth: S, samples_per_pixel: S) -> Self {
        Self {
            scene,
            cam,
            // max_depth,
            samples_per_pixel,
            rng: RngGen::new(),
            integrator: PathIntegrator::new(max_depth),
        }
    }

    pub fn preprocess(&mut self) {
        self.scene.preprocess();
        self.integrator.preprocess(&self.scene, &self.cam);
    }

    pub fn render_pixel(&self, x: S, y: S) -> [u8; 4] {
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

        color_to_pixel(out_col)
        // let out_idx = xy.y as S * WIDTH as S + xy.x as S;
        // let out_idx = pixel_idx;
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new().with_title("RustyRays").with_inner_size(size).with_min_inner_size(size).with_max_inner_size(size).build(&event_loop)?
    };

    let mut pixels = {
        let surface_texture = SurfaceTexture::new(WIDTH as u32, HEIGHT as u32, &window);
        Pixels::new(WIDTH as u32, HEIGHT as u32, surface_texture)?
    };

    let sky = color3(0.7, 0.8, 1.0);

    let objs: Scene = Scene {
        objs: vec![
            // Box::new(Sphere::new(
            //     false, 20.0, -1.0, 1.0, 2.0*PI)),
            Primitive::new(
                Arc::new(Sphere::new(
                    false,
                    100.0,
                    Transform::new_translate(vec3(0.0, -100.01, 0.0)),
                    MediumInterface::new_empty(),
                )),
                Arc::new(Matte {
                    kd: Arc::new(SolidColor {
                        color: color3(0.1, 0.1, 0.1),
                    }),
                    bump_map: None,
                    sigma: Some(Arc::new(ConstantValue { val: 0.0 })),
                }),
                None,
            ),
            Primitive::new(
                Arc::new(Sphere::new(
                    false,
                    3.0,
                    Transform::new_translate(vec3(0.0, 3.0, 0.0)),
                    MediumInterface::new_empty(),
                )),
                Arc::new(Matte {
                    kd: Arc::new(SolidColor {
                        color: color3(1.0, 0.1, 0.1),
                    }),
                    bump_map: None,
                    sigma: Some(Arc::new(ConstantValue { val: 0.0 })),
                }),
                None,
            ),
        ],
        lights: vec![
            // Box::new(PointLight::new(
            //     Transform::new_translate(vec3(-5.0, 12.0, 0.0)),
            //     color3(1.0, 1.0, 1.0),
            //     1000.0,
            // )),
            Box::new(ConstantInfiniteLight::new(
                Transform::new_identity(),
                sky,
                100.0,
            ))
        ],
    };
    // let cam = Camera::new(

    //     Transform::new_lookat(
    //         point3(10.0,10.0,10.0),
    //         point3(0.0, 0.1,0.0),
    //         vec3(0.0, 1.0, 0.0)),// * Transform::new_translate(point3(10.0, 10.0, 10.0)),

    //         // * Transform::new_translate(vec3(10.0,10.0,10.0)),
    //     90.0
    // );
    let cam = SimpleCamera::new(point3(10.0, 10.0, 10.0), point3(0.0, 0.0, 0.0), 40.0);

    let mut world = World::new(objs, cam, 8, 100);

    world.preprocess();

    // let mut current_frame = 0;
    

    // Draw the current frame
    let frame = pixels.frame_mut();
    // let chunker = ImageChunker::new(pixels, 1, 2, WIDTH, HEIGHT);
    // let chunks = chunker.request_frame_chunks();
    // let i = current_frame % frame.chunks_exact_mut(4).len();

    // let x = i % WIDTH as S;
    // let y = i / WIDTH as S;

    // world.draw_next_pixel(i, x, y, frame);
    let chunk_size = HEIGHT / 5;
    assert_eq!(HEIGHT % chunk_size, 0);
    let start = Instant::now();
    println!("Rendering...");
    frame.par_chunks_exact_mut(4 * chunk_size).enumerate().for_each(|(chunk_idx, chunk)| {
        let chunk_offset = chunk_size * chunk_idx;
        chunk.chunks_exact_mut(4).enumerate().for_each(|(pixel_idx, pixel)| {
            let i = chunk_offset + pixel_idx;
            let x = i % WIDTH as S;
            let y = i / WIDTH as S;
            let rendered_pixel = world.render_pixel(x, y);
            pixel[0] = rendered_pixel[0];
            pixel[1] = rendered_pixel[1];
            pixel[2] = rendered_pixel[2];
            pixel[3] = rendered_pixel[3];
        });
    });
    pixels.render()?;
    let end = Instant::now();
    println!("Done in {} seconds!", (end - start).as_secs_f32());
    event_loop.run(move |event, _, control_flow| {
        if input.update(&event) {
            if input.key_pressed(VirtualKeyCode::Escape) || input.close_requested() {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }
    });
}
