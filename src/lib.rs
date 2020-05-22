/// this macro builds a `wgpu::BindingLayoutDescriptor` with a format not unlike json.
/// ## Syntax
/// ( visibility expression | field ) => { ( bind_location => type )* }
///
/// visibility expressions : Fragment, Vertex, Compute, None. these can be enclosed in brackets and
/// seperated with the binary or operator to use multiple visibility flags. this corresponds to the
/// `visibility` field in the `wgpu::BindGroupLayoutEntry` struct.
///
/// fields: Label. instead of taking a binding label takes a &'static str literal. this field is
/// totally optional and corresponds with the `label` field in the `wgpu::BindGroupLayoutEntry`
/// struct
///
/// textures are described using their equivalent opengl samplers, in the form Tex{Dimension}{MS}?
///
/// type:
///       Buffer: (Dyn)?,
///       StorageBuffer: (Dyn)? + (Readonly)? ,
///       Sampler: (Cmp)?,
///       glsl_sampler_decl<ComponentType>: (Storage(format))? + (Readonly)?
///
/// Usage:
///```
/// # use wgpu_bind_dsl::binding_layout;
///  let desc = binding_layout! {
///     Label => "OptionalName",
///     Vertex => {
///        1 => Buffer,
///        2 => StorageBuffer: Dyn,
///     },
///     Fragment => {
///        2 => Sampler,
///        3 => Sampler: Cmp,
///     },
///     Compute => {
///        1 => Buffer: Dyn,
///        2 => StorageBuffer: Dyn + Readonly,
///     },
///     { Compute | Fragment } => {
///        5 => Tex2D<Sint>: Storage<R8Unorm> + Readonly,
///     },
/// };
///```
///
#[macro_export]
macro_rules! binding_layout {
    // initializer, takes comma seperated list.
    ($($loc:tt => $fmt:tt,)*) => {
        binding_layout!([] ; None; $($loc => $fmt ,)*)
    };
    ([$($t:expr,)*] ; $name:expr ; $vis:tt => {
        $($loc:expr =>
            $binding:tt$(<$generic:ident>)?$(:$($trait1:ident$(<$tgen1:ident>)?)? $(+$trait2:tt$(<$tgen2:ident>)?)?)?,)*
    }, $($ll:tt => $ii:tt,)*) => {
        $crate::binding_layout!(
            [$($t,)* $(
                wgpu::BindGroupLayoutEntry {
                    binding: $loc,
                    visibility: $crate::vis!($vis),
                    ty: $crate::generics!($binding ; $($generic)? ; $($($trait1)?)? ; $($($($tgen1)?)?)? ; $($($trait2)?)? ; $($($($tgen2)?)?)? ),
                },)*] ;
        $name;
        $($ll => $ii,)*
            );
    };
    // special case for naming
    ([$($t:expr,)*] ; $old_name:expr ;  Label => $name:expr, $($ll:tt => $ii:tt,)*) => {
        binding_layout!([$($t,)*] ; Some($name) ; $($ll => $ii,)*)
    };
    // base case: ([binding list] ; name ;])
    ([$($t:expr,)*] ; $name:expr ; ) => { wgpu::BindGroupLayoutDescriptor { label: $name, bindings: &[$($t,)*] } };
}

#[macro_export]
macro_rules! only_traits {
    (Buffer ; ; ) => {
        wgpu::BindingType::UniformBuffer { dynamic: false }
    };
    (Buffer ; Dyn ; ) => {
        wgpu::BindingType::UniformBuffer { dynamic: true }
    };
    (Sampler ; ; ) => {
        wgpu::BindingType::Sampler { comparison: false }
    };
    (Sampler ; Cmp ; ) => {
        wgpu::BindingType::Sampler { comparison: true }
    };
    (StorageBuffer ; ; ) => {
        wgpu::BindingType::StorageBuffer {
            dynamic: false,
            readonly: false,
        }
    };
    (StorageBuffer ; Dyn ; ) => {
        wgpu::BindingType::StorageBuffer {
            dynamic: true,
            readonly: false,
        }
    };
    (StorageBuffer ; Readonly ; ) => {
        wgpu::BindingType::StorageBuffer {
            dynamic: false,
            Readonly: true,
        }
    };
    (StorageBuffer ; Readonly ; Dyn) => {
        wgpu::BindingType::StorageBuffer {
            dynamic: true,
            readonly: true,
        }
    };

    (StorageBuffer ; Dyn ; Readonly) => {
        wgpu::BindingType::StorageBuffer {
            dynamic: true,
            readonly: true,
        }
    };
}

