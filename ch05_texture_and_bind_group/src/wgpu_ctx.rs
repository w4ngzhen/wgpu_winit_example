use crate::img_utils::RgbaImg;
use crate::vertex::{create_vertex_buffer_layout, VERTEX_INDEX_LIST, VERTEX_LIST};
use std::borrow::Cow;
use std::sync::Arc;
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use wgpu::MemoryHints::Performance;
use wgpu::{SamplerDescriptor, ShaderSource};
use winit::window::Window;

pub struct WgpuCtx<'window> {
    surface: wgpu::Surface<'window>,
    surface_config: wgpu::SurfaceConfiguration,
    adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    vertex_index_buffer: wgpu::Buffer,
    texture: wgpu::Texture,
    texture_image: RgbaImg,
    texture_size: wgpu::Extent3d,
    sampler: wgpu::Sampler,
}

impl<'window> WgpuCtx<'window> {
    pub async fn new_async(window: Arc<Window>) -> WgpuCtx<'window> {
        let instance = wgpu::Instance::default();
        let surface = instance.create_surface(Arc::clone(&window)).unwrap();
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                force_fallback_adapter: false,
                // Request an adapter which can render to our surface
                compatible_surface: Some(&surface),
            })
            .await
            .expect("Failed to find an appropriate adapter");
        // Create the logical device and command queue
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    required_features: wgpu::Features::empty(),
                    // Make sure we use the texture resolution limits from the adapter, so we can support images the size of the swapchain.
                    required_limits: wgpu::Limits::downlevel_webgl2_defaults()
                        .using_resolution(adapter.limits()),
                    memory_hints: Performance,
                },
                None,
            )
            .await
            .expect("Failed to create device");

        // 获取窗口内部物理像素尺寸（没有标题栏）
        let mut size = window.inner_size();
        // 至少（w = 1, h = 1），否则Wgpu会panic
        let width = size.width.max(1);
        let height = size.height.max(1);
        // 获取一个默认配置
        let surface_config = surface.get_default_config(&adapter, width, height).unwrap();
        // 完成首次配置
        surface.configure(&device, &surface_config);

        let render_pipeline = create_pipeline(&device, surface_config.format);

        let bytes: &[u8] = bytemuck::cast_slice(&VERTEX_LIST);
        let vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: bytes,
            usage: wgpu::BufferUsages::VERTEX,
        });
        // 将顶点索引数据转为字节数据
        let vertex_index_bytes = bytemuck::cast_slice(&VERTEX_INDEX_LIST);
        // 创建顶点索引缓冲数据
        let vertex_index_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: vertex_index_bytes,
            usage: wgpu::BufferUsages::INDEX, // 注意，usage字段使用INDEX枚举，表明是顶点索引
        });

        // 构造图片对象（这里为了代码简洁，我们假设图片加载没有问题，直接unwrap，请读者务必保证图片加载正确性）
        let img = RgbaImg::new("/Users/w4ngzhen/projects/rust-projects/wgpu_winit_example/ch05_texture_and_bind_group/assets/example-img.png").unwrap();
        // 纹理是以3D形式存储，如果想要表示2D纹理，只需要将下方的深度字段设置为1
        let texture_size = wgpu::Extent3d {
            width: img.width, // 图片的宽高
            height: img.height,
            depth_or_array_layers: 1, // <-- 设置为1表示2D纹理
        };
        // 构造Texture实例
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: None,
            // size字段用于表达纹理的基本尺寸结构（宽、高以及深度）
            size: texture_size,
            mip_level_count: 1, // 后面会详细介绍此字段
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            // 大多数图像都是使用 sRGB 来存储的，我们需要在这里指定。
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            // TEXTURE_BINDING 表示我们要在着色器中使用这个纹理。
            // COPY_DST 表示我们能将数据复制到这个纹理上。
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        // 创建采样器
        let sampler = device.create_sampler(&SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            ..Default::default()
        });

        WgpuCtx {
            surface,
            surface_config,
            adapter,
            device,
            queue,
            render_pipeline,
            vertex_buffer,
            vertex_index_buffer,
            texture,
            texture_image: img,
            texture_size,
            sampler,
        }
    }

    pub fn new(window: Arc<Window>) -> WgpuCtx<'window> {
        pollster::block_on(WgpuCtx::new_async(window))
    }

    pub fn resize(&mut self, new_size: (u32, u32)) {
        let (width, height) = new_size;
        self.surface_config.width = width.max(1);
        self.surface_config.height = height.max(1);
        self.surface.configure(&self.device, &self.surface_config);
    }

    pub fn draw(&mut self) {
        let surface_texture = self
            .surface
            .get_current_texture()
            .expect("Failed to acquire next swap chain texture");
        let texture_view = surface_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &texture_view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::GREEN),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
            rpass.set_pipeline(&self.render_pipeline);
            // 消费存放的 vertex_buffer
            rpass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            // 消费存放的 vertex_index_buffer
            rpass.set_index_buffer(
                self.vertex_index_buffer.slice(..),
                wgpu::IndexFormat::Uint16,
            ); // 1.
               // 调用draw_indexed，传入对应数量的顶点数量
            rpass.draw_indexed(0..VERTEX_INDEX_LIST.len() as u32, 0, 0..1);
            // 顶点有原来的固定3个顶点，调整为根据 VERTEX_LIST 动态来计算
            rpass.draw(0..VERTEX_LIST.len() as u32, 0..1);
        }
        self.queue.write_texture(
            // 告诉 wgpu 将像素数据复制到何处
            wgpu::ImageCopyTexture {
                texture: &self.texture, // <-- 纹理对象
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &self.texture_image.bytes, // <-- 像素rgba二进制数据
            // 纹理的内存布局
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4 * self.texture_image.width),
                rows_per_image: Some(self.texture_image.height),
            },
            self.texture_size, // <-- Extend3d对象
        );
        self.queue.submit(Some(encoder.finish()));
        surface_texture.present();
    }
}

fn create_pipeline(
    device: &wgpu::Device,
    swap_chain_format: wgpu::TextureFormat,
) -> wgpu::RenderPipeline {
    // Load the shaders from disk
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: None,
        source: ShaderSource::Wgsl(Cow::Borrowed(include_str!("shader.wgsl"))),
    });
    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: None,
        layout: None,
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: Some("vs_main"),
            buffers: &[create_vertex_buffer_layout()],
            compilation_options: Default::default(),
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: Some("fs_main"),
            compilation_options: Default::default(),
            targets: &[Some(swap_chain_format.into())],
        }),
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            ..Default::default()
        },
        depth_stencil: None,
        multisample: wgpu::MultisampleState::default(),
        multiview: None,
        cache: None,
    })
}
