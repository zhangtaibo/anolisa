pub(crate) mod broker;
pub(crate) mod classification;
pub(crate) mod command_risk;
pub mod display;
pub(crate) mod guarded_diagnostic;
pub(crate) mod readonly_pipeline;
pub(crate) mod readonly_rules;

pub use broker::{apply_readonly_config, can_run_approved_bash_tool};
pub use classification::{
    classify_command_interaction, is_readonly_builtin_tool_name, is_shell_tool_name,
    obvious_tty_command_reason, provider_tool_class, ApprovalRisk, CommandInteractionProfile,
    OutputStability, ProviderToolClass, PtyRequirement,
};
pub use command_risk::{
    assess_shell_command, blocked_shell_binding_assessment, AssessmentConfidence, AssessmentPolicy,
    AssessmentSource, AssessmentSummary, AutoAllowEvidence, AutoExecutionPolicy,
    AutoExecutionRoute, CommandAssessment, CommandShape, ExecutionDecision, InteractionRequirement,
    OutputExposure, OutputStability as CommandRiskOutputStability, ReadonlyEvidence, RiskImpact,
    RiskReason, SideEffectClass,
};
pub use guarded_diagnostic::{
    run_guarded_diagnostic, validate_guarded_diagnostic, GuardedDiagnosticConfig,
    GuardedDiagnosticError, GuardedDiagnosticOutput, GuardedDiagnosticPlan,
};
pub use readonly_pipeline::{
    run_readonly_pipeline, validate_readonly_pipeline, ReadonlyPipelineConfig,
    ReadonlyPipelineError, ReadonlyPipelineOutput, ReadonlyPipelinePlan, ReadonlyPipelineStage,
};
