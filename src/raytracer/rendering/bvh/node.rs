use crate::maths::vec3::Vec3;
use crate::rendering::aabb::Aabb;

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct BvhNode {
    pub min: [f32; 3],
    pub data0: u32,
    pub max: [f32; 3],
    pub count: u32,
}

impl BvhNode {
    #[inline(always)]
    pub fn is_leaf(&self) -> bool {
        self.count > 0
    }

    #[inline(always)]
    pub fn aabb(&self) -> Aabb {
        Aabb {
            min: Vec3::from_xyz(self.min[0], self.min[1], self.min[2]),
            max: Vec3::from_xyz(self.max[0], self.max[1], self.max[2]),
        }
    }

    #[inline(always)]
    pub fn set_data(&mut self, bbox: Aabb, data0: u32, count: u32) {
        self.min = [bbox.min.x, bbox.min.y, bbox.min.z];
        self.max = [bbox.max.x, bbox.max.y, bbox.max.z];
        self.data0 = data0;
        self.count = count;
    }
}
