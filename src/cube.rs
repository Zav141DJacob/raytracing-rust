use crate::hit::*;
use crate::material::Material;
use crate::ray::Ray;
use crate::rect::XYRect;
use crate::rect::XZRect;
use crate::rect::YZRect;
use crate::vec3::Vec3;

#[derive(Debug)]
pub struct Cube {
    pub minimum: Vec3,
    pub maximum: Vec3,
    pub sides: HittableList,
}

impl Cube {
    pub fn new(p0: Vec3, p1: Vec3, mat: Material) -> Cube {
        let sides = HittableList::new(vec![
            Box::new(XYRect::new(p0.x(), p1.x(), p0.y(), p1.y(), p1.z(), mat)),
            Box::new(XYRect::new(p0.x(), p1.x(), p0.y(), p1.y(), p0.z(), mat)),
            Box::new(XZRect::new(p0.x(), p1.x(), p0.z(), p1.z(), p1.y(), mat)),
            Box::new(XZRect::new(p0.x(), p1.x(), p0.z(), p1.z(), p0.y(), mat)),
            Box::new(YZRect::new(p0.y(), p1.y(), p0.z(), p1.z(), p1.x(), mat)),
            Box::new(YZRect::new(p0.y(), p1.y(), p0.z(), p1.z(), p0.x(), mat)),
        ]);

        Cube {
            minimum: p0,
            maximum: p1,
            sides,
        }
    }
}

impl Hittable for Cube {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.sides.hit(r, t_min, t_max)
    }
}
