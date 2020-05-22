# wgpu_bind_dsl

a macro for working with the wgpu-rs library. allows for declarative binding descriptors!

previously you would have to describe a (deep breath) `wgpu::BindLayoutDescriptor` as such

```Rust
 &wgpu::BindGroupLayoutDescriptor {
          bindings: &[
              wgpu::BindGroupLayoutEntry {
                  binding: 0, // global
                  visibility: wgpu::ShaderStage::VERTEX | wgpu::ShaderStage::FRAGMENT,
                  ty: wgpu::BindingType::UniformBuffer { dynamic: false },
              },
              wgpu::BindGroupLayoutEntry {
                  binding: 1, // lights
                  visibility: wgpu::ShaderStage::VERTEX | wgpu::ShaderStage::FRAGMENT,
                  ty: wgpu::BindingType::UniformBuffer { dynamic: false },
              },
              wgpu::BindGroupLayoutEntry {
                  binding: 2,
                  visibility: wgpu::ShaderStage::FRAGMENT,
                  ty: wgpu::BindingType::SampledTexture {
                      multisampled: false,
                      component_type: wgpu::TextureComponentType::Float,
                      dimension: wgpu::TextureViewDimension::D2Array,
                  },
              },
              wgpu::BindGroupLayoutEntry {
                  binding: 3,
                  visibility: wgpu::ShaderStage::FRAGMENT,
                  ty: wgpu::BindingType::Sampler { comparison: true },
              },
          ],
          label: None,
      }
```
but now you can simply write

```Rust
binding_layout! {
  { Vertex |  Fragment } => {
      0 => Buffer,
      1 => Buffer,
  },
  Fragment => {
     2 => Tex2DArray<Float>,
     3 => Sampler: Cmp,
  },
};
```

