use raytracer::maths::vec3::Vec3;
use raytracer::primitives::Primitive;
use raytracer::rendering::aabb::Aabb;
use raytracer::rendering::ray::{CanHit, HitRecord, Ray};
use raytracer::rendering::transform::Transform;
use serde::Deserialize;

pub struct Torus {
    pub major_radius: f32,
    pub minor_radius: f32,
}

impl Torus {
    pub fn new(major_radius: f32, minor_radius: f32) -> Self {
        Torus {
            major_radius,
            minor_radius,
        }
    }
}

impl CanHit for Torus {
    fn aabb(&self, transform: &Transform) -> Aabb {
        let c = transform.translation;
        let r = self.major_radius + self.minor_radius;
        Aabb::new(
            Vec3::from_xyz(c.x - r, c.y - self.minor_radius, c.z - r),
            Vec3::from_xyz(c.x + r, c.y + self.minor_radius, c.z + r),
        )
    }

    fn hit(&self, ray: &Ray, transform: &Transform) -> Option<HitRecord> {
        let m = &transform.rot_mat;

        // Transform ray to local space
        let local_origin = {
            let o = ray.position - transform.translation;
            Vec3::from_xyz(
                m[0][0] * o.x + m[1][0] * o.y + m[2][0] * o.z,
                m[0][1] * o.x + m[1][1] * o.y + m[2][1] * o.z,
                m[0][2] * o.x + m[1][2] * o.y + m[2][2] * o.z,
            )
        };

        let local_dir = Vec3::from_xyz(
            m[0][0] * ray.direction.x + m[1][0] * ray.direction.y + m[2][0] * ray.direction.z,
            m[0][1] * ray.direction.x + m[1][1] * ray.direction.y + m[2][1] * ray.direction.z,
            m[0][2] * ray.direction.x + m[1][2] * ray.direction.y + m[2][2] * ray.direction.z,
        );

        let major_rad = self.major_radius;
        let minor_rad = self.minor_radius;

        // Ray equation: P = origin + t * direction
        let ox = local_origin.x;
        let oy = local_origin.y;
        let oz = local_origin.z;
        let dx = local_dir.x;
        let dy = local_dir.y;
        let dz = local_dir.z;

        // Coefficients for the quartic equation
        let dx2 = dx * dx;
        let dy2 = dy * dy;
        let dz2 = dz * dz;
        let ox2 = ox * ox;
        let oy2 = oy * oy;
        let oz2 = oz * oz;
        let major_rad_sq = major_rad * major_rad;
        let minor_rad_sq = minor_rad * minor_rad;

        let a = dx2 + dy2 + dz2;
        let a2 = a * a;

        let b = 2.0 * (ox * dx + oy * dy + oz * dz);
        let b2 = b * b;

        let c = ox2 + oy2 + oz2 - major_rad_sq - minor_rad_sq;

        // Quartic coefficients: At^4 + Bt^3 + Ct^2 + Dt + E = 0
        let coeff_a = a2;
        let coeff_b = 2.0 * a * b;
        let coeff_c = b2 + 2.0 * a * c + 4.0 * major_rad_sq * (dx2 + dz2);
        let coeff_d = 2.0 * (b * c + 4.0 * major_rad_sq * (ox * dx + oz * dz));
        let coeff_e = c * c + 4.0 * major_rad_sq * (ox2 + oz2) - 4.0 * major_rad_sq * minor_rad_sq;

        let mut best_t: Option<f32> = None;

        // Solve quartic equation
        let solutions = solve_quartic(coeff_a, coeff_b, coeff_c, coeff_d, coeff_e);
        for t in solutions {
            if t > 0.001 && t.is_finite() {
                if best_t.is_none() || t < best_t.unwrap() {
                    best_t = Some(t);
                }
            }
        }

        best_t.and_then(|t| {
            let point_local = local_origin + t * local_dir;
            
            // Calculate normal in local space
            let rho_sq = point_local.x * point_local.x + point_local.z * point_local.z;
            let rho = rho_sq.sqrt();
            
            if rho < 1e-6 {
                return None;
            }

            let normal_x = point_local.x * (rho - major_rad) / rho;
            let normal_y = point_local.y;
            let normal_z = point_local.z * (rho - major_rad) / rho;

            let normal_local = Vec3::from_xyz(normal_x, normal_y, normal_z);
            
            // Check if normal is valid
            let normal_len_sq = normal_local.x * normal_local.x + normal_local.y * normal_local.y + normal_local.z * normal_local.z;
            if normal_len_sq < 1e-6 {
                return None;
            }

            let normal_local = normal_local.normalize();

            // Transform normal to world space
            let normal = Vec3::from_xyz(
                m[0][0] * normal_local.x + m[0][1] * normal_local.y + m[0][2] * normal_local.z,
                m[1][0] * normal_local.x + m[1][1] * normal_local.y + m[1][2] * normal_local.z,
                m[2][0] * normal_local.x + m[2][1] * normal_local.y + m[2][2] * normal_local.z,
            );

            let point = ray.position + t * ray.direction;
            Some(HitRecord { t, point, normal })
        })
    }
}