/// internal use only
#[macro_export]
macro_rules! generics {
    ( $texType:ident ; ; $($trait1:ident)? ; ; $($trait2:ident)? ; ) => {
        $crate::only_traits!($texType ; $($trait1)? ; $($trait2)? )
    };
    ( $texType:ident ; $fmt:ident ; ; ; ; ) => {
        wgpu::BindingType::SampledTexture {
            dimension: $crate::d!($texType).0,
            multisampled: $crate::d!($texType).1,
            component_type: wgpu::TextureComponentType::$fmt,
        }
    };
    ( $texType:ident ; $fmt:ident ; Storage ; $strgfmt:ident ; ; ) => {
        wgpu::BindingType::StorageTexture {
            dimension: $crate::d!($texType).0,
            component_type: wgpu::TextureComponentType::$fmt,
            format: wgpu::TextureFormat::$strgfmt,
            readonly: false,
        }
    };
    ( $texType:ident ; $fmt:ident ; Storage ; $strgfmt:ident ; Readonly ; /** */ ) => {
        wgpu::BindingType::StorageTexture {
            dimension: $crate::d!($texType).0,
            component_type: wgpu::TextureComponentType::$fmt,
            format: wgpu::TextureFormat::$strgfmt,
            readonly: true,
        }
    };
    ( $texType:ident ; $fmt:ident ; Readonly ; /** None */ ; Storage ; $strgfmt:ident) => {
        wgpu::BindingType::StorageTexture {
            dimension: $crate::d!($texType).0,
            component_type: wgpu::TextureComponentType::$fmt,
            format: wgpu::TextureFormat::$strgfmt,
            readonly: true,
        }
    };
}

#[macro_export]
macro_rules! d {
    (Tex1D) => {
        (wgpu::TextureViewDimension::D1, false)
    };
    (Tex1DMS) => {
        (wgpu::TextureViewDimension::D1, true)
    };
    (Tex2D) => {
        (wgpu::TextureViewDimension::D2, false)
    };
    (Tex2DMS) => {
        (wgpu::TextureViewDimension::D2, true)
    };
    (Tex3D) => {
        (wgpu::TextureViewDimension::D3, false)
    };
    (Tex3DMS) => {
        (wgpu::TextureViewDimension::D3, true)
    };
    (Tex2DArray) => {
        (wgpu::TextureViewDimension::D2Array, false)
    };
    (Tex2DArrayMs) => {
        (wgpu::TextureViewDimension::D2Array, true)
    };
    (TexCube) => {
        (wgpu::TextureViewDimension::Cube, false)
    };
    (TexCubeMS) => {
        (wgpu::TextureViewDimension::CubeArray, false)
    };
    (TexCubeArrayMs) => {
        (wgpu::TextureViewDimension::CubeArray, true)
    };
}

// for internal use only but this one might actually be usefull
#[macro_export]
macro_rules! vis {
    ( Vertex ) => {
        wgpu::ShaderStage::VERTEX
    };
    ( Fragment ) => {
        wgpu::ShaderStage::FRAGMENT
    };
    ( Compute ) => {
        wgpu::ShaderStage::COMPUTE
    };
    ( None ) => {
        wgpu::ShaderStage::NONE
    };
    ( { $id:ident | $rest:ident }) => {
        $crate::vis!($id) | $crate::vis!($rest)
    };
    ( { $id:ident | $rest:ident | $more:ident }) => {
        $crate::vis!($id) | $crate::vis!($rest) | $crate::vis!($more)
    };
}

#[cfg(test)]
mod test {
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
}
