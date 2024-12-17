#![allow(dead_code)]
// Using
use core::ops;
use lazy_static::lazy_static;
use num_traits::{cast::NumCast, Float, FloatConst};

#[derive(Debug, Clone, Copy)]
pub struct Vector2<T> {
    pub x: T,
    pub y: T,
}

impl<T : Sized + Copy + NumCast> Vector2<T> {
    pub fn new(x: T, y: T) -> Self {
        Vector2 { x, y }
    }

    pub fn new_x(x: T) -> Self where T: Default {
        Vector2 { x: x, y: T::default() }
    }

    pub fn new_y(y: T) -> Self where T: Default {
        Vector2 { x: T::default(), y: y }
    }

    pub fn zeros() -> Self where T: Default {
        Vector2 { x: T::default() , y: T::default() }
    }

    pub fn yx(&self) -> Vector2<T> {
        Vector2 { x: self.y, y: self.x }
    }
}

impl<T : Sized + Copy + NumCast + Default> Vector2<T> {
    pub fn from<U: Sized + Copy + NumCast + Default>(vec: &Vector2<U>) -> Vector2<T> {
        Vector2::<T>{ 
            x: NumCast::from(vec.x).unwrap_or_default(),
            y: NumCast::from(vec.y).unwrap_or_default(),
        }
    }
}

impl<T: Float> Vector2<T> {
    pub fn normalize(&self) -> Vector2<T> {
        let length = self.dot(&self).sqrt();
        Vector2 { x: self.x / length, y: self.y / length }
    }
    
    pub fn distance(&self, right: &Vector2<T>) -> T {
        let diff = *self - *right;
        diff.dot(&diff).sqrt()
    }
    
    pub fn magnitude(&self) -> T {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
    
    pub fn round(&self) -> Vector2<T> {
        Vector2 { x: self.x.round(), y: self.y.round() }
    }
}

impl<T: ops::Add<Output = T> + ops::Mul<Output = T> + ops::Sub<Output = T> + Sized + Copy + NumCast> Vector2<T> {
    pub fn dot(&self, right: &Vector2<T>) -> T {
        return self.x * right.x + self.y * right.y;
    }

    pub fn cross(&self, right: &Vector2<T>) -> T {
        self.x * right.y - self.y * right.x
    } 
}

impl<T: ops::Add<Output = T> + Sized + Copy + NumCast> ops::Add<Vector2<T>> for Vector2<T> {
    type Output = Vector2<T>;
    fn add(self, right: Vector2<T>) -> Vector2<T> {
        Vector2::new(self.x + right.x, self.y + right.y)
    }
}

impl<T: ops::Mul<Output = T> + Sized + Copy + NumCast> ops::Mul<Vector2<T>> for Vector2<T> {
    type Output = Vector2<T>;
    fn mul(self, right: Vector2<T>) -> Vector2<T> {
        Vector2::new(self.x * right.x, self.y * right.y)
    }
}

impl<T: ops::Sub<Output = T> + Sized + Copy + NumCast> ops::Sub<Vector2<T>> for Vector2<T> {
    type Output = Vector2<T>;
    fn sub(self, right: Vector2<T>) -> Vector2<T> {
        Vector2::new(self.x - right.x, self.y - right.y)
    }
}

impl<T: ops::Add<Output = T> + Sized + Copy + NumCast> ops::Add<T> for Vector2<T> {
    type Output = Vector2<T>;
    fn add(self, right: T) -> Vector2<T> {
        Vector2::new(self.x + right, self.y + right)
    }
}

impl<T: ops::Mul<Output = T> + Sized + Copy + NumCast> ops::Mul<T> for Vector2<T> {
    type Output = Vector2<T>;
    fn mul(self, right: T) -> Vector2<T> {
        Vector2::new(self.x * right, self.y * right)
    }
}

impl<T: ops::Sub<Output = T> + Sized + Copy + NumCast> ops::Sub<T> for Vector2<T> {
    type Output = Vector2<T>;
    fn sub(self, right: T) -> Vector2<T> {
        Vector2::new(self.x - right, self.y - right)
    }
}


impl<T: ops::Add<Output = T> + Sized + Copy + NumCast> ops::AddAssign<Vector2<T>> for Vector2<T> {
    fn add_assign(&mut self, right: Vector2<T>) {
        *self = Vector2::new(self.x + right.x, self.y + right.y);
    }
}

impl<T: ops::Mul<Output = T> + Sized + Copy + NumCast> ops::MulAssign<Vector2<T>> for Vector2<T> {
    fn mul_assign(&mut self, right: Vector2<T>) {
        *self = Vector2::new(self.x * right.x, self.y * right.y);
    }
}

impl<T: ops::Sub<Output = T> + Sized + Copy + NumCast> ops::SubAssign<Vector2<T>> for Vector2<T> {
    fn sub_assign(&mut self, right: Vector2<T>) {
        *self = Vector2::new(self.x - right.x, self.y - right.y);
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Vector3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T : Sized + Copy + NumCast> Vector3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Vector3 { x, y, z }
    }

