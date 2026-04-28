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

    #[inline]
    pub fn intersect(&self, origin: Vec3, inv_dir: Vec3) -> Option<f32> {
        let mut tmin = (self.min.x - origin.x) * inv_dir.x;
        let mut tmax = (self.max.x - origin.x) * inv_dir.x;

        if inv_dir.x < 0.0 {
            std::mem::swap(&mut tmin, &mut tmax);
        }

        let mut tmin_y = (self.min.y - origin.y) * inv_dir.y;
        let mut tmax_y = (self.max.y - origin.y) * inv_dir.y;

        if inv_dir.y < 0.0 {
            std::mem::swap(&mut tmin_y, &mut tmax_y);
        }

        if tmin > tmax_y || tmin_y > tmax {
            return None;
        }

        tmin = tmin.max(tmin_y);
        tmax = tmax.min(tmax_y);

        let mut tmin_z = (self.min.z - origin.z) * inv_dir.z;
        let mut tmax_z = (self.max.z - origin.z) * inv_dir.z;

        if inv_dir.z < 0.0 {
            std::mem::swap(&mut tmin_z, &mut tmax_z);
        }

        if tmin > tmax_z || tmin_z > tmax {
            return None;
        }

        tmin = tmin.max(tmin_z);
        tmax = tmax.min(tmax_z);

        if tmax >= tmin && tmax > 0.0 {
            Some(tmin.max(0.0))
        } else {
            None
        }
    }
}
