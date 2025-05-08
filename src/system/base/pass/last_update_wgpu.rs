
use pi_world::{prelude::SingleRes, single_res::SingleResMut};
use pi_bevy_ecs_extend::prelude::{OrInitSingleResMut, OrInitSingleRes};

use pi_bevy_render_plugin::{PiRenderDevice, PiRenderQueue, PiVertexBufferAlloter};

use crate::resource::{draw_obj::{GroupAlloterCenter, InstanceContext}, GlobalDirtyMark, IsRun};



pub fn last_update_wgpu(

    device: SingleRes<PiRenderDevice>,
    queue: SingleRes<PiRenderQueue>,
    vertbuffer_alloter: OrInitSingleRes<PiVertexBufferAlloter>,
    // index_alloter: OrInitSingleRes<PiIndexBufferAlloter>,
    group_alloc_center: SingleRes<GroupAlloterCenter>,
    // mut depth_cache: OrInitSingleResMut<DepthCache>,
    // mut post_resource: SingleResMut<PostprocessResource>,
    // depth_group_alloter: OrInitSingleRes<ShareGroupAlloter<DepthGroup>>,
	mut instances: OrInitSingleResMut<InstanceContext>,
    mut global_mark: SingleResMut<GlobalDirtyMark>,
	r: OrInitSingleRes<IsRun>
) {
	if r.0 {
		return;
	}

    global_mark.mark = Default::default();

    
    // let time1 = pi_time::Instant::now();
    group_alloc_center.write_buffer(&device, &queue);
    // let time2 = pi_time::Instant::now();
    vertbuffer_alloter.write_buffer();
    // let time3 = pi_time::Instant::now();
    // index_alloter.write_buffer();
	instances.update(&device, &queue);
    // let time4 = pi_time::Instant::now();
	// println!("last_update_wgpu==================={:?}", (time2 - time1, time3 - time2, time4 - time3));
}