    pub fn new_vec2_z(xy: &Vector2<T>, z: T) -> Self {
        Vector3 { x: xy.x, y: xy.y, z: z }
    }

    pub fn new_x(x: T) -> Self where T: Default {
        Vector3 { x: x, y: T::default(), z: T::default() }
    }

    pub fn new_y(y: T) -> Self where T: Default {
        Vector3 { x: T::default(), y: y, z: T::default() }
    }

    pub fn new_z(z: T) -> Self where T: Default {
        Vector3 { x: T::default(), y: T::default(), z: z }
    }
    
    pub fn zeros() -> Self where T: Default {
        Vector3 { x: T::default(), y: T::default(), z: T::default() }
    }

    pub fn xy(&self) -> Vector2<T> {
        Vector2::new( self.x, self.y )
    }

    pub fn xz(&self) -> Vector2<T> {
        Vector2::new( self.x, self.z )
    }

    pub fn yx(&self) -> Vector2<T> {
        Vector2::new( self.y, self.x )
    }

    pub fn yz(&self) -> Vector2<T> {
        Vector2::new( self.y, self.z )
    }

    pub fn zx(&self) -> Vector2<T> {
        Vector2::new( self.z, self.x )
    }

    pub fn zy(&self) -> Vector2<T> {
        Vector2::new( self.z, self.y )
    }
}
    
impl<T : Sized + Copy + NumCast + Default> Vector3<T> {
    pub fn from< U:  Sized + Copy + NumCast + Default>(vec: &Vector3<U>) -> Vector3<T> {
        Vector3::<T>{ 
            x: NumCast::from(vec.x).unwrap_or_default(),
            y: NumCast::from(vec.y).unwrap_or_default(),
            z: NumCast::from(vec.z).unwrap_or_default(),
        }
    }
}

impl<T: Float> Vector3<T> {
    pub fn normalize(&self) -> Vector3<T> {
        let length = self.dot(&self).sqrt();
        Vector3 { x: self.x / length, y: self.y / length, z: self.z / length }
    }

    pub fn distance(&self, right: &Vector3<T>) -> T {
        let diff = *self - *right;
        diff.dot(&diff).sqrt()
    } 
    
    pub fn magnitude(&self) -> T {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    pub fn round(&self) -> Vector3<T> {
        Vector3 { x: self.x.round(), y: self.y.round(), z: self.z.round() }
    }
}

impl<T: ops::Add<Output = T> + ops::Mul<Output = T> + ops::Sub<Output = T> + Sized + Copy + NumCast> Vector3<T> {
    pub fn dot(&self, right: &Vector3<T>) -> T {
        return self.x * right.x + self.y * right.y + self.z * right.z;
    }

