use crate::{geometries::hittable::Hittable, materials::material::Material};

pub struct MaterialHittable {
    hittable: Box<dyn Hittable>,
    material: Box<dyn Material>,
}

impl MaterialHittable {
    pub   fn new(hittable: Box<dyn Hittable>, material: Box<dyn Material>) -> Self {
        Self {
            hittable: hittable,
            material: material,
        }
    }
}

impl Hittable for MaterialHittable {
    fn test_ray(
        &self,
        ray: &crate::ray::Ray,
        min_t: f32,
        max_t: f32,
    ) -> Option<crate::geometries::hittable::Hit> {
        let option_hit = self.hittable.test_ray(ray, min_t, max_t);
        if option_hit.is_none() {
            return None;
        }

        let mut hit = option_hit.unwrap();
        hit.material = Some(&*self.material);
        return Some(hit);
    }
}
