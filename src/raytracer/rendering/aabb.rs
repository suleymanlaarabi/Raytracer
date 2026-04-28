use crate::maths::vec3::Vec3;

#[derive(Clone, Copy)]
pub struct Aabb {
    pub min: Vec3,
    pub max: Vec3,
}

impl Default for Aabb {
    #[inline]
    fn default() -> Self {
        Self::EMPTY
    }
}

impl Aabb {
    pub const EMPTY: Self = Self {
        min: Vec3 {
            x: f32::MAX,
            y: f32::MAX,
            z: f32::MAX,
        },
        max: Vec3 {
            x: f32::MIN,
            y: f32::MIN,
            z: f32::MIN,
        },
    };

    #[inline]
    pub fn new(min: Vec3, max: Vec3) -> Self {
        Self { min, max }
    }

    #[inline]
    pub fn merge(a: Self, b: Self) -> Self {
        Self {
            min: Vec3::from_xyz(
                a.min.x.min(b.min.x),
                a.min.y.min(b.min.y),
                a.min.z.min(b.min.z),
            ),
            max: Vec3::from_xyz(
                a.max.x.max(b.max.x),
                a.max.y.max(b.max.y),
                a.max.z.max(b.max.z),
            ),
        }
    }

    #[inline]
    pub fn extend(self, p: Vec3) -> Self {
        Self {
            min: Vec3::from_xyz(
                self.min.x.min(p.x),
                self.min.y.min(p.y),
                self.min.z.min(p.z),
            ),
            max: Vec3::from_xyz(
                self.max.x.max(p.x),
                self.max.y.max(p.y),
                self.max.z.max(p.z),
            ),
        }
    }

    #[inline]
    pub fn centroid(self) -> Vec3 {
        Vec3::from_xyz(
            (self.min.x + self.max.x) * 0.5,
            (self.min.y + self.max.y) * 0.5,
            (self.min.z + self.max.z) * 0.5,
        )
    }

    #[inline(always)]
    pub fn intersect(&self, origin: Vec3, inv_dir: Vec3) -> Option<f32> {
        let t1 = (self.min - origin) * inv_dir;
        let t2 = (self.max - origin) * inv_dir;

        let tmin = Vec3::from_xyz(t1.x.min(t2.x), t1.y.min(t2.y), t1.z.min(t2.z));
        let tmax = Vec3::from_xyz(t1.x.max(t2.x), t1.y.max(t2.y), t1.z.max(t2.z));

        let t_near = tmin.x.max(tmin.y).max(tmin.z);
        let t_far = tmax.x.min(tmax.y).min(tmax.z);

        if t_near <= t_far && t_far > 0.0 {
            Some(t_near.max(0.0))
        } else {
            None
        }
    }
}
