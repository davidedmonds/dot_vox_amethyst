use amethyst::assets::AssetStorage;
use amethyst::core::GlobalTransform;
use amethyst::core::cgmath::{Matrix4, SquareMatrix, Transform};
use amethyst::ecs::{Join, Read, ReadExpect, ReadStorage};
use amethyst::renderer::{ActiveCamera, Attributes, Camera, Color, Effect, Encoder, Factory,
                         Material, MaterialDefaults, Mesh, MeshHandle, NewEffect, Position, Query,
                         Visibility};
use amethyst::renderer::error::Result;
use amethyst::renderer::pipe::DepthMode;
use amethyst::renderer::pipe::pass::{Pass, PassData};

use glsl_layout::*;

use std::marker::PhantomData;

static VERT_SRC: &[u8] = include_bytes!("shaders/vertex/basic.glsl");
static FRAG_SRC: &[u8] = include_bytes!("shaders/fragment/flat.glsl");

#[repr(C, align(16))]
#[derive(Clone, Copy, Debug, Uniform)]
pub(crate) struct VertexArgs {
    proj: mat4,
    view: mat4,
    model: mat4,
}

/// Draw mesh without lighting
/// `V` is `VertexFormat`
#[derive(Derivative, Clone, Debug, PartialEq)]
#[derivative(Default(bound = "V: Query<(Position, Color)>, Self: Pass"))]
pub struct DrawVoxels<V> {
    _pd: PhantomData<V>,
}

impl<V> DrawVoxels<V>
    where
        V: Query<(Position, Color)>,
        Self: Pass,
{
    /// Create instance of `DrawVoxels` pass
    pub fn new() -> Self {
        Default::default()
    }
}

impl<'a, V> PassData<'a> for DrawVoxels<V>
    where
        V: Query<(Position, Color)>,
{
    type Data = (
        Option<Read<'a, ActiveCamera>>,
        ReadStorage<'a, Camera>,
        Read<'a, AssetStorage<Mesh>>,
        ReadExpect<'a, MaterialDefaults>,
        Option<Read<'a, Visibility>>,
        ReadStorage<'a, MeshHandle>,
        ReadStorage<'a, Material>,
        ReadStorage<'a, GlobalTransform>,
    );
}

impl<V> Pass for DrawVoxels<V>
    where
        V: Query<(Position, Color)>,
{
    fn compile(&mut self, effect: NewEffect) -> Result<Effect> {
        use std::mem;
        let mut builder = effect.simple(VERT_SRC, FRAG_SRC);
        builder
            .with_raw_constant_buffer(
                "VertexArgs",
                mem::size_of::<<VertexArgs as Uniform>::Std140>(),
                1,
            )
            .with_raw_vertex_buffer(V::QUERIED_ATTRIBUTES, V::size() as u8, 0);
        builder.with_output("color", Some(DepthMode::LessEqualWrite));
        builder.build()
    }

    fn apply<'a, 'b: 'a>(
        &'a mut self,
        encoder: &mut Encoder,
        effect: &mut Effect,
        _factory: Factory,
        (
            active,
            camera,
            mesh_storage,
            material_defaults,
            visibility,
            mesh,
            material,
            global,
        ): <Self as PassData<'a>>::Data,
    ) {
        let camera = get_camera(active, &camera, &global);

        match visibility {
            None => for (mesh, material, global) in (&mesh, &material, &global).join() {
                draw_mesh(
                    encoder,
                    effect,
                    mesh_storage.get(mesh),
                    Some(material),
                    &material_defaults,
                    camera,
                    Some(global),
                    &[V::QUERIED_ATTRIBUTES],
                );
            },
            Some(ref visibility) => {
                for (mesh, material, global, _) in
                    (&mesh, &material, &global, &visibility.visible_unordered).join()
                    {
                        draw_mesh(
                            encoder,
                            effect,
                            mesh_storage.get(mesh),
                            Some(material),
                            &material_defaults,
                            camera,
                            Some(global),
                            &[V::QUERIED_ATTRIBUTES]
                        );
                    }

                for entity in &visibility.visible_ordered {
                    if let Some(mesh) = mesh.get(*entity) {
                        draw_mesh(
                            encoder,
                            effect,
                            mesh_storage.get(mesh),
                            material.get(*entity),
                            &material_defaults,
                            camera,
                            global.get(*entity),
                            &[V::QUERIED_ATTRIBUTES]
                        );
                    }
                }
            }
        }
    }
}

pub(crate) fn draw_mesh(
    encoder: &mut Encoder,
    effect: &mut Effect,
    mesh: Option<&Mesh>,
    material: Option<&Material>,
    material_defaults: &MaterialDefaults,
    camera: Option<(&Camera, &GlobalTransform)>,
    global: Option<&GlobalTransform>,
    attributes: &[Attributes<'static>],
) {
    let mesh = match mesh {
        Some(mesh) => mesh,
        None => return,
    };
    if material.is_none() || global.is_none() {
        return;
    }

    if !set_attribute_buffers(effect, mesh, attributes) {
        effect.clear();
        return;
    }

    set_vertex_args(effect, encoder, camera, global.unwrap());

    effect.draw(mesh.slice(), encoder);
    effect.clear();
}

pub(crate) fn get_camera<'a>(
    active: Option<Read<'a, ActiveCamera>>,
    camera: &'a ReadStorage<Camera>,
    global: &'a ReadStorage<GlobalTransform>,
) -> Option<(&'a Camera, &'a GlobalTransform)> {
    active
        .and_then(|a| {
            let cam = camera.get(a.entity);
            let transform = global.get(a.entity);
            cam.into_iter().zip(transform.into_iter()).next()
        })
        .or_else(|| (camera, global).join().next())
}

pub(crate) fn set_vertex_args(
    effect: &mut Effect,
    encoder: &mut Encoder,
    camera: Option<(&Camera, &GlobalTransform)>,
    global: &GlobalTransform,
) {
    let vertex_args = camera
        .as_ref()
        .map(|&(ref cam, ref transform)| VertexArgs {
            proj: cam.proj.into(),
            view: transform.0.invert().unwrap().into(),
            model: global.0.into(),
        })
        .unwrap_or_else(|| VertexArgs {
            proj: Matrix4::one().into(),
            view: Matrix4::one().into(),
            model: global.0.into(),
        });
    effect.update_constant_buffer("VertexArgs", &vertex_args.std140(), encoder);
}

pub(crate) fn set_attribute_buffers(
    effect: &mut Effect,
    mesh: &Mesh,
    attributes: &[Attributes<'static>],
) -> bool {
    for attr in attributes.iter() {
        match mesh.buffer(attr) {
            Some(vbuf) => effect.data.vertex_bufs.push(vbuf.clone()),
            None => {
                error!(
                    "Required vertex attribute buffer with format {:?} missing in mesh",
                    attr
                );
                return false;
            }
        }
    }
    true
}
