use std::f32::consts::PI;
#[cfg(test)]
use std::f32::INFINITY;

use crate::common::F;
use crate::ray::Ray;
use crate::transform::Transform;
use crate::vector::*;

pub fn assert_close_vec(a: Vec3, b: Vec3) {
    let diff = (a - b).magnitude_squared();
    assert!(diff < 0.0001)
}

#[test]
fn test_transforms() {
    let x = vec3(1.0, 0.0, 0.0);
    let y = vec3(0.0, 1.0, 0.0);
    let z = vec3(0.0, 0.0, 1.0);

    let translate_x = Transform::new_translate(x);
    let translate_y = Transform::new_translate(y);
    let translate_z = Transform::new_translate(z);

    let x_t_x = translate_x.fpt(x);
    let y_t_x = translate_x.fpt(y);
    let z_t_x = translate_x.fpt(z);

    let x_t_y = translate_y.fpt(x);
    let y_t_y = translate_y.fpt(y);
    let z_t_y = translate_y.fpt(z);

    let x_t_z = translate_z.fpt(x);
    let y_t_z = translate_z.fpt(y);
    let z_t_z = translate_z.fpt(z);

    assert_close_vec(x_t_x, vec3(2.0, 0.0, 0.0));
    assert_close_vec(y_t_x, vec3(1.0, 1.0, 0.0));
    assert_close_vec(z_t_x, vec3(1.0, 0.0, 1.0));

    assert_close_vec(x_t_y, vec3(1.0, 1.0, 0.0));
    assert_close_vec(y_t_y, vec3(0.0, 2.0, 0.0));
    assert_close_vec(z_t_y, vec3(0.0, 1.0, 1.0));

    assert_close_vec(x_t_z, vec3(1.0, 0.0, 1.0));
    assert_close_vec(y_t_z, vec3(0.0, 1.0, 1.0));
    assert_close_vec(z_t_z, vec3(0.0, 0.0, 2.0));

    assert_close_vec(translate_x.ipt(x_t_x), x);
    assert_close_vec(translate_x.ipt(y_t_x), y);
    assert_close_vec(translate_x.ipt(z_t_x), z);

    assert_close_vec(translate_y.ipt(x_t_y), x);
    assert_close_vec(translate_y.ipt(y_t_y), y);
    assert_close_vec(translate_y.ipt(z_t_y), z);

    assert_close_vec(translate_z.ipt(x_t_z), x);
    assert_close_vec(translate_z.ipt(y_t_z), y);
    assert_close_vec(translate_z.ipt(z_t_z), z);

    let theta = PI;
    let rotate_x = Transform::new_rotate_x(theta);
    let rotate_y = Transform::new_rotate_y(theta);
    let rotate_z = Transform::new_rotate_z(theta);

    let x_r_x = rotate_x.fpt(x);
    let y_r_x = rotate_x.fpt(y);
    let z_r_x = rotate_x.fpt(z);

    let x_r_y = rotate_y.fpt(x);
    let y_r_y = rotate_y.fpt(y);
    let z_r_y = rotate_y.fpt(z);

    let x_r_z = rotate_z.fpt(x);
    let y_r_z = rotate_z.fpt(y);
    let z_r_z = rotate_z.fpt(z);

    assert_close_vec(rotate_x.ipt(x_r_x), x);
    assert_close_vec(rotate_x.ipt(y_r_x), y);
    assert_close_vec(rotate_x.ipt(z_r_x), z);

    assert_close_vec(rotate_y.ipt(x_r_y), x);
    assert_close_vec(rotate_y.ipt(y_r_y), y);
    assert_close_vec(rotate_y.ipt(z_r_y), z);

    assert_close_vec(rotate_z.ipt(x_r_z), x);
    assert_close_vec(rotate_z.ipt(y_r_z), y);
    assert_close_vec(rotate_z.ipt(z_r_z), z);

    let ray_xy = Ray::new_non_differential(x, y, 0.0, F::INFINITY, 0.0);
    let ray_yz = Ray::new_non_differential(y, z, 0.0, F::INFINITY, 0.0);
    let ray_zx = Ray::new_non_differential(z, x, 0.0, F::INFINITY, 0.0);

    let xy_t_x = translate_x.fray(&ray_xy);
    let yz_t_x = translate_x.fray(&ray_yz);
    let zx_t_x = translate_x.fray(&ray_zx);

    // let xy_t_x = translate_x.fray(&ray_xy);
    // let yz_t_x = translate_x.fray(&ray_yz);
    // let zx_t_x = translate_x.fray(&ray_zx);

    assert_close_vec(xy_t_x.origin, ray_xy.origin + x);
    assert_close_vec(yz_t_x.origin, ray_yz.origin + x);
    assert_close_vec(zx_t_x.origin, ray_zx.origin + x);

    assert_close_vec(xy_t_x.direction, ray_xy.direction);
    assert_close_vec(yz_t_x.direction, ray_yz.direction);
    assert_close_vec(zx_t_x.direction, ray_zx.direction);
}
