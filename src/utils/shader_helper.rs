use pi_render::rhi::{device::RenderDevice, bind_group_layout::BindGroupLayout};

pub const CAMERA_GROUP: usize = 0;
pub const WORLD_MATRIX_GROUP: usize = 1;
pub const DEPTH_GROUP: usize = 2;


pub fn create_depth_layout(device: &RenderDevice) -> BindGroupLayout {
	device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
		label: Some("depth_layout"),
		entries: &[
			wgpu::BindGroupLayoutEntry {
				binding: 0,
				visibility: wgpu::ShaderStages::VERTEX,
				ty: wgpu::BindingType::Buffer {
					ty: wgpu::BufferBindingType::Uniform,
					has_dynamic_offset: false,
					min_binding_size: wgpu::BufferSize::new(4), // f32
				},
				count: None,
			},
		],
	})
}

pub fn create_camera_layout(device: &RenderDevice) -> BindGroupLayout {
	device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
		label: Some("camera_layout"),
		entries: &[
			// project matrix & view matrix
			wgpu::BindGroupLayoutEntry {
				binding: 0,
				visibility: wgpu::ShaderStages::VERTEX,
				ty: wgpu::BindingType::Buffer {
					ty: wgpu::BufferBindingType::Uniform,
					has_dynamic_offset: false,
					min_binding_size: wgpu::BufferSize::new(128), // matrix * 2
				},
				count: None,
			},
		],
	})
}

pub fn create_matrix_group_layout(device: &RenderDevice) -> BindGroupLayout {
	device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
		label: Some("wolrd_matrix_layout"),
		entries: &[
			wgpu::BindGroupLayoutEntry {
				binding: 0,
				visibility: wgpu::ShaderStages::VERTEX,
				ty: wgpu::BindingType::Buffer {
					ty: wgpu::BufferBindingType::Uniform,
					has_dynamic_offset: false,
					min_binding_size: wgpu::BufferSize::new(64),
				},
				count: None,
			},
		],
	})
}