fn solve_quartic(a: f32, b: f32, c: f32, d: f32, e: f32) -> Vec<f32> {
    let mut solutions = Vec::new();

    if a.abs() < 1e-10 {
        // Cubic equation
        solutions.extend(solve_cubic(b, c, d, e));
    } else {
        // Normalize to t^4 + pt^3 + qt^2 + rt + s = 0
        let a_inv = 1.0 / a;
        let p = b * a_inv;
        let q = c * a_inv;
        let r = d * a_inv;
        let s = e * a_inv;

        // Ferrari's method
        let p2 = p * p;
        let _p3 = p2 * p;
        let _p4 = p2 * p2;
        let _q2 = q * q;
        let r2 = r * r;

        // Resolvent cubic: y^3 - q*y^2 + (p*r - 4*s)*y - (p^2*s - 4*q*s + r^2) = 0
        let cubic_b = -q;
        let cubic_c = p * r - 4.0 * s;
        let cubic_d = -(p2 * s - 4.0 * q * s + r2);

        let y_solutions = solve_cubic(1.0, cubic_b, cubic_c, cubic_d);
        
        if let Some(&y) = y_solutions.first() {
            let discriminant = p2 - 4.0 * (q - y);

            if discriminant >= 0.0 {
                let sqrt_disc = discriminant.sqrt();
                let coeff = (p * y - 2.0 * r) / sqrt_disc;

                // First quadratic: t^2 + (p/2 + sqrt(p^2 - 4(q-y))/2)*t + ... = 0
                let a1 = 1.0;
                let b1 = 0.5 * p + 0.5 * sqrt_disc;
                let c1 = 0.5 * (y + coeff);

                let mut quad_sols = solve_quadratic(a1, b1, c1);
                solutions.append(&mut quad_sols);

                // Second quadratic
                let a2 = 1.0;
                let b2 = 0.5 * p - 0.5 * sqrt_disc;
                let c2 = 0.5 * (y - coeff);

                let mut quad_sols = solve_quadratic(a2, b2, c2);
                solutions.append(&mut quad_sols);
            }
        }
    }

    solutions
}

fn solve_cubic(a: f32, b: f32, c: f32, d: f32) -> Vec<f32> {
    let mut solutions = Vec::new();

    if a.abs() < 1e-10 {
        // Quadratic
        solutions.extend(solve_quadratic(b, c, d));
    } else {
        let a_inv = 1.0 / a;
        let p = b * a_inv;
        let q = c * a_inv;
        let r = d * a_inv;

        let p3 = p / 3.0;
        let p3_sq = p3 * p3;
        let p3_cube = p3_sq * p3;

        let a_coeff = q / 3.0 - p3_sq;
        let b_coeff = r / 2.0 + p3_cube - p3 * q / 3.0;

        let discriminant = b_coeff * b_coeff + a_coeff * a_coeff * a_coeff;

        if discriminant >= 0.0 {
            let sqrt_disc = discriminant.sqrt();
            let cbrt_1 = (b_coeff + sqrt_disc).abs().powf(1.0 / 3.0)
                * if b_coeff + sqrt_disc >= 0.0 { 1.0 } else { -1.0 };
            let cbrt_2 = (b_coeff - sqrt_disc).abs().powf(1.0 / 3.0)
                * if b_coeff - sqrt_disc >= 0.0 { 1.0 } else { -1.0 };

            solutions.push(cbrt_1 + cbrt_2 - p3);
        } else {
            let a_sqrt = (-a_coeff).sqrt();
            let angle = (b_coeff / (a_sqrt * a_sqrt * a_sqrt)).acos() / 3.0;
            let const_mult = 2.0 * a_sqrt;

            for k in 0..3 {
                let k_f = k as f32;
                solutions.push(const_mult * (angle + 2.0 * std::f32::consts::PI * k_f / 3.0).cos() - p3);
            }
        }
    }

    solutions
}

fn solve_quadratic(a: f32, b: f32, c: f32) -> Vec<f32> {
    let mut solutions = Vec::new();

    if a.abs() < 1e-10 {
        if b.abs() > 1e-10 {
            solutions.push(-c / b);
        }
    } else {
        let discriminant = b * b - 4.0 * a * c;
        if discriminant >= 0.0 {
            let sqrt_disc = discriminant.sqrt();
            solutions.push((-b - sqrt_disc) / (2.0 * a));
            solutions.push((-b + sqrt_disc) / (2.0 * a));
        }
    }

    solutions
}

#[derive(Deserialize)]
struct TorusConfig {
    major_radius: f32,
    minor_radius: f32,
}

#[unsafe(no_mangle)]
pub fn create(cfg: &ron::Value) -> Primitive {
    let config: TorusConfig = cfg.clone().into_rust().expect("invalid torus config");
    Box::new(Torus::new(config.major_radius, config.minor_radius))
}