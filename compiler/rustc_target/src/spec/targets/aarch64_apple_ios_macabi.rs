use crate::spec::base::apple::{mac_catalyst_llvm_target, opts, pre_link_args, Arch};
use crate::spec::{FramePointer, MaybeLazy, SanitizerSet, Target, TargetOptions};

pub fn target() -> Target {
    const ARCH: Arch = Arch::Arm64_macabi;
    const OS: &'static str = "ios";

    let mut base = opts(OS, ARCH, MaybeLazy::lazy(|| pre_link_args(OS, ARCH)));
    base.supported_sanitizers = SanitizerSet::ADDRESS | SanitizerSet::LEAK | SanitizerSet::THREAD;

    Target {
        llvm_target: MaybeLazy::lazy(|| mac_catalyst_llvm_target(ARCH)),
        metadata: crate::spec::TargetMetadata {
            description: None,
            tier: None,
            host_tools: None,
            std: None,
        },
        pointer_width: 64,
        data_layout: "e-m:o-i64:64-i128:128-n32:64-S128".into(),
        arch: ARCH.target_arch(),
        options: TargetOptions {
            features: "+neon,+fp-armv8,+apple-a12".into(),
            max_atomic_width: Some(128),
            frame_pointer: FramePointer::NonLeaf,
            ..base
        },
    }
}
