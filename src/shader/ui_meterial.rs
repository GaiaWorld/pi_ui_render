
use pi_render::rhi::shader::{
    merge_defines, ArrayLen, AsLayoutEntry, BindingExpandDesc, BindingExpandDescList, BlockCodeAtom, CodeSlice, Define, ShaderMeta, TypeKind,
    TypeSize,
};
use render_derive::{BindLayout, BindingType, BufferSize, Uniform};

#[derive(BindLayout, BufferSize, BindingType)]
#[layout(set(2), binding(0))]
#[min_size(240)]
#[uniformbuffer]
pub struct UiMaterialBind; // storagebuffer: TODO


#[derive(Uniform)]
#[uniform(offset(0), len(64), bind(UiMaterialBind))]
pub struct WorldUniform<'a>(pub &'a [f32]);


#[derive(Uniform)]
#[uniform(offset(64), len(64), bind(UiMaterialBind))]
pub struct ClipSdfOrSdflineUniform<'a>(pub &'a [f32]);


#[derive(Uniform)]
#[uniform(offset(128), len(16), bind(UiMaterialBind))]
pub struct ColorUniform<'a>(pub &'a [f32]);


#[derive(Uniform)]
#[uniform(offset(144), len(16), bind(UiMaterialBind))]
pub struct StrokeColorOrURectUniform<'a>(pub &'a [f32]);


#[derive(Uniform)]
#[uniform(offset(160), len(16), bind(UiMaterialBind))]
pub struct UWeightAndOffsetUniform<'a>(pub &'a [f32]);


#[derive(Uniform)]
#[uniform(offset(176), len(16), bind(UiMaterialBind))]
pub struct UGradientStarteEndUniform<'a>(pub &'a [f32]);


#[derive(Uniform)]
#[uniform(offset(192), len(8), bind(UiMaterialBind))]
pub struct TextureSizeOrBottomLeftBorderUniform<'a>(pub &'a [f32]);


#[derive(Uniform)]
#[uniform(offset(200), len(8), bind(UiMaterialBind))]
pub struct DataTexSizeUniform<'a>(pub &'a [f32]);


#[derive(Uniform)]
#[uniform(offset(208), len(8), bind(UiMaterialBind))]
pub struct SlopeUniform<'a>(pub &'a [f32]);


#[derive(Uniform)]
#[uniform(offset(216), len(8), bind(UiMaterialBind))]
pub struct ScaleUniform<'a>(pub &'a [f32]);


#[derive(Uniform)]
#[uniform(offset(224), len(12), bind(UiMaterialBind))]
pub struct Patch20Uniform<'a>(pub &'a [f32]);


#[derive(Uniform)]
#[uniform(offset(236), len(4), bind(UiMaterialBind))]
pub struct BlurUniform<'a>(pub &'a [f32]);

pub fn push_meta(_meta: &mut ShaderMeta, _visibility: wgpu::ShaderStages, _defines: &[Define]) {
    _meta.add_binding_entry(
        2,
        (
            UiMaterialBind::as_layout_entry(_visibility),
            BindingExpandDescList::new(
                vec![
                    BindingExpandDesc::new_buffer::<f32>(
                        "world",
                        &[0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
                        TypeKind::Float,
                        TypeSize::Mat { rows: 4, columns: 4 },
                        ArrayLen::None,
                    ),
                    BindingExpandDesc::new_buffer::<f32>(
                        "clipSdfOrSdfline",
                        &[0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
                        TypeKind::Float,
                        TypeSize::Mat { rows: 4, columns: 4 },
                        ArrayLen::None,
                    ),
                    BindingExpandDesc::new_buffer::<f32>("color", &[0.0, 0.0, 0.0, 0.0], TypeKind::Float, TypeSize::Vec(4), ArrayLen::None),
                    BindingExpandDesc::new_buffer::<f32>(
                        "strokeColorOrURect",
                        &[0.0, 0.0, 0.0, 0.0],
                        TypeKind::Float,
                        TypeSize::Vec(4),
                        ArrayLen::None,
                    ),
                    BindingExpandDesc::new_buffer::<f32>(
                        "u_weightAndOffset",
                        &[0.0, 0.0, 0.0, 0.0],
                        TypeKind::Float,
                        TypeSize::Vec(4),
                        ArrayLen::None,
                    ),
                    BindingExpandDesc::new_buffer::<f32>(
                        "u_gradientStarteEnd",
                        &[0.0, 0.0, 0.0, 0.0],
                        TypeKind::Float,
                        TypeSize::Vec(4),
                        ArrayLen::None,
                    ),
                    BindingExpandDesc::new_buffer::<f32>(
                        "textureSizeOrBottomLeftBorder",
                        &[0.0, 0.0],
                        TypeKind::Float,
                        TypeSize::Vec(2),
                        ArrayLen::None,
                    ),
                    BindingExpandDesc::new_buffer::<f32>("data_tex_size", &[0.0, 0.0], TypeKind::Float, TypeSize::Vec(2), ArrayLen::None),
                    BindingExpandDesc::new_buffer::<f32>("slope", &[0.0, 0.0], TypeKind::Float, TypeSize::Vec(2), ArrayLen::None),
                    BindingExpandDesc::new_buffer::<f32>("scale", &[0.0, 0.0], TypeKind::Float, TypeSize::Vec(2), ArrayLen::None),
                    BindingExpandDesc::new_buffer::<f32>("PATCH_2_0", &[0.0, 0.0, 0.0], TypeKind::Float, TypeSize::Vec(3), ArrayLen::None),
                    BindingExpandDesc::new_buffer::<f32>("blur", &[0.0], TypeKind::Float, TypeSize::Scalar, ArrayLen::None),
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