    pub fn cross(&self, right: &Vector3<T>) -> Vector3<T> {
        Vector3::new(
            self.y * right.z - self.z * right.y, 
            self.z * right.x - self.x * right.z, 
            self.x * right.y - self.y * right.x
        )
    }
}

impl<T: ops::Add<Output = T> + Sized + Copy + NumCast> ops::Add<Vector3<T>> for Vector3<T> {
    type Output = Vector3<T>;
    fn add(self, right: Vector3<T>) -> Vector3<T> {
        Vector3::new(self.x + right.x, self.y + right.y, self.z + right.z)
    }
}

impl<T: ops::Mul<Output = T> + Sized + Copy + NumCast> ops::Mul<Vector3<T>> for Vector3<T> {
    type Output = Vector3<T>;
    fn mul(self, right: Vector3<T>) -> Vector3<T> {
        Vector3::new(self.x * right.x, self.y * right.y, self.z * right.z)
    }
}

impl<T: ops::Sub<Output = T> + Sized + Copy + NumCast> ops::Sub<Vector3<T>> for Vector3<T> {
    type Output = Vector3<T>;
    fn sub(self, right: Vector3<T>) -> Vector3<T> {
        Vector3::new(self.x - right.x, self.y - right.y, self.z - right.z)
    }
}

impl<T: ops::Add<Output = T> + Sized + Copy + NumCast> ops::Add<T> for Vector3<T> {
    type Output = Vector3<T>;
    fn add(self, right: T) -> Vector3<T> {
        Vector3::new(self.x + right, self.y + right, self.z + right)
    }
}

impl<T: ops::Mul<Output = T> + Sized + Copy + NumCast> ops::Mul<T> for Vector3<T> {
    type Output = Vector3<T>;
    fn mul(self, right: T) -> Vector3<T> {
        Vector3::new(self.x * right, self.y * right, self.z * right)
    }
}

impl<T: ops::Sub<Output = T> + Sized + Copy + NumCast> ops::Sub<T> for Vector3<T> {
    type Output = Vector3<T>;
    fn sub(self, right: T) -> Vector3<T> {
        Vector3::new(self.x - right, self.y - right, self.z - right)
    }
}

impl<T: ops::Add<Output = T> + Sized + Copy + NumCast> ops::AddAssign<Vector3<T>> for Vector3<T> {
    fn add_assign(&mut self, right: Vector3<T>) {
        *self = Vector3::new(self.x + right.x, self.y + right.y, self.z + right.z);
    }
}

impl<T: ops::Mul<Output = T> + Sized + Copy + NumCast> ops::MulAssign<Vector3<T>> for Vector3<T> {
    fn mul_assign(&mut self, right: Vector3<T>) {
        *self = Vector3::new(self.x * right.x, self.y * right.y, self.z * right.z);
    }
}

impl<T: ops::Sub<Output = T> + Sized + Copy + NumCast> ops::SubAssign<Vector3<T>> for Vector3<T> {
    fn sub_assign(&mut self, right: Vector3<T>) {
        *self = Vector3::new(self.x - right.x, self.y - right.y, self.z - right.z);
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Vector4<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

impl<T : Sized + Copy + NumCast> Vector4<T> {
    pub fn new(x: T, y: T, z: T, w: T) -> Self {
        Vector4 { x, y, z, w }
    }

    pub fn new_vec2_zw(xy: &Vector2<T>, z: T, w: T) -> Self {
        Vector4 { x: xy.x, y: xy.y, z: z, w: w }
    }

    pub fn new_vec3_w(xyz: &Vector3<T>, w: T) -> Self {
        Vector4 { x: xyz.x, y: xyz.y, z: xyz.z, w: w }
    }

    pub fn new_x(x: T) -> Self where T: Default {
        Vector4 { x: x, y: T::default(), z: T::default(), w: T::default() }
    }

    pub fn new_y(y: T) -> Self where T: Default {
        Vector4 { x: T::default(), y: y, z: T::default(), w: T::default() }
    }

    pub fn new_z(z: T) -> Self where T: Default {
        Vector4 { x: T::default(), y: T::default(), z: z, w: T::default() }
    }

    pub fn new_w(w: T) -> Self where T: Default {
        Vector4 { x: T::default(), y: T::default(), z: T::default(), w: w }
    }
    
    pub fn zeros() -> Self where T: Default {
        Vector4 { x: T::default(), y: T::default(), z: T::default(), w: T::default() }
    }

    pub fn xy(&self) -> Vector2<T> {
        Vector2::new( self.x, self.y )
    }

    pub fn xz(&self) -> Vector2<T> {
        Vector2::new( self.x, self.z )
    }

    pub fn yx(&self) -> Vector2<T> {
        Vector2::new( self.y, self.x )
    }

    pub fn yz(&self) -> Vector2<T> {
        Vector2::new( self.y, self.z )
    }

    pub fn zx(&self) -> Vector2<T> {
        Vector2::new( self.z, self.x )
    }

    pub fn zy(&self) -> Vector2<T> {
        Vector2::new( self.z, self.y )
    }

    pub fn xw(&self) -> Vector2<T> {
        Vector2::new( self.x, self.w )
    }
    
    pub fn yw(&self) -> Vector2<T> {
        Vector2::new( self.y, self.w )
    }
    
    pub fn zw(&self) -> Vector2<T> {
        Vector2::new( self.z, self.w )
    }
    pub fn wx(&self) -> Vector2<T> {
        Vector2::new( self.w, self.x )
    }
    
    pub fn wy(&self) -> Vector2<T> {
        Vector2::new( self.w, self.y )
    }
    
    pub fn wz(&self) -> Vector2<T> {
        Vector2::new( self.w, self.z )
    }
    
    pub fn xyz(&self) -> Vector3<T> {
        Vector3::new( self.x, self.y, self.z )
    }

    pub fn xyw(&self) -> Vector3<T> {
        Vector3::new( self.x, self.y, self.w )
    }

    pub fn xzw(&self) -> Vector3<T> {
        Vector3::new( self.x, self.z, self.w )
    }

    pub fn yxw(&self) -> Vector3<T> {
        Vector3::new( self.y, self.x, self.w )
    }

    pub fn yzw(&self) -> Vector3<T> {
        Vector3::new( self.y, self.z, self.w )
    }

    pub fn zxy(&self) -> Vector3<T> {
        Vector3::new( self.z, self.x, self.y )
    }

    pub fn zyw(&self) -> Vector3<T> {
        Vector3::new( self.z, self.y, self.w )
    }

    pub fn wxz(&self) -> Vector3<T> {
        Vector3::new( self.w, self.x, self.z )
    }

    pub fn wyz(&self) -> Vector3<T> {
        Vector3::new( self.w, self.y, self.z )
    }
}
    
impl<T : Sized + Copy + NumCast + Default> Vector4<T> {
    pub fn from< U:  Sized + Copy + NumCast + Default>(vec: &Vector4<U>) -> Vector4<T> {
        Vector4::<T>{ 
            x: NumCast::from(vec.x).unwrap_or_default(),
            y: NumCast::from(vec.y).unwrap_or_default(),
            z: NumCast::from(vec.z).unwrap_or_default(),
            w: NumCast::from(vec.w).unwrap_or_default(),
        }
    }
}

impl<T: Float> Vector4<T> {
    pub fn normalize(&self) -> Vector4<T> {
        let length = self.dot(&self).sqrt();
        Vector4 { x: self.x / length, y: self.y / length, z: self.z / length, w: self.w / length }
    }

    pub fn distance(&self, right: &Vector4<T>) -> T {
        let diff = *self - *right;
        diff.dot(&diff).sqrt()
    }
    
    pub fn magnitude(&self) -> T {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2) + self.w.powi(2)).sqrt()
    }
    
