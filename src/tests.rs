use wgpu;
#[test]
fn buffers_and_names() {
    let a = binding_layout! {
        Label => "named",
        { Compute | Fragment } => { 1 => Buffer, },
    };

    assert_eq!(a.label, Some("named"));
    assert_eq!(a.bindings[0].binding, 1);
    assert_eq!(
        a.bindings[0].ty,
        wgpu::BindingType::UniformBuffer { dynamic: false }
    );

    assert_eq!(
        a.bindings[0].visibility,
        wgpu::ShaderStage::FRAGMENT | wgpu::ShaderStage::COMPUTE
    );
}

#[test]
fn buffer_types() {
    let a = binding_layout! {
        Label => "named",
        { Compute | Fragment } => {
                    1 => Buffer: Dyn,
          },
    };

    assert_eq!(a.label, Some("named"));
    assert_eq!(a.bindings[0].binding, 1);
    assert_eq!(
        a.bindings[0].ty,
        wgpu::BindingType::UniformBuffer { dynamic: true }
    );

    assert_eq!(
        a.bindings[0].visibility,
        wgpu::ShaderStage::FRAGMENT | wgpu::ShaderStage::COMPUTE
    );
}

#[test]
fn templates() {
    let a = binding_layout! {
        Label => "named",
        { Compute | Fragment } => {
                2 => Tex1D<Float>: Storage<R8Unorm> + Readonly,
                4 => Buffer,
        },
    };

    assert_eq!(a.label, Some("named"));
    assert_eq!(a.bindings[0].binding, 2);
    assert_eq!(
        a.bindings[0].ty,
        wgpu::BindingType::StorageTexture {
            dimension: wgpu::TextureViewDimension::D1,
            component_type: wgpu::TextureComponentType::Float,
            readonly: true,
            format: wgpu::TextureFormat::R8Unorm,
        }
    );

    assert_eq!(
        a.bindings[0].visibility,
        wgpu::ShaderStage::FRAGMENT | wgpu::ShaderStage::COMPUTE
    );
}
#[test]
fn long() {
    let a = binding_layout! {
        { Vertex | Fragment } => {
                                     0 => Tex1D<Float>: Storage<R8Unorm> + Readonly,
                                     1 => Tex2D<Sint>: Readonly + Storage<Rgba32Uint>,
                                     2 => StorageBuffer: Dyn,
                                     3 => StorageBuffer,
                                     4 => Buffer: Dyn,
                                     5 => Buffer,
                                 },
    };

    let b = &[
        wgpu::BindGroupLayoutEntry {
            ty: wgpu::BindingType::StorageTexture {
                dimension: wgpu::TextureViewDimension::D1,
                component_type: wgpu::TextureComponentType::Float,
                readonly: true,
                format: wgpu::TextureFormat::R8Unorm,
            },
            binding: 0,
            visibility: wgpu::ShaderStage::FRAGMENT | wgpu::ShaderStage::VERTEX,
        },
        wgpu::BindGroupLayoutEntry {
            ty: wgpu::BindingType::StorageTexture {
                dimension: wgpu::TextureViewDimension::D2,
                component_type: wgpu::TextureComponentType::Sint,
                readonly: true,
                format: wgpu::TextureFormat::Rgba32Uint,
            },
            binding: 1,
            visibility: wgpu::ShaderStage::FRAGMENT | wgpu::ShaderStage::VERTEX,
        },
        wgpu::BindGroupLayoutEntry {
            ty: wgpu::BindingType::StorageBuffer {
                dynamic: true,
                readonly: false,
            },
            binding: 2,
            visibility: wgpu::ShaderStage::FRAGMENT | wgpu::ShaderStage::VERTEX,
        },
        wgpu::BindGroupLayoutEntry {
            ty: wgpu::BindingType::StorageBuffer {
                dynamic: false,
                readonly: false,
            },
            binding: 3,
            visibility: wgpu::ShaderStage::FRAGMENT | wgpu::ShaderStage::VERTEX,
        },
        wgpu::BindGroupLayoutEntry {
            ty: wgpu::BindingType::UniformBuffer { dynamic: true },
            binding: 4,
            visibility: wgpu::ShaderStage::FRAGMENT | wgpu::ShaderStage::VERTEX,
        },
        wgpu::BindGroupLayoutEntry {
            ty: wgpu::BindingType::UniformBuffer { dynamic: false },
            binding: 5,
            visibility: wgpu::ShaderStage::FRAGMENT | wgpu::ShaderStage::VERTEX,
        },
    ];

    assert_eq!(a.label, None);

    a.bindings.iter().zip(b.iter()).for_each(|(a, b)| {
        assert_eq!(a.ty, b.ty);
        assert_eq!(a.binding, b.binding);
        assert_eq!(a.visibility, b.visibility);
    });
}
