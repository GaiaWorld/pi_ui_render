use pi_render::rhi::shader::{
    merge_defines, BlockCodeAtom, CodeSlice, Define, InOut, ShaderInput, ShaderMeta, ShaderOutput, ShaderProgram, ShaderVarying,
};
use render_derive::Input;


#[derive(Input)]
#[location(0)]
pub struct PositionVert;


#[derive(Input)]
#[location(1)]
pub struct VertColorVert;


pub struct ProgramMeta;
impl ShaderProgram for ProgramMeta {
    fn create_meta() -> pi_render::rhi::shader::ShaderMeta {
        let mut meta = ShaderMeta::default();
        meta.name = std::any::type_name::<Self>().to_string();
        let _defines: &[Define] = &[];
        let _meta = &mut meta;
        let _visibility = wgpu::ShaderStages::VERTEX;

        super::depth::push_meta(_meta, _visibility, &[]);
        super::camera::push_meta(_meta, _visibility, &[]);
        let _visibility = wgpu::ShaderStages::FRAGMENT;


        super::sdf::push_meta(_meta, _visibility, &[]);
        let _visibility = wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT;


        super::ui_meterial::push_meta(_meta, _visibility, &[]);
        push_vs_code(&mut meta.vs);
        push_fs_code(&mut meta.fs);
        meta.varyings = ShaderVarying(vec![
            InOut::new("vVertexPosition", "vec2", 0, vec![]),
            InOut::new("vColor", "vec4", 1, vec![Define::new(true, VERT_COLOR_DEFINE.clone())]),
        ]);
        meta.ins = ShaderInput(vec![
            InOut::new("position", "vec2", 0, vec![]),
            InOut::new("vertColor", "vec4", 1, vec![Define::new(true, VERT_COLOR_DEFINE.clone())]),
        ]);
        meta.outs = ShaderOutput(vec![InOut::new("o_Target", "vec4", 0, vec![])]);
        meta
    }
}
fn push_vs_code(_codes: &mut BlockCodeAtom) {
    let _defines: &[Define] = &[];
    _codes.define.push(VS_CODE[0].clone().push_defines_front(_defines));
    super::camera::push_code(_codes, merge_defines(_defines, &[]).as_slice());
    super::depth::push_code(_codes, merge_defines(_defines, &[]).as_slice());
    super::ui_meterial::push_code(_codes, merge_defines(_defines, &[]).as_slice());
    _codes.running.push(VS_CODE[1].clone().push_defines_front(_defines));
    _codes.running.push(VS_CODE[2].clone().push_defines_front(_defines));
}

fn push_fs_code(_codes: &mut BlockCodeAtom) {
    let _defines: &[Define] = &[];
    _codes.define.push(FS_CODE[0].clone().push_defines_front(_defines));
    super::ui_meterial::push_code(_codes, merge_defines(_defines, &[]).as_slice());
    super::sdf::push_code(_codes, merge_defines(_defines, &[]).as_slice());
    _codes.define.push(FS_CODE[1].clone().push_defines_front(_defines));
    _codes.running.push(FS_CODE[2].clone().push_defines_front(_defines));
    _codes.running.push(FS_CODE[3].clone().push_defines_front(_defines));
    _codes.running.push(FS_CODE[4].clone().push_defines_front(_defines));
    _codes.running.push(FS_CODE[5].clone().push_defines_front(_defines));
    _codes.running.push(FS_CODE[6].clone().push_defines_front(_defines));
    _codes.running.push(FS_CODE[7].clone().push_defines_front(_defines));
}

lazy_static! {
    static ref VS_CODE: Vec<CodeSlice> = vec![
        CodeSlice {
            code: pi_atom::Atom::from(
                "#version 450
"
            ),
            defines: vec![]
        },
        CodeSlice {
            code: pi_atom::Atom::from(
                "	vVertexPosition = position;
	gl_Position = project * view * world * vec4(position.x, position.y, 1.0, 1.0);
	gl_Position.z = depth/60000.0;
"
            ),
            defines: vec![]
        },
        CodeSlice {
            code: pi_atom::Atom::from(
                "		vColor = vertColor;
"
            ),
            defines: vec![Define::new(true, VERT_COLOR_DEFINE.clone())]
        }
    ];
    static ref FS_CODE: Vec<CodeSlice> = vec![
        CodeSlice {
            code: pi_atom::Atom::from(
                "#version 450

precision highp float;
"
            ),
            defines: vec![]
        },
        CodeSlice {
            code: pi_atom::Atom::from(
                "	float erf(float x) {

		bool negative = x < 0.0;

		if (negative)

			x = -x;

		float x2 = x * x;

		float x3 = x2 * x;

		float x4 = x2 * x2;

		float denom = 1.0 + 0.278393 * x + 0.230389 * x2 + 0.000972 * x3 + 0.078108 * x4;

		float result = 1.0 - 1.0 / (denom * denom * denom * denom);

		return negative ? -result : result;

	}

	float erfSigma(float x, float sigma) {

		return erf(x / (sigma * 1.4142135623730951));

	}

	float colorFromRect(vec2 p0, vec2 p1, float sigma) {

		return (erfSigma(p1.x, sigma) - erfSigma(p0.x, sigma)) *

			(erfSigma(p1.y, sigma) - erfSigma(p0.y, sigma)) / 4.0;

	}

	float getShadowAlpha(vec2 pos, vec2 ptMin, vec2 ptMax, float sigma) {

		vec2 dMin = pos - ptMin, dMax = pos - ptMax;

		return colorFromRect(dMin, dMax, sigma);

	}
"
            ),
            defines: vec![Define::new(true, SHADOW_DEFINE.clone())]
        },
        CodeSlice {
            code: pi_atom::Atom::from(
                "	vec4 c = color.rgba;
"
            ),
            defines: vec![]
        },
        CodeSlice {
            code: pi_atom::Atom::from(
                "		c = vColor;
"
            ),
            defines: vec![Define::new(true, VERT_COLOR_DEFINE.clone())]
        },
        CodeSlice {
            code: pi_atom::Atom::from(
                "		c.a = c.a * getShadowAlpha(vVertexPosition, strokeColorOrURect.xy, strokeColorOrURect.zw, blur / 2.0);
"
            ),
            defines: vec![Define::new(true, SHADOW_DEFINE.clone())]
        },
        CodeSlice {
            code: pi_atom::Atom::from(
                "		c.a = c.a * calc_alpha(vVertexPosition, clipSdfOrSdfline);
"
            ),
            defines: vec![Define::new(true, BORDER_RADIUS_DEFINE.clone())]
        },
        CodeSlice {
            code: pi_atom::Atom::from(
                "		c.a = c.a * calc_alpha(vVertexPosition, clipSdfOrSdfline);
"
            ),
            defines: vec![Define::new(true, BORDER_DEFINE.clone())]
        },
        CodeSlice {
            code: pi_atom::Atom::from(
                "	o_Target = c;
"
            ),
            defines: vec![]
        }
    ];
    pub static ref BORDER_DEFINE: pi_atom::Atom = pi_atom::Atom::from("BORDER");
    pub static ref SHADOW_DEFINE: pi_atom::Atom = pi_atom::Atom::from("SHADOW");
    pub static ref BORDER_RADIUS_DEFINE: pi_atom::Atom = pi_atom::Atom::from("BORDER_RADIUS");
    pub static ref VERT_COLOR_DEFINE: pi_atom::Atom = pi_atom::Atom::from("VERT_COLOR");
}