    pub fn round(&self) -> Vector4<T> {
        Vector4 { x: self.x.round(), y: self.y.round(), z: self.z.round(), w: self.w.round() }
    }
}

impl<T: ops::Add<Output = T> + ops::Mul<Output = T> + ops::Sub<Output = T> + Sized + Copy + NumCast> Vector4<T> {
    pub fn dot(&self, right: &Vector4<T>) -> T {
        return self.x * right.x + self.y * right.y + self.z * right.z + self.w * right.w;
    }
}

impl<T: ops::Add<Output = T> + Sized + Copy + NumCast> ops::Add<Vector4<T>> for Vector4<T> {
    type Output = Vector4<T>;
    fn add(self, right: Vector4<T>) -> Vector4<T> {
        Vector4::new(self.x + right.x, self.y + right.y, self.z + right.z, self.w + right.w)
    }
}

impl<T: ops::Mul<Output = T> + Sized + Copy + NumCast> ops::Mul<Vector4<T>> for Vector4<T> {
    type Output = Vector4<T>;
    fn mul(self, right: Vector4<T>) -> Vector4<T> {
        Vector4::new(self.x * right.x, self.y * right.y, self.z * right.z, self.w * right.w)
    }
}

impl<T: ops::Sub<Output = T> + Sized + Copy + NumCast> ops::Sub<Vector4<T>> for Vector4<T> {
    type Output = Vector4<T>;
    fn sub(self, right: Vector4<T>) -> Vector4<T> {
        Vector4::new(self.x - right.x, self.y - right.y, self.z - right.z, self.w - right.w)
    }
}

impl<T: ops::Add<Output = T> + Sized + Copy + NumCast> ops::Add<T> for Vector4<T> {
    type Output = Vector4<T>;
    fn add(self, right: T) -> Vector4<T> {
        Vector4::new(self.x + right, self.y + right, self.z + right, self.w + right)
    }
}

impl<T: ops::Mul<Output = T> + Sized + Copy + NumCast> ops::Mul<T> for Vector4<T> {
    type Output = Vector4<T>;
    fn mul(self, right: T) -> Vector4<T> {
        Vector4::new(self.x * right, self.y * right, self.z * right, self.w * right)
    }
}

impl<T: ops::Sub<Output = T> + Sized + Copy + NumCast> ops::Sub<T> for Vector4<T> {
    type Output = Vector4<T>;
    fn sub(self, right: T) -> Vector4<T> {
        Vector4::new(self.x - right, self.y - right, self.z - right, self.w - right)
    }
}

impl<T: ops::Add<Output = T> + Sized + Copy + NumCast> ops::AddAssign<Vector4<T>> for Vector4<T> {
    fn add_assign(&mut self, right: Vector4<T>) {
        *self = Vector4::new(self.x + right.x, self.y + right.y, self.z + right.z, self.w + right.w);
    }
}

impl<T: ops::Mul<Output = T> + Sized + Copy + NumCast> ops::MulAssign<Vector4<T>> for Vector4<T> {
    fn mul_assign(&mut self, right: Vector4<T>) {
        *self = Vector4::new(self.x * right.x, self.y * right.y, self.z * right.z, self.w * right.w);
    }
}

impl<T: ops::Sub<Output = T> + Sized + Copy + NumCast> ops::SubAssign<Vector4<T>> for Vector4<T> {
    fn sub_assign(&mut self, right: Vector4<T>) {
        *self = Vector4::new(self.x - right.x, self.y - right.y, self.z - right.z, self.w - right.w);
    }
}

pub fn max<T : std::cmp::PartialOrd>(value1:T, value2: T) -> T {
    if value1 < value2 {
        return value2;
    }
   return value1;
}

pub fn min<T : std::cmp::PartialOrd>(value1:T, value2: T) -> T {
    if value2 < value1 {
        return value2;
    }
   return value1;
}

pub fn clamp<T : std::cmp::PartialOrd>(value:T,min:T,max:T) -> T {
    if value < min {
        return min;
    } else if value > max {
        return max;
    }
   return value;
}

pub fn lerp<T: Float>(start: T, end: T, alpha: T) -> T
{
    start + (end - start) * alpha
}

pub fn no_negative<T : std::cmp::PartialOrd + Default>(value:T) -> T {
    if value < T::default() {
        return T::default();
    }
   return value;
}

pub fn radians<T: Float + NumCast + Default>(degrees: T) -> T {
    let pi: T = T::from(std::f64::consts::PI).unwrap_or_default();
    degrees * (pi / T::from(180.0).unwrap())
}

pub fn degrees<T: Float + NumCast + Default>(radians: T) -> T {
    let pi: T = T::from(std::f64::consts::PI).unwrap_or_default();
    radians * (T::from(180.0).unwrap() / pi)
}

pub fn angle<T: Float + NumCast>(v1: &Vector2<T>, v2: &Vector2<T>) -> T {
    let delta = *v2 - *v1;
    (delta.y).atan2(delta.x)
}

pub fn normalize_radians<T: Float + FloatConst + NumCast + Default + ops::AddAssign>(angle: T) -> T {
    let two_pi = T::PI() * T::from(2.0).unwrap();
    let mut normalized_angle = angle % two_pi;
    if normalized_angle < T::default() {
        normalized_angle += two_pi;
    }
    normalized_angle
}

pub fn normalize_degrees<T: Float + NumCast + Default + ops::AddAssign>(angle: T) -> T {
    let mut normalized_angle = angle % T::from(360.0).unwrap();
    if normalized_angle < T::default() {
        normalized_angle += T::from(360.0).unwrap();
    }
    normalized_angle
}

lazy_static! {
    
    pub static ref SIN: [f32; 360] = {
        let mut sin_values = [0.0; 360];
        for i in 0..360 {
            sin_values[i] = (i as f32 * std::f32::consts::PI / 180.0).sin();
        }
        sin_values
    };
    
    pub static ref COS: [f32; 360] = {
        let mut cos_values = [0.0; 360];
        for i in 0..360 {
            cos_values[i] = (i as f32 * std::f32::consts::PI / 180.0).cos();
        }
        cos_values
    };

}
