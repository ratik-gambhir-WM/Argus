//! Typed presentation data, independent of rendering and telemetry sources.
use gpui_kit::{SharedString, assets::IconName};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(super) enum Section {
    #[default]
    Overview,
    Cpu,
    Memory,
    Disk,
    Network,
    Gpu,
    Neural,
    Battery,
    Sound,
    Bluetooth,
    Projects,
    Agents,
    Containers,
}

impl Section {
    pub(super) const ALL: [Self; 13] = [
        Self::Overview,
        Self::Cpu,
        Self::Memory,
        Self::Disk,
        Self::Network,
        Self::Gpu,
        Self::Neural,
        Self::Battery,
        Self::Sound,
        Self::Bluetooth,
        Self::Projects,
        Self::Agents,
        Self::Containers,
    ];

    pub(super) fn label(self) -> &'static str {
        match self {
            Self::Overview => "Overview",
            Self::Cpu => "CPU",
            Self::Memory => "Memory",
            Self::Disk => "Disk",
            Self::Network => "Network",
            Self::Gpu => "GPU",
            Self::Neural => "Neural",
            Self::Battery => "Battery",
            Self::Sound => "Sound",
            Self::Bluetooth => "Bluetooth",
            Self::Projects => "Projects",
            Self::Agents => "Agents",
            Self::Containers => "Containers",
        }
    }

    pub(super) fn icon(self) -> IconName {
        match self {
            Self::Overview => IconName::LayoutGrid,
            Self::Cpu => IconName::Cpu,
            Self::Memory => IconName::MemoryStick,
            Self::Disk => IconName::HardDrive,
            Self::Network => IconName::Globe,
            Self::Gpu => IconName::Layers,
            Self::Neural => IconName::Brain,
            Self::Battery => IconName::Battery,
            Self::Sound => IconName::Volume2,
            Self::Bluetooth => IconName::Bluetooth,
            Self::Projects => IconName::Folder,
            Self::Agents => IconName::Bot,
            Self::Containers => IconName::Box,
        }
    }
}

pub(super) struct Statistic {
    pub label: &'static str,
    pub value: SharedString,
    pub color: Option<u32>,
}

impl Statistic {
    pub(super) fn new(label: &'static str, value: impl Into<SharedString>) -> Self {
        Self {
            label,
            value: value.into(),
            color: None,
        }
    }

    pub(super) fn colored(label: &'static str, value: impl Into<SharedString>, color: u32) -> Self {
        Self {
            label,
            value: value.into(),
            color: Some(color),
        }
    }
}

pub(super) enum CardChart {
    Bars(Vec<u8>),
    Providers,
}

pub(super) struct Metric {
    pub section: Section,
    pub title: &'static str,
    pub color: u32,
    pub caption: SharedString,
    pub value: SharedString,
    pub unit: &'static str,
    pub status: Option<&'static str>,
    pub statistics: [Statistic; 3],
    pub chart: CardChart,
}

pub(super) struct BreakdownItem {
    pub label: &'static str,
    pub display_value: SharedString,
    pub amount: f32,
    pub color: u32,
    pub icon: Option<IconName>,
}

pub(super) struct Breakdown {
    pub section: Section,
    pub title: &'static str,
    pub icon: IconName,
    pub color: u32,
    pub center_value: SharedString,
    pub center_caption: &'static str,
    pub items: Vec<BreakdownItem>,
}

pub(super) struct DashboardData {
    pub metrics: Vec<Metric>,
    pub breakdowns: Vec<Breakdown>,
}
