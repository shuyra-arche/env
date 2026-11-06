use env::{
    FlatReference, JoinReply, MemoryResult, Reason, RoomResult, TaskId, TeamId,
    UnitResult, UnitTarget, ViewControl, ViewId, VirtAddr, Wait,
};

#[test]
fn legacy_paths_keep_raw_signatures_and_the_canonical_handle_owner() {
    let _: fn(TeamId, usize, VirtAddr, usize, usize) -> UnitResult<TaskId> = env::unit::spawn;
    let _: fn(UnitTarget, Wait, bool) -> UnitResult<JoinReply> = env::unit::join;
    let _: fn(usize) -> RoomResult<()> = env::room::park;
    let _: fn(VirtAddr, TeamId, VirtAddr, u64, ViewControl) -> MemoryResult<(ViewId, VirtAddr)> =
        env::memory::attach;
    let _: fn(FlatReference) -> MemoryResult<()> = env::memory::release;
    let _: fn(TaskId, Wait) -> UnitResult<bool> = env::unit::join_task;
    let _: fn(TaskId) -> UnitResult<()> = env::unit::embark_task;
    let _: fn(TaskId) -> UnitResult<()> = env::unit::debark_task;
    let _: fn(TaskId) -> UnitResult<()> = env::unit::slay_task;
    let _: fn(TeamId) -> UnitResult<()> = env::unit::embark_team;
    let _: fn(TeamId) -> UnitResult<()> = env::unit::debark_team;
    let _: fn(TeamId) -> UnitResult<()> = env::unit::slay_team;
    let _: unsafe fn(usize, [usize; 6]) -> (usize, usize, usize) = env::ecall::trap;
    let _: fn(kernel_client::TaskId) -> kernel_api_env::TaskId = core::convert::identity;
    let _: fn(kernel_api_env::Reason) -> Reason = core::convert::identity;
}
