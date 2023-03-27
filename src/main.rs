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

use std::{sync::Arc, io::{Write, Seek}};

use camera::SimpleCamera;
use integrator::{Integrator, PathIntegrator};
use light::{ConstantInfiniteLight};
use material::Matte;
use media::MediumInterface;
use primitive::Primitive;
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
const ASPECT_RATIO: f32 = (WIDTH as f32) / (HEIGHT as f32);

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

    // let sdl = Sdl::init(InitFlags::VIDEO | InitFlags::EVENTS)?;
    // let window = sdl.create_vk_window(
    //     zstr!("RustyRays"),
    //     None,
    //     (WIDTH as i32, HEIGHT as i32),
    //     WindowFlags::ALLOW_HIGHDPI,
    // )?;

    let mut fb0 = std::fs::File::open("/dev/fb0")?;
    let mut frame = vec![0u8; WIDTH * HEIGHT * 4];

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
            //     Transform::new_translate(vec3(-5.0, 8.0, 0.0)),
            //     color3(1.0, 1.0, 1.0),
            //     100.0,
            // )),
            Box::new(ConstantInfiniteLight::new(
                Transform::new_identity(),
                sky,
                1.0,
            ))
        ],
    };
    let cam = SimpleCamera::new(point3(10.0, 10.0, 10.0), point3(0.0, 0.0, 0.0), 40.0);

    let mut world = World::new(objs, cam, 8, 100);

    world.preprocess();

    // let mut current_frame = 0;
    

    // Draw the current frame
    // let chunker = ImageChunker::new(pixels, 1, 2, WIDTH, HEIGHT);
    // let chunks = chunker.request_frame_chunks();
    // let i = current_frame % frame.chunks_exact_mut(4).len();

    // let x = i % WIDTH as S;
    // let y = i / WIDTH as S;

    // world.draw_next_pixel(i, x, y, frame);
    let chunk_size = HEIGHT / 5;
    assert_eq!(HEIGHT % chunk_size, 0);
    println!("Rendering...");
    frame.chunks_exact_mut(4 * chunk_size).enumerate().for_each(|(chunk_idx, chunk)| {
        let chunk_offset = chunk_size * chunk_idx;
        chunk.chunks_exact_mut(4).enumerate().for_each(|(pixel_idx, pixel)| {
            let i = chunk_offset + pixel_idx;
            let x = i % WIDTH as S;
            let y = i / WIDTH as S;
            let rendered_pixel = world.render_pixel(x, y);
            // pixel[0] = rendered_pixel[0];
            // pixel[1] = rendered_pixel[1];
            // pixel[2] = rendered_pixel[2];
            // pixel[3] = rendered_pixel[3];
            fb0.seek(std::io::SeekFrom::Start(i as u64 * 4)).unwrap();
            fb0.write(&rendered_pixel).unwrap();
            fb0.flush().unwrap();
        });
    });
    
    // fb0.write_all(&frame)?;
    // println!("Done!");

    // 'game_loop: loop {
    //     while let Some(event) = sdl.poll_event() {
    //         match event {
    //             // Close events
    //             Event::Quit { .. } => break 'game_loop,
    //             Event::Keyboard { keycode: key, .. } if key == keycode::SDLK_ESCAPE => {
    //                 break 'game_loop
    //             }
    //             Event::Keyboard {
    //                 // scancode,
    //                 // is_pressed,
    //                 ..
    //             } => {
    //             }
    //             _ => (),
    //         }
    //     }
    // }
    loop {}

    Ok(())
}
