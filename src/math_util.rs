use std::ops::{Add,AddAssign,Sub,SubAssign,Mul,MulAssign,Div,DivAssign,Neg};

use rand::{prelude::ThreadRng, Rng};

#[derive(Debug,Copy,Clone,Default)]
pub struct Vec3{
    pub x:f64,
    pub y:f64,
    pub z:f64,
}

pub type Color=Vec3;
pub type Point3=Vec3;

impl Vec3{
    pub fn from(x:f64,y:f64,z:f64)->Self{
        Self {x,y,z}
    }
    pub fn length(&self)->f64{
        f64::sqrt(self.x*self.x+self.y*self.y+self.z*self.z)
    }
    pub fn length_squared(&self)->f64{
        self.x*self.x+self.y*self.y+self.z*self.z
    }
    pub fn get_x(&self)->f64{
        self.x
    }
    pub fn get_y(&self)->f64{
        self.y
    }
    pub fn get_z(&self)->f64{
        self.z
    }
    pub fn unit_vector(&self)->Vec3{
        *self/self.length()
    }
    pub fn random_range(l:f64,h:f64,rng:&mut ThreadRng)->Self{
        Vec3 {x:rng.gen_range(l..h), y:rng.gen_range(l..h), z:rng.gen_range(l..h)}
    }
    pub fn random_in_unit_sphere(rng:&mut ThreadRng)->Self{
        loop{
            let p=Vec3::random_range(-1.0, 1.0, rng);
            if p.length_squared()<1.0 {return p;}
        }
    }
    pub fn random_unit_vector(rng:&mut ThreadRng)->Self{
        Self::random_in_unit_sphere(rng).unit_vector()
    }
    //to vector of u8 with gamma correction
    pub fn to_uvec(&self)->[u8;3]{
        let r=self.x.sqrt();
        let g=self.y.sqrt();
        let b=self.z.sqrt();
        [f64::min(255.0,r*256.0)as u8,f64::min(255.0,g*256.0)as u8,f64::min(255.0,b*256.0)as u8]
    }
}

impl Add for Vec3{
    type Output=Self;
    fn add(self, rhs: Self)-> Self{
        Self{x:(self.x+rhs.x),y:(self.y+rhs.y),z:(self.z+rhs.z)}
    }
}

impl AddAssign for Vec3{
    fn add_assign(&mut self,rhs:Self){
        *self=Self{
            x:self.x+rhs.x,
            y:self.y+rhs.y,
            z:self.z+rhs.z
        };
    }
}

impl Sub for Vec3{
    type Output=Self;
    fn sub(self, rhs: Self)-> Self{
        Self{x:(self.x-rhs.x),y:(self.y-rhs.y),z:(self.z-rhs.z)}
    }
}

impl SubAssign for Vec3{
    fn sub_assign(&mut self,rhs:Self){
        *self=Self{
            x:self.x-rhs.x,
            y:self.y-rhs.y,
            z:self.z-rhs.z
        };
    }
}

impl Mul<f64> for Vec3{
    type Output = Self;
    fn mul(self, rhs: f64)-> Self{
        Self{
            x:(self.x*rhs),
            y:(self.y*rhs),
            z:(self.z*rhs)
        }
    }
}

impl Mul<Vec3> for f64{
    type Output = Vec3;
    fn mul(self,rhs:Vec3)->Vec3{
        rhs*self
    }
}

impl MulAssign<f64> for Vec3{
    fn mul_assign(&mut self,rhs:f64){
        *self=Self{
            x:self.x*rhs,
            y:self.y*rhs,
            z:self.z*rhs
        }
    }
}

impl Div<f64> for Vec3{
    type Output = Self;
    fn div(self, rhs: f64)-> Self{
        Self{
            x:(self.x/rhs),
            y:(self.y/rhs),
            z:(self.z/rhs)
        }
    }
}

impl Div<Vec3> for f64{
    type Output = Vec3;
    fn div(self,rhs:Vec3)->Vec3{
        rhs/self
    }
}

impl DivAssign<f64> for Vec3{
    fn div_assign(&mut self, rhs: f64) {
        *self=Self{
            x:self.x/rhs,
            y:self.y/rhs,
            z:self.z/rhs
        }
    }
}

impl Neg for Vec3{
    type Output = Self;
    fn neg(self) -> Self {
        Self { x: -self.x, y: -self.y, z: -self.z }
    }
}

pub fn dot(u:&Vec3,v:&Vec3)->f64{
    u.x*v.x+u.y*v.y+u.z*v.z
}
pub fn cross(u:&Vec3,v:&Vec3)->Vec3{
    Vec3 {
        x: u.y*v.z-u.z*v.y,
        y: u.z*v.x-u.x*v.z,
        z: u.x*v.y-u.y*v.x
    }
}

#[derive(Debug,Default,Copy,Clone)]
pub struct Ray{
    orig:Point3,
    dir:Vec3,
}

impl Ray{
    pub fn from(orig:&Point3,dir:&Vec3)->Ray{
        Ray {
            orig:*orig,
            dir:*dir
        }
    }
    pub fn get_origin(&self)->&Point3{
        &self.orig
    }
    pub fn get_direction(&self)->&Vec3{
        &self.dir
    }
    pub fn at(&self,t:f64)->Point3{
        self.orig+self.dir*t
    }
}

pub fn clamp(x:f64,min:f64,max:f64)->f64{
    if x<min {min} else {if x>max {max} else {x}}
}

