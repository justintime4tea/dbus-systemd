use strum::{AsRefStr, AsStaticStr, IntoStaticStr};

#[derive(AsRefStr, AsStaticStr, IntoStaticStr)]
pub enum Mode {
    /// Start the unit and its dependencies, while maybe replacing existing jobs related to unit.
    Replace,
    /// Start the unit and its dependencies but fail if this would affect an existing job.
    Fail,
    /// Start the unit and terminate all units that aren't dependencies of it.
    Isolate,
    /// Start a unit but ignore all its dependencies.
    IgnoreDependencies,
    /// Start a unit but only ignore the requirement dependencies.
    IgnoreRequirements,
}
