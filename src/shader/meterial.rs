use pi_render::rhi::shader::{
    merge_defines, ArrayLen, AsLayoutEntry, BindingExpandDesc, BindingExpandDescList, BlockCodeAtom, CodeSlice, Define, ShaderMeta, TypeKind,
    TypeSize,
};
use render_derive::{BindLayout, BindingType, BufferSize, Uniform};

#[derive(BindLayout, BufferSize, BindingType)]
#[layout(set(2), binding(3))]
#[min_size(144)]
#[uniformbuffer]
pub struct BackgroundImageBind; // storagebuffer: TODO


#[derive(Uniform)]
#[uniform(offset(0), len(64), bind(BackgroundImageBind))]
pub struct WorldUniform<'a>(pub &'a [f32]);


#[derive(Uniform)]
#[uniform(offset(64), len(16), bind(BackgroundImageBind))]
pub struct RectUniform<'a>(pub &'a [f32]);


#[derive(Uniform)]
#[uniform(offset(80), len(16), bind(BackgroundImageBind))]
pub struct TopRadiuUniform<'a>(pub &'a [f32]);


#[derive(Uniform)]
#[uniform(offset(96), len(16), bind(BackgroundImageBind))]
pub struct BottomRadiuUniform<'a>(pub &'a [f32]);


#[derive(Uniform)]
#[uniform(offset(112), len(16), bind(BackgroundImageBind))]
pub struct BackgroundUvUniform<'a>(pub &'a [f32]);


#[derive(Uniform)]
#[uniform(offset(128), len(12), bind(BackgroundImageBind))]
pub struct Patch23Uniform<'a>(pub &'a [f32]);


#[derive(Uniform)]
#[uniform(offset(140), len(4), bind(BackgroundImageBind))]
pub struct TyUniform<'a>(pub &'a [f32]);

pub fn push_meta(_meta: &mut ShaderMeta, _visibility: wgpu::ShaderStages, _defines: &[Define]) {
    _meta.add_binding_entry(
        2,
        (
            BackgroundImageBind::as_layout_entry(_visibility),
            BindingExpandDescList::new(
                vec![
                    BindingExpandDesc::new_buffer::<f32>(
                        "world",
                        &[0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
                        TypeKind::Float,
                        TypeSize::Mat { rows: 4, columns: 4 },
                        ArrayLen::None,
                    ),
                    BindingExpandDesc::new_buffer::<f32>("rect", &[0.0, 0.0, 0.0, 0.0], TypeKind::Float, TypeSize::Vec(4), ArrayLen::None),
                    BindingExpandDesc::new_buffer::<f32>("top_radius", &[0.0, 0.0, 0.0, 0.0], TypeKind::Float, TypeSize::Vec(4), ArrayLen::None),
                    BindingExpandDesc::new_buffer::<f32>("bottom_radius", &[0.0, 0.0, 0.0, 0.0], TypeKind::Float, TypeSize::Vec(4), ArrayLen::None),
                    BindingExpandDesc::new_buffer::<f32>("background_uv", &[0.0, 0.0, 0.0, 0.0], TypeKind::Float, TypeSize::Vec(4), ArrayLen::None),
                    BindingExpandDesc::new_buffer::<f32>("PATCH_2_3", &[0.0, 0.0, 0.0], TypeKind::Float, TypeSize::Vec(3), ArrayLen::None),
                    BindingExpandDesc::new_buffer::<f32>("ty", &[0.0], TypeKind::Float, TypeSize::Scalar, ArrayLen::None),
                ],
                merge_defines(_defines, &[]),
            ),
        ),
    );
}

pub fn push_code(_codes: &mut BlockCodeAtom, _defines: &[Define]) {}

lazy_static! {
    static ref CODE: Vec<CodeSlice> = vec![];
}
