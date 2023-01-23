use crate::hit::HitRecord;
use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::color::Color;
use rand::prelude::*;

#[allow(dead_code)]
#[derive(Copy, Clone, Debug)]
pub enum Material {
    Lambertian { albedo: Color },
    Metal { albedo: Color },
    Dielectric { ref_idx: f64 },
}

impl Default for Material {
    fn default() -> Self {
        Material::Lambertian { albedo: Color::default() }
    }
}

pub fn scatter(material: &Material, ray_in: &Ray, rec: &HitRecord, attentuation: &mut Color, scattered: &mut Ray) -> bool {
    match material {
        &Material::Lambertian { albedo } => {
            let target = rec.point + rec.normal + random_in_unit_sphere();
            *scattered = Ray::new(rec.point, target - rec.point);
            *attentuation = albedo;
            return true;
        }
        &Material::Metal { albedo} => {
            let reflected = reflect(&Vec3::unit_vector(&ray_in.direction), &rec.normal);
            *scattered = Ray::new(rec.point, reflected);
            *attentuation = albedo;
            return Vec3::dot(&scattered.direction, &rec.normal) > 0.0;
        }
        &Material::Dielectric { ref_idx } => {
            let outward_normal: Vec3;
            let reflected = reflect(&ray_in.direction, &rec.normal);
            let ni_over_nt:f64;
            *attentuation = Color::new(1.0, 1.0, 1.0);
            let mut refracted = Vec3::default();

            let reflect_prob: f64;
            let cosine: f64;

            if Vec3::dot(&ray_in.direction, &rec.normal) > 0.0 {
                outward_normal = -rec.normal;
                ni_over_nt = ref_idx;
                cosine = ref_idx * Vec3::dot(&ray_in.direction, &rec.normal) / ray_in.direction.length();
            } else {
                outward_normal = rec.normal;
                ni_over_nt = 1.0 / ref_idx;
                cosine = - Vec3::dot(&ray_in.direction, &rec.normal) / ray_in.direction.length();
            }

            if refract(&ray_in.direction, &outward_normal, ni_over_nt, &mut refracted) {
                reflect_prob = schlick(cosine, ref_idx);
            } else {
                reflect_prob = 1.0;
            }

            let mut rng = rand::thread_rng();
            if rng.gen::<f64>() < reflect_prob {
                *scattered = Ray::new(rec.point, reflected);
            } else {
                *scattered = Ray::new(rec.point, refracted);
            }

            return true;
        }
    }
}

pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    return *v - 2.0 * Vec3::dot(v, n) * *n;
}

pub fn refract(v: &Vec3, n: &Vec3, ni_over_nt: f64, refracted: &mut Vec3) -> bool {
    let uv = Vec3::unit_vector(v);
    let dt = Vec3::dot(&uv, n);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);

    if discriminant > 0.0 {
        *refracted = ni_over_nt * (uv - *n * dt) - *n * discriminant.sqrt();
        return true
    }
    return false
}

fn schlick(cosine: f64, ref_idx: f64) -> f64 {
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;
    return r0 + (1.0 - r0) * (1.0 - cosine).powi(5);
}

fn random_in_unit_sphere() -> Vec3 {
    let mut p: Vec3;
    let mut rng = rand::thread_rng();

    loop {
        p = 2.0 * Vec3::new(rng.gen::<f64>(), rng.gen::<f64>(), 0.0) - Vec3::new(1.0, 1.0, 0.0);

        if p.squared_length() < 1.0 {
            return p;
        }
    }
}
