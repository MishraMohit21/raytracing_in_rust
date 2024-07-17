use crate::vec3;

pub struct Ray {
    origin: vec3::Point3,
    direction: vec3::Vec3,
}

impl Ray {

    pub fn new(org: vec3::Point3, dir: vec3::Vec3) -> Ray {
        Ray {
            origin: org,
            direction: dir,
        }
    }

    pub fn get_origin(&self) -> &vec3::Point3 { &self.origin }
    pub fn get_direction(&self) -> &vec3::Vec3 { &self.direction }

    pub fn at(&self, t: f64) -> vec3::Point3 { self.origin * t + self.direction }

}