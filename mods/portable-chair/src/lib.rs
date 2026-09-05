use std::time::Duration;
use windows::{
    Win32::{Foundation::HINSTANCE, System::SystemServices::DLL_PROCESS_ATTACH},
    core::BOOL,
};

use eldenring::{
    cs::{
        CSTaskGroupIndex, CSTaskImp, CSWorldGeomMan, ChrInsExt, GeometrySpawnParameters,
        WorldChrMan,
    },
    fd4::FD4TaskData,
    util::{input, system::wait_for_system_init},
};
use fromsoftware_shared::{FromStatic, Program, SharedTaskImpExt};

#[unsafe(no_mangle)]
/// # Safety
///
/// This is exposed this way such that windows LoadLibrary API can call it. Do not call this yourself.
pub extern "C" fn DllMain(_module: HINSTANCE, reason: u32) -> BOOL {
    if reason != DLL_PROCESS_ATTACH {
        return true.into();
    }

    // Kick off new thread.
    std::thread::spawn(move || {
        wait_for_system_init(&Program::current(), Duration::MAX).unwrap();
        let cs_task = CSTaskImp::wait_for_instance(Duration::MAX).unwrap();
        cs_task.run_recurring(
            |_: &FD4TaskData| {
                if !input::is_key_pressed(0x48) {
                    return;
                }

                let Some(player) = unsafe { WorldChrMan::instance() }
                    .ok()
                    .and_then(|w| w.main_player.as_ref())
                else {
                    return;
                };

                let Some(block_geom_data) = unsafe { CSWorldGeomMan::instance_mut() }
                    .ok()
                    .and_then(|wgm| wgm.geom_block_data_by_id_mut(&player.chr_ins.block_id()))
                else {
                    return;
                };

                block_geom_data.spawn_geometry(
                    "AEG099_590",
                    &GeometrySpawnParameters {
                        position: player.block_position,
                        rot_x: 0.0,
                        rot_y: 0.0,
                        rot_z: 0.0,
                        scale_x: 2.0,
                        scale_y: 2.0,
                        scale_z: 2.0,
                    },
                );
            },
            CSTaskGroupIndex::ChrIns_PostPhysics,
        );
    });

    true.into()
}
