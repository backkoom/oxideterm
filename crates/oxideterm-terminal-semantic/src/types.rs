// Copyright (C) 2026 AnalyseDeCircuit
// SPDX-License-Identifier: GPL-3.0-only

use std::ops::Range;

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SemanticClass {
    Command,
    Keyword,
    Option,
    Operator,
    String,
    Variable,
    Comment,
    Link,
    Path,
    Address,
    Weekday,
    Month,
    Timestamp,
    PermissionRead,
    PermissionWrite,
    PermissionExecute,
    PermissionSpecial,
    Number,
    /// 显式不着色：把一段文本钉回终端默认前景色。
    ///
    /// 用在「这一列不该被任何规则染色」的场景，比如 df 的 Size 列——它只是参照值，
    /// 而通用的「数字+单位」规则会把 248G 染成品红。给这段标 Plain 并配上更高
    /// 优先级，就能压住那些规则。
    Plain,
    Error,
    Warning,
    Success,
    Info,
}

pub const SEMANTIC_CLASSES: &[SemanticClass] = &[
    SemanticClass::Command,
    SemanticClass::Keyword,
    SemanticClass::Option,
    SemanticClass::Operator,
    SemanticClass::String,
    SemanticClass::Variable,
    SemanticClass::Comment,
    SemanticClass::Link,
    SemanticClass::Path,
    SemanticClass::Address,
    SemanticClass::Weekday,
    SemanticClass::Month,
    SemanticClass::Timestamp,
    SemanticClass::PermissionRead,
    SemanticClass::PermissionWrite,
    SemanticClass::PermissionExecute,
    SemanticClass::PermissionSpecial,
    SemanticClass::Number,
    SemanticClass::Error,
    SemanticClass::Warning,
    SemanticClass::Success,
    SemanticClass::Info,
    // 追加在末尾：这个数组的下标就是设置页里自定义颜色的索引，
    // 中间插入会让用户已保存的配色整体错位。
    SemanticClass::Plain,
];

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SemanticShellDialect {
    #[default]
    Auto,
    Bash,
    Zsh,
    Fish,
    PowerShell,
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum SemanticLineRole {
    Command,
    Output,
    PsAuxOutput,
    PsFullOutput,
    RustToolOutput,
    CCompilerOutput,
    GitStatusOutput,
    GitDiffOutput,
    GitLogOutput,
    SystemdOutput,
    TestOutput,
    ContainerOutput,
    FileListingOutput,
    FileListingOwnerOutput,
    FileListingGroupOutput,
    FileListingAnonymousOutput,
    FileStatOutput,
    FileAclOutput,
    DiskUsageOutput,
    MemoryUsageOutput,
    IpOutput,
    SocketOutput,
    PingOutput,
    #[default]
    Unknown,
}

impl SemanticLineRole {
    pub(crate) fn is_output(self) -> bool {
        matches!(
            self,
            Self::Output
                | Self::PsAuxOutput
                | Self::PsFullOutput
                | Self::RustToolOutput
                | Self::CCompilerOutput
                | Self::GitStatusOutput
                | Self::GitDiffOutput
                | Self::GitLogOutput
                | Self::SystemdOutput
                | Self::TestOutput
                | Self::ContainerOutput
                | Self::FileListingOutput
                | Self::FileListingOwnerOutput
                | Self::FileListingGroupOutput
                | Self::FileListingAnonymousOutput
                | Self::FileStatOutput
                | Self::FileAclOutput
                | Self::DiskUsageOutput
                | Self::MemoryUsageOutput
                | Self::IpOutput
                | Self::SocketOutput
                | Self::PingOutput
        )
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum SemanticScheme {
    #[default]
    Balanced,
    Conservative,
}

impl SemanticScheme {
    pub(crate) const fn includes(self, class: SemanticClass) -> bool {
        match self {
            Self::Balanced => true,
            // Generic numbers and informational words are the two classes
            // most likely to make ordinary terminal output visually noisy.
            Self::Conservative => !matches!(class, SemanticClass::Number | SemanticClass::Info),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SemanticSpan {
    pub range: Range<usize>,
    pub class: SemanticClass,
    /// Optional presentation variant for semantic peers such as nested brackets.
    pub style_variant: Option<u8>,
}

impl SemanticSpan {
    pub(crate) fn new(range: Range<usize>, class: SemanticClass) -> Self {
        Self {
            range,
            class,
            style_variant: None,
        }
    }
}
