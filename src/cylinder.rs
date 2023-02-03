use crate::hit::*;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Debug)]
pub struct Cylinder {
    center: Vec3,
    radius: f64,
    height: f64,
    material: Material,
    
}

// #[derive(Debug)]
// pub struct XZCircle {
//     center: Vec3,
//     radius: f64,
//     height: f64,
//     material: Material
// }

impl Cylinder {
    pub fn new(center: Vec3, radius: f64, height: f64, material: Material) -> Cylinder {
        Cylinder {
            center,
            radius,
            height,
            material,
        }
    }
}

impl Hittable for Cylinder {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        // let t = (0.0001 - r.origin.y()) / r.direction.y();
        let oc = r.origin - self.center;

        let oc_xz = Vec3::new(
            r.origin.x() - self.center.x(), 
            0.0,
            r.origin.z() - self.center.z() 
        );


        let a = r.direction.dot_xz(&r.direction);
        let b = oc_xz.dot_xz(&r.direction);
        let c = oc_xz.dot_xz(&oc_xz) - self.radius * self.radius;
        // let a = r.direction.x().powi(2) + r.direction.z().powi(2);
        // let b = 2.0 * (r.origin.x() - self.center.x()) * r.direction.x()
        //     + 2.0 * (r.origin.z() - self.center.z()) * r.direction.z();
        // let c = (r.origin.x() - self.center.x()).powi(2) + (r.origin.z() - self.center.z()).powi(2) - self.radius.powi(2);


        let a_xy = Vec3::dot(&r.direction, &r.direction);
        let b_xy = Vec3::dot(&oc, &r.direction);
        let c_xy = Vec3::dot(&oc, &oc) - (self.height * self.radius);
        
        // let a_xy = r.direction.dot_xz(&r.direction);
        // let b_xy = oc.dot_xz(&r.direction);
        // let c_xy = oc.dot_xz(&oc) - self.radius * self.radius;
        

        let discriminant = b * b - a * c;
        let discriminant_xy = b_xy * b_xy - a_xy * c_xy;
        if discriminant > 0.0 {
            // if oc.y() < self.center.y() + self.height {
            //     return None;
            // }
            let mut temp = (-b - discriminant.sqrt()) / a;
            let mut y = r.origin.y() + temp * r.direction.y();

            if temp < t_max && temp > t_min && y > self.center.y() && y <= self.center.y() + self.height {
                return Some(HitRecord {
                    t: temp,
                    point: r.at(temp),
                    normal: (r.at(temp) - self.center) / self.radius,
                    u: 0.0,
                    v: 0.0,
                    material: self.material,
                });
            }
            temp = (-b + discriminant.sqrt()) / a;
            y = r.origin.y() + temp * r.direction.y();

            if temp < t_max && temp > t_min && y > self.center.y() && y <= self.center.y() + self.height {
                return Some(HitRecord {
                    t: temp,
                    point: r.at(temp),
                    normal: (r.at(temp) - self.center) / self.radius,
                    u: 0.0,
                    v: 0.0,
                    material: self.material,
                });
            }
            
        }
        None
    }
}


// impl XZCircle {
//     pub fn new(center: Vec3, radius: f64, material: Material) -> Self {
//         Self {
//             center,
//             radius,
//             height: 0.0,
//             material,
//         }
//     }
// }

// impl Hittable for XZCircle {
//     fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
//         let t = (self.height - r.origin.y()) / r.direction.y();

//         if t < t_min || t > t_max {
//             return None;
//         }

//         let oc = r.origin - self.center;
//         let a = Vec3::dot(&r.direction, &r.direction);
//         let b = Vec3::dot(&oc, &r.direction);
//         let c = Vec3::dot(&oc, &oc) - self.radius * self.radius;

//         let discriminant = b * b - a * c;

//         if discriminant > 0.0 {
//             let mut temp = (-b - discriminant.sqrt()) / a;
//             if temp < t_max && temp > t_min {
//                 return Some(HitRecord {
//                     t: temp,
//                     point: r.at(temp),
//                     normal: (r.at(temp) - self.center) / self.radius,
//                     u: 0.0,
//                     v: 0.0,
//                     material: self.material,
//                 });
//             }
//             temp = (-b + discriminant.sqrt()) / a;
//             if temp < t_max && temp > t_min {
//                 return Some(HitRecord {
//                     t: temp,
//                     point: r.at(temp),
//                     normal: (r.at(temp) - self.center) / self.radius,
//                     u: 0.0,
//                     v: 0.0,
//                     material: self.material,
//                 });
//             }
//         }

//         // let x = r.origin.x() + t * r.direction.x();
//         // let z = r.origin.z() + t * r.direction.z();

//         // if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
//         //     return None;
//         // }

//         // let outward_normal = Vec3::new(0.0, 1.0, 0.0);
//         // let mut rec = HitRecord {
//         //     t,
//         //     point: r.at(t),
//         //     normal: outward_normal,
//         //     u: (x - self.x0) / (self.x1 - self.x0),
//         //     v: (z - self.z0) / (self.z1 - self.z0),
//         //     material: self.material,
//         // };
//         // rec.set_face_normal(r, &outward_normal);

//         // Some(rec)
//         None
//     }
// }
