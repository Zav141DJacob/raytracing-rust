use crate::Vec3;
use crate::ray::Ray;
use crate::material::Material;

#[derive(Default)]
pub struct HitRecord {
    pub t: f64,
    pub point: Vec3,
    pub normal: Vec3,
    pub material: Material
}

pub trait Hittable {
    fn hit(&self, _ray: &Ray, _t_min: f64, _t_max: f64) -> Option<HitRecord> {
        None
    }
}

pub struct HittableList {
    list: Vec<Box<dyn Hittable>>
}

impl HittableList {
    pub fn new(list: Vec<Box<dyn Hittable>>) -> HittableList {
        HittableList { list }
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut hit_record = None;
        let mut closest_so_far = t_max;

        for object in &self.list {
            if let Some(rec) = object.hit(ray, t_min, closest_so_far) {
                closest_so_far = rec.t;
                hit_record = Some(rec);
            }
        }
        return hit_record;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::Color;

    #[test]
    fn test_hitrecord() {
        let hit_record = HitRecord {
            t: 1.0,
            point: Vec3(1.0, 2.0, 3.0),
            normal: Vec3(0.0, 0.0, 1.0),
            material: Material::Lambertian { albedo: Color::default() }
        };
        assert_eq!(hit_record.point, Vec3(1.0, 2.0, 3.0));
        assert_eq!(hit_record.t, 1.0);
        assert_eq!(hit_record.normal, Vec3(0.0, 0.0, 1.0));
    }
}