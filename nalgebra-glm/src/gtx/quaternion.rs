use na::{Real, Unit, Rotation3, Vector4, UnitQuaternion, U3, U4};

use aliases::{Qua, Vec, Mat};

/// Rotate the vector `v` by the quaternion `q` assumed to be normalized.
pub fn quat_cross<N: Real>(q: &Qua<N>, v: &Vec<N, U3>) -> Vec<N, U3> {
    UnitQuaternion::new_unchecked(*q) * v
}

/// Rotate the vector `v` by the inverse of the quaternion `q` assumed to be normalized.
pub fn quat_inv_cross<N: Real>(v: &Vec<N, U3>, q: &Qua<N>) -> Vec<N, U3> {
    UnitQuaternion::new_unchecked(*q).inverse() * v
}

/// The quaternion `w` component.
pub fn quat_extract_real_component<N: Real>(q: &Qua<N>) -> N {
    q.w
}

/// Normalized linear interpolation between two quaternions.
pub fn quat_fast_mix<N: Real>(x: &Qua<N>, y: &Qua<N>, a: N) -> Qua<N> {
    Unit::new_unchecked(*x).nlerp(&Unit::new_unchecked(*y), a).unwrap()
}

//pub fn quat_intermediate<N: Real>(prev: &Qua<N>, curr: &Qua<N>, next: &Qua<N>) -> Qua<N> {
//    unimplemented!()
//}

/// The squared magnitude of a quaternion `q`.
pub fn quat_length2<N: Real>(q: &Qua<N>) -> N {
    q.norm_squared()
}

/// The squared magnitude of a quaternion `q`.
pub fn quat_magnitude2<N: Real>(q: &Qua<N>) -> N {
    q.norm_squared()
}

/// The quaternion representing the identity rotation.
pub fn quat_quat_identity<N: Real>() -> Qua<N> {
    UnitQuaternion::identity().unwrap()
}

/// Rotates a vector by a quaternion assumed to be normalized.
pub fn quat_rotate_vec3<N: Real>(q: &Qua<N>, v: &Vec<N, U3>) -> Vec<N, U3> {
    UnitQuaternion::new_unchecked(*q) * v
}

/// Rotates a vector in homogeneous coordinates by a quaternion assumed to be normalized.
pub fn quat_rotate<N: Real>(q: &Qua<N>, v: &Vec<N, U4>) -> Vec<N, U4> {
//    UnitQuaternion::new_unchecked(*q) * v
    let rotated = Unit::new_unchecked(*q) * v.fixed_rows::<U3>(0);
    Vector4::new(rotated.x, rotated.y, rotated.z, v.w)
}

/// The rotation required to align `orig` to `dest`.
pub fn quat_rotation<N: Real>(orig: &Vec<N, U3>, dest: &Vec<N, U3>) -> Qua<N> {
    UnitQuaternion::rotation_between(orig, dest).unwrap_or(UnitQuaternion::identity()).unwrap()
}

/// The spherical linear interpolation between two quaternions.
pub fn quat_short_mix<N: Real>(x: &Qua<N>, y: &Qua<N>, a: N) -> Qua<N> {
    Unit::new_normalize(*x).slerp(&Unit::new_normalize(*y), a).unwrap()
}

//pub fn quat_squad<N: Real>(q1: &Qua<N>, q2: &Qua<N>, s1: &Qua<N>, s2: &Qua<N>, h: N) -> Qua<N> {
//    unimplemented!()
//}

/// Converts a quaternion to a rotation matrix.
pub fn quat_to_mat3<N: Real>(x: &Qua<N>) -> Mat<N, U3, U3> {
    UnitQuaternion::new_unchecked(*x).to_rotation_matrix().unwrap()
}

/// Converts a quaternion to a rotation matrix in homogenous coordinates.
pub fn quat_to_mat4<N: Real>(x: &Qua<N>) -> Mat<N, U4, U4> {
    UnitQuaternion::new_unchecked(*x).to_homogeneous()
}

/// Converts a rotation matrix to a quaternion.
pub fn mat3_to_quat<N: Real>(x: &Mat<N, U3, U3>) -> Qua<N> {
    let r = Rotation3::from_matrix_unchecked(*x);
    UnitQuaternion::from_rotation_matrix(&r).unwrap()
}

/// Converts a rotation matrix in homogeneous coordinates to a quaternion.
pub fn to_quat<N: Real>(x: &Mat<N, U4, U4>) -> Qua<N> {
    let rot = x.fixed_slice::<U3, U3>(0, 0).into_owned();
    to_quat(&rot)
}

