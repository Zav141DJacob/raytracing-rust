use crate::hit::*;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct XYRect {
    x0: f64,
    x1: f64,
    y0: f64,
    y1: f64,
    k: f64,
    mat: Material,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct XZRect {
    x0: f64,
    x1: f64,
    z0: f64,
    z1: f64,
    k: f64,
    mat: Material,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct YZRect {
    y0: f64,
    y1: f64,
    z0: f64,
    z1: f64,
    k: f64,
    mat: Material,
}

impl XYRect {
    pub fn new(x0: f64, x1: f64, y0: f64, y1: f64, k: f64, mat: Material) -> XYRect {
        XYRect {
            x0,
            x1,
            y0,
            y1,
            k,
            mat,
        }
    }
}

impl XZRect {
    pub fn new(x0: f64, x1: f64, z0: f64, z1: f64, k: f64, mat: Material) -> XZRect {
        XZRect {
            x0,
            x1,
            z0,
            z1,
            k,
            mat,
        }
    }
}

impl YZRect {
    pub fn new(y0: f64, y1: f64, z0: f64, z1: f64, k: f64, mat: Material) -> YZRect {
        YZRect {
            y0,
            y1,
            z0,
            z1,
            k,
            mat,
        }
    }
}

impl Hittable for XYRect {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k - r.origin.z()) / r.direction.z();

        if t < t_min || t > t_max {
            return None;
        }

        let x = r.origin.x() + t * r.direction.x();
        let y = r.origin.y() + t * r.direction.y();

        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return None;
        }

        let outward_normal = Vec3::new(0.0, 0.0, 1.0);
        let mut rec = HitRecord {
            t,
            point: r.at(t),
            normal: outward_normal,
            u: (x - self.x0) / (self.x1 - self.x0),
            v: (y - self.y0) / (self.y1 - self.y0),
            material: self.mat,
        };
        rec.set_face_normal(r, &outward_normal);

        Some(rec)
    }
}

impl Hittable for XZRect {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k - r.origin.y()) / r.direction.y();

        if t < t_min || t > t_max {
            return None;
        }

        let x = r.origin.x() + t * r.direction.x();
        let z = r.origin.z() + t * r.direction.z();

        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
            return None;
        }

        let outward_normal = Vec3::new(0.0, 1.0, 0.0);
        let mut rec = HitRecord {
            t,
            point: r.at(t),
            normal: outward_normal,
            u: (x - self.x0) / (self.x1 - self.x0),
            v: (z - self.z0) / (self.z1 - self.z0),
            material: self.mat,
        };
        rec.set_face_normal(r, &outward_normal);

        Some(rec)
    }
}

impl Hittable for YZRect {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k - r.origin.x()) / r.direction.x();

        if t < t_min || t > t_max {
            return None;
        }

        let y = r.origin.y() + t * r.direction.y();
        let z = r.origin.z() + t * r.direction.z();

        if y < self.y0 || y > self.y1 || z < self.z0 || z > self.z1 {
            return None;
        }

        let outward_normal = Vec3::new(1.0, 0.0, 0.0);
        let mut rec = HitRecord {
            t,
            point: r.at(t),
            normal: outward_normal,
            u: (y - self.y0) / (self.y1 - self.y0),
            v: (z - self.z0) / (self.z1 - self.z0),
            material: self.mat,
        };
        rec.set_face_normal(r, &outward_normal);

        Some(rec)
    }
}
