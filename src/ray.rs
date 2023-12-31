use cgmath::Vector3;

// allows using println!("{:?}", ray);
#[derive(Debug)]
pub struct Ray {
    pub origin: Vector3<f32>,
    pub direction: Vector3<f32>,
}

impl Ray {
    pub fn new(origin: Vector3<f32>, direction: Vector3<f32>) -> Self {
        Self { origin, direction }
    }

    pub fn eval_at(&self, param: f32) -> Vector3<f32> {
        return self.origin + self.direction * param;
    }
}
