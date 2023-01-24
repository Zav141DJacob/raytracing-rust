use crate::hit::*;
use crate::material::Material;
use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::hit::*;
use crate::rect::XYRect;
use crate::rect::XZRect;
use crate::rect::YZRect;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Cube {
    pub minimum: Vec3,
    pub maximum: Vec3,
    pub sides: HittableList,
}

impl Cube {
    pub fn new(p0: Vec3, p1: Vec3, mat: Material) -> Cube {
        let mut list: Vec<Box<dyn Hittable>> = Vec::new();
        list.push(Box::new(XYRect::xy_rect(p0.x(), p1.x(), p0.y(), p1.y(), p1.z(), mat)));
        list.push(Box::new(XYRect::xy_rect(p0.x(), p1.x(), p0.y(), p1.y(), p0.z(), mat)));

        list.push(Box::new(XZRect::xz_rect(p0.x(), p1.x(), p0.z(), p1.z(), p1.y(), mat)));
        list.push(Box::new(XZRect::xz_rect(p0.x(), p1.x(), p0.z(), p1.z(), p0.y(), mat)));

        list.push(Box::new(YZRect::yz_rect(p0.y(), p1.y(), p0.z(), p1.z(), p1.x(), mat)));
        list.push(Box::new(YZRect::yz_rect(p0.y(), p1.y(), p0.z(), p1.z(), p0.x(), mat)));
        let mut sides = HittableList::new(list);

        Cube{minimum: p0, maximum: p1, sides}
    }
}

#[typetag::serde(name = "Cube")]
impl Hittable for Cube {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.sides.hit(r, t_min, t_max)
    }
}