use glam::{Mat4, Vec3};

pub struct Camera {
    pub eye: Vec3,
    pub target: Vec3,
    pub up: Vec3,
    pub aspect: f32,
    pub fov_y: f32,
    pub near: f32,
    pub far: f32,
}

impl Camera {
    pub fn new(aspect: f32) -> Self {
        Self {
            eye: Vec3::new(5.0, 5.0, 5.0),
            target: Vec3::ZERO,
            up: Vec3::Y,
            aspect,
            fov_y: 45.0_f32.to_radians(),
            near: 0.1,
            far: 1000.0,
        }
    }

    pub fn view_matrix(&self) -> Mat4 {
        Mat4::look_at_rh(self.eye, self.target, self.up)
    }

    pub fn projection_matrix(&self) -> Mat4 {
        Mat4::perspective_rh(self.fov_y, self.aspect, self.near, self.far)
    }

    pub fn view_projection(&self) -> Mat4 {
        self.projection_matrix() * self.view_matrix()
    }
}

pub struct CameraController {
    // Spherical coordinates for orbit
    phi: f32,   // azimuthal angle (around Y)
    theta: f32, // polar angle (from Y axis)
    radius: f32,
    sensitivity: f32,
    pan_sensitivity: f32,
    zoom_sensitivity: f32,
}

impl CameraController {
    pub fn new() -> Self {
        // Default: looking from (5,5,5)
        let radius = (5.0_f32 * 3.0).sqrt(); // ~8.66
        let theta = (5.0_f32 / radius).acos();
        let phi = 5.0_f32.atan2(5.0);

        Self {
            phi,
            theta,
            radius,
            sensitivity: 0.005,
            pan_sensitivity: 0.01,
            zoom_sensitivity: 0.1,
        }
    }

    pub fn orbit(&mut self, camera: &mut Camera, dx: f32, dy: f32) {
        self.phi -= dx * self.sensitivity;
        self.theta = (self.theta - dy * self.sensitivity).clamp(0.01, std::f32::consts::PI - 0.01);
        self.update_camera(camera);
    }

    pub fn pan(&mut self, camera: &mut Camera, dx: f32, dy: f32) {
        let forward = (camera.target - camera.eye).normalize();
        let right = forward.cross(camera.up).normalize();
        let up = right.cross(forward).normalize();

        let offset = right * (-dx * self.pan_sensitivity * self.radius)
            + up * (dy * self.pan_sensitivity * self.radius);
        camera.target += offset;
        self.update_camera(camera);
    }

    pub fn zoom(&mut self, camera: &mut Camera, delta: f32) {
        self.radius = (self.radius - delta * self.zoom_sensitivity * self.radius).clamp(0.5, 500.0);
        self.update_camera(camera);
    }

    fn update_camera(&self, camera: &mut Camera) {
        camera.eye = camera.target
            + Vec3::new(
                self.radius * self.theta.sin() * self.phi.cos(),
                self.radius * self.theta.cos(),
                self.radius * self.theta.sin() * self.phi.sin(),
            );
    }
}

impl Default for CameraController {
    fn default() -> Self {
        Self::new()
    }
}
