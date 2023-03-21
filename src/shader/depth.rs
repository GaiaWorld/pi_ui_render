
use pi_render::rhi::shader::{
    merge_defines, ArrayLen, AsLayoutEntry, BindingExpandDesc, BindingExpandDescList, BlockCodeAtom, CodeSlice, Define, ShaderMeta, TypeKind,
    TypeSize,
};
use render_derive::{BindLayout, BindingType, BufferSize, Uniform};

#[derive(BindLayout, BufferSize, BindingType)]
#[layout(set(1), binding(0))]
#[min_size(16)]
#[uniformbuffer]
pub struct DepthBind; // storagebuffer: TODO


#[derive(Uniform)]
#[uniform(offset(0), len(12), bind(DepthBind))]
pub struct Patch10Uniform<'a>(pub &'a [f32]);


#[derive(Uniform)]
#[uniform(offset(12), len(4), bind(DepthBind))]
pub struct DepthUniform<'a>(pub &'a [f32]);

pub fn push_meta(_meta: &mut ShaderMeta, _visibility: wgpu::ShaderStages, _defines: &[Define]) {
    _meta.add_binding_entry(
        1,
        (
            DepthBind::as_layout_entry(_visibility),
            BindingExpandDescList::new(
                vec![
                    BindingExpandDesc::new_buffer::<f32>("PATCH_1_0", &[0.0, 0.0, 0.0], TypeKind::Float, TypeSize::Vec(3), ArrayLen::None),
                    BindingExpandDesc::new_buffer::<f32>("depth", &[0.0], TypeKind::Float, TypeSize::Scalar, ArrayLen::None),
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
