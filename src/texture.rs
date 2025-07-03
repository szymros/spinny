use winit::dpi::PhysicalSize;


pub struct Texture {
    pub texture: wgpu::Texture,
    pub texture_view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
}

impl Texture {
    pub fn create_depth_texture(
        device: &wgpu::Device,
        size: PhysicalSize<u32>,
        array_layers: u32,
    ) -> Texture {
        let depth_texture_size = wgpu::Extent3d {
            width: size.width.max(1),
            height: size.height.max(1),
            depth_or_array_layers: array_layers,
        };
        let depth_texture_descriptor = wgpu::TextureDescriptor {
            label: None,
            size: depth_texture_size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Depth32Float,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        };
        let depth_texture = device.create_texture(&depth_texture_descriptor);
        let depth_texture_view = depth_texture.create_view(&wgpu::TextureViewDescriptor {
            ..Default::default()
        });
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: None,
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Nearest,
            lod_min_clamp: 0.0,
            lod_max_clamp: 100.0,
            compare: Some(wgpu::CompareFunction::LessEqual),
            ..Default::default()
        });
        return Texture {
            texture: depth_texture,
            texture_view: depth_texture_view,
            sampler,
        };
    }

    pub fn create_cube_depth_texture(device: &wgpu::Device, size: PhysicalSize<u32>) -> Texture{
        let mut cube_depth_texture = Texture::create_depth_texture(device, size, 6);
        cube_depth_texture.texture_view =
            cube_depth_texture
                .texture
                .create_view(&wgpu::TextureViewDescriptor {
                    format: Some(cube_depth_texture.texture.format()),
                    dimension: Some(wgpu::TextureViewDimension::Cube),
                    aspect: wgpu::TextureAspect::DepthOnly,
                    base_array_layer: 0,
                    array_layer_count: Some(6),
                    ..Default::default()
                });
        return cube_depth_texture;
    }

    pub fn load_texture(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        image_rgba: &image::RgbaImage,
    ) -> Texture {
        let dimensions = image_rgba.dimensions();

        let texture_size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1,
        };

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: None,
            size: texture_size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });
        queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &image_rgba,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(4 * dimensions.0),
                rows_per_image: Some(dimensions.1),
            },
            texture_size,
        );
        let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: None,
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });
        return Texture {
            texture,
            texture_view,
            sampler,
        };
    }

    pub fn create_solid_color_texture(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        rgba: [u8; 4],
    ) -> Texture {
        let image_rgba = image::RgbaImage::from_fn(8, 8, |_, _| image::Rgba(rgba));
        return Texture::load_texture(device, queue, &image_rgba);
    }
}

