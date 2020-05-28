/// builds a `wgpu::BindingLayoutDescriptor` with a format not unlike json.
/// ## Syntax
///
///  the syntax is complex enough that explaining it with a BNF is harder than just
///  learning to read it. so here is a series of examples that show in increasingly complex ways
///  how to use the macro.
///
///  here is the BNF anyways.
///  ```compile_fail
///  <valid_input> ::= <visibility_expr> "=>" <binding_expr>","
///                  | <valid_input> <valid_input>
///                  | <empty>
///
///  <visibility_expr> ::=  "Vertex"
///                       | "Fragment"
///                       | "Compute"
///                       | "None"
///                       | "Label" | "{" <visibility_expr> "|" <visibility_expr> "}"
///
///  <binding_expr> ::= | <digit> "=>" <binding>","
///                     | "{" <binding_expr> <binding_expr>  "}"
///                     | "{" <empty> "}"
///
///  <binding> ::= <buffer> | <texture> | <sampler>
///
///  // Dyn here sets the buffer to a dynamic buffer. note that order on the traits does not matter.
///  <buffer> ::= "Buffer"
///             | "Buffer: Dyn"
///             | "StorageBuffer"
///             | "StorageBuffer: Dyn"
///             | "StorageBuffer: Dyn + Readonly"
///
///  // Cmp here sets the sampler to a comparison sampler.
///  <sampler> ::= "Sampler" | "Sampler: Cmp,"
///
///  <texture> ::= <tex_type>"<"<component_type>">"<tex_traits>
///
///  // any texture type suffixed with MS is multisampled, this does nothing for storage textures.
///  <tex_type> ::= "Tex1D" | "Tex1DMS"
///               | "Tex2D" | "Tex2DMS"
///               | "Tex3D" | "Tex3DMS"
///               | "Tex2DArray"| "Tex2DArrayMS"
///               |  "TexCubeArray" | "TexCubeArrayMS"
///               | "TexCube" | "TexCubeMS" |
///
///  <component_type> ::= "Float" | "Sint" | "Uint"
///
///  <tex_traits> ::= <empty>
///               | "Storage<" <storage_type> ">"
///               | "Readonly"
///               | <tex_traits> "+" <tex_traits>
///
/// <storage_type> ::= // Any variant from `wgpu::TextureFormat`
///
///  ```
///
/// ## An empty bind group
/// an empty bind group with no label can be made by default.
/// ```
/// # use wgpu_bind_dsl::binding_layout;
/// # use wgpu;
///
/// let desc = binding_layout!{};
/// let equivalent =  wgpu::BindGroupLayoutDescriptor { label: None, bindings: &[]};
///
/// assert_eq!(desc.bindings.len(), 0);
/// assert_eq!(desc.label, equivalent.label);
/// ```
///
/// ## Single buffer and a label
/// ```
/// # use wgpu_bind_dsl::binding_layout;
///
/// let desc = binding_layout! {
///              Label => "named",
///             { Compute | Fragment } => { 1 => Buffer, },
///        };
/// ```
///
/// ## Big Usage:
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
/// F.A.Q:
/// Q: why isn't this a proc macro?
/// A: writing this as a macro by example was actually easier given the current tooling hurt of
/// rolling a proc macro. this is less hygenic but I don't like adding dev dependencies to
/// projects. turning this into a proc macro in the future is a definite possibility.
///
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

/// internal use only
/// converts a type expression into its `wgpu::BindingType` equivalent. this only works for the
/// subset of types that do not use the pseudo generics notation. which is to say Buffers,
/// StorageBuffers, and Samplers.
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
/// converts a Type expression into its `wgpu::BindingType` equivalent
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

/// internal use only
/// converts a texture expression to a tuple of its dimension and a bool indicating whether or not
/// it is multisampled.
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
    (Tex2DArrayMS) => {
        (wgpu::TextureViewDimension::D2Array, true)
    };
    (TexCube) => {
        (wgpu::TextureViewDimension::Cube, false)
    };
    (TexCubeMS) => {
        (wgpu::TextureViewDimension::CubeArray, false)
    };
    (TexCubeArrayMS) => {
        (wgpu::TextureViewDimension::CubeArray, true)
    };
}

/// for internal use only.
/// converts a visibility expression into its equivalent `wgpu::ShaderStage` bitflag
/// representation. only works on expression of up to three items.
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
}
