use crate::matrix::matrix4;
use crate::{common::*, WIDTH, HEIGHT, ASPECT_RATIO};
use crate::ray::Ray;
use crate::vector::*;
use crate::transform::Transform;

pub struct Camera {
    camera_to_world: Transform,
    // p_div_inv: Transform,
    // p_scale: Transform,
    raster_to_camera: Transform,
    // pub world_to_raster: Transform,
    // screen_to_raster: Transform,
    // raster_to_screen: Transform,
    // screen_to_camera: Transform,
    // dx_camera: Vec3,
    // dy_camera: Vec3,
}
impl Camera {
    pub fn new(camera_to_world: Transform, fov_deg: F) -> Camera {
        let f = 1000.0;
        let n = 0.0001;
        let screen = 
            if ASPECT_RATIO > 1.0 {
                [-ASPECT_RATIO, ASPECT_RATIO, -1.0, 1.0]
            } else {
                [-1.0, 1.0, -1.0 / ASPECT_RATIO, 1.0 / ASPECT_RATIO]
            };
        // let camera_to_screen = Transform::new_perspective(fov_deg, n, f);
        let screen_to_raster = 
            Transform::new_scale(vec3(WIDTH as F, HEIGHT as F, 1.0))
            * Transform::new_scale(vec3(1.0 / (screen[1] - screen[0]) as F, 1.0 / (screen[2] - screen[3]) as F, 1.0))
            * Transform::new_translate(vec3(-screen[0], -screen[3], 0.0));
        
        let raster_to_screen = screen_to_raster.as_inverse_transform();
        
        // let persp = matrix4([
        //     [1.0, 0.0, 0.0, 0.0],
        //     [0.0, 1.0, 0.0, 0.0],
        //     [0.0, 0.0, f / (f-n), -f*n/(f-n)],
        //     [0.0, 0.0, 1.0, 0.0],
        // ]);
        // let tan_fov = deg2rad(fov_deg / 2.0).tan();
        // let p_scale = Transform::new_scale(vec3(tan_fov, tan_fov, 1.0));
        // let p_div = Transform::new_from_forward(persp).expect("Error creating projection matrix");
        // let p_div_inv = p_div.as_inverse_transform();
        // let raster_to_camera = p_div_inv * p_scale * raster_to_screen;

        let proj = Transform::new_perspective(fov_deg, n, f);
        let raster_to_camera = proj * raster_to_screen;

        Camera {
            camera_to_world,
            // p_scale,
            // p_div_inv,
            // screen_to_raster,
            raster_to_camera,
            // camera_to_raster,
            // p_scale,
            // raster_to_screen,
            // p_div_inv,
            // p_scale,
            // dx_camera: raster_to_camera.fpt(point3(1.0, 0.0, 0.0)) - raster_to_camera.fpt(point3(0.0,0.0,0.0)),
            // dy_camera: raster_to_camera.fpt(point3(0.0, 1.0, 0.0)) - raster_to_camera.fpt(point3(0.0,0.0,0.0)),
        }
    }

    // pub fn world_to_raster(&self, xyz: Point3) -> Point2 {
    //     let p_camera = self.camera_to_world.ipt(xyz);
    //     // // let p_screen = self.screen_to_camera.ipt(p_camera);
    //     let p_raster = self.raster_to_camera.ipt(p_camera);
    //     // let p_raster = self.world_to_raster(xyz);
    //     point2(p_raster.x, p_raster.y)
    // }

    pub fn get_ray(&self, xy: Point2) -> Ray {
        let p_raster = point3(xy.x, xy.y, 0.0);
        // let p_screen = self.screen_to_raster.ipt(p_raster);
        
        let p_camera = self.raster_to_camera.fpt(p_raster);
        let direction = p_camera.normalize();
        // let p_camera = self.p_scale.fpt((self.p_div_inv * self.screen_to_raster).fpt(p_film));
        let ray = Ray::new_non_differential(point3(0.0,0.0,0.0), direction, 0.0000, F::INFINITY, 0.0);
        self.camera_to_world.iray(&ray)
    }
}

pub struct SimpleCamera {
    pub fov: F,
    // pub origin: Point3,
    lookat: Transform,
}

impl SimpleCamera {
    pub fn new(origin: Point3, lookat: Point3, fov: F) -> Self {
        Self {
            fov,
            lookat: Transform::new_lookat(origin, lookat, vec3(0.0, 1.0, 0.0))
        }
    }

    pub fn get_ray(&self, xy: Point2) -> Ray {
        let angle = deg2rad(self.fov/2.0).tan();
        let xx = (2.0 * ((xy.x + 0.5) * (1.0 / WIDTH as F)) - 1.0) * angle * ASPECT_RATIO;
        let yy = (1.0 - 2.0 * ((xy.y + 0.5) * (1.0 / HEIGHT as F))) * angle;
        let direction = vec3(xx, yy, -1.0).normalize();
        let ray = Ray::new_non_differential(point3(0.0,0.0,0.0), direction, 0.0001, F::INFINITY, 0.0);
        let out = self.lookat.iray(&ray);
        out
    }
}