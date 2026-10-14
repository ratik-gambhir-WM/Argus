//! Reference fixtures. No machine measurements or account access are performed.
use super::{model::*, theme};
use gpui_kit::assets::IconName;

impl DashboardData {
    pub(super) fn sample() -> Self {
        use Statistic as Stat;
        let metrics = vec![
            Metric {
                section: Section::Cpu,
                title: "CPU",
                color: theme::BLUE,
                caption: "Now".into(),
                value: "71".into(),
                unit: "%",
                status: None,
                statistics: [
                    Stat::new("User", "49%"),
                    Stat::new("System", "21%"),
                    Stat::new("Average Today", "38%"),
                ],
                chart: CardChart::Bars(vec![
                    18, 20, 21, 25, 31, 38, 39, 38, 36, 38, 39, 41, 42, 39, 37, 37, 39, 40, 39, 39,
                    38, 38, 37, 38, 42, 44, 46, 45, 43, 42, 40, 44, 31, 22, 30, 39, 48, 54, 60, 63,
                ]),
            },
            Metric {
                section: Section::Memory,
                title: "Memory",
                color: theme::PURPLE,
                caption: "In Use of 64 GB".into(),
                value: "54.01".into(),
                unit: "GB",
                status: Some("Normal"),
                statistics: [
                    Stat::colored("App", "25.80 GB", theme::BLUE),
                    Stat::colored("Wired", "5.37 GB", 0xf46932),
                    Stat::colored("Compressed", "22.84 GB", theme::TEAL),
                ],
                chart: CardChart::Bars(vec![
                    74, 76, 75, 77, 76, 76, 75, 77, 76, 77, 76, 77, 77, 76, 76, 77, 77, 76, 77, 77,
                    76, 77, 77, 76, 77, 77, 76, 77, 78, 77, 76, 77, 77, 78, 77, 77, 78, 78, 77, 78,
                ]),
            },
            Metric {
                section: Section::Gpu,
                title: "GPU",
                color: theme::PINK,
                caption: "Apple M2 Max".into(),
                value: "73".into(),
                unit: "%",
                status: None,
                statistics: [
                    Stat::new("Memory", "1.69 GB"),
                    Stat::new("Average", "42%"),
                    Stat::new("Peak", "73%"),
                ],
                chart: CardChart::Bars(vec![
                    23, 21, 17, 13, 11, 9, 11, 12, 15, 18, 19, 15, 17, 21, 22, 24, 25, 22, 21, 19,
                    18, 18, 19, 18, 17, 14, 16, 27, 39, 47, 59, 63, 61, 60, 59, 61, 65, 61, 61, 64,
                ]),
            },
            Metric {
                section: Section::Disk,
                title: "Disk",
                color: theme::ORANGE,
                caption: "Free of 994.66 GB".into(),
                value: "479.72".into(),
                unit: "GB",
                status: None,
                statistics: [
                    Stat::new("Reading", "116 kB/s"),
                    Stat::new("Writing", "291 kB/s"),
                    Stat::new("Written Today", "41.3 GB"),
                ],
                chart: CardChart::Bars(vec![
                    4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 5, 39, 54, 69, 78, 56, 37, 23, 13, 9, 39, 65, 73,
                    76, 56, 36, 23, 15, 8, 4, 4, 4, 4, 4, 4, 45, 64, 73, 54, 33, 23, 15,
                ]),
            },
            Metric {
                section: Section::Network,
                title: "Network",
                color: theme::TEAL,
                caption: "Downloading".into(),
                value: "4.3".into(),
                unit: "MB/s",
                status: None,
                statistics: [
                    Stat::new("Uploading", "117 kB/s"),
                    Stat::new("Today", "6.5 GB"),
                    Stat::new("Last 7 Days", "48.3 GB"),
                ],
                chart: CardChart::Bars(vec![
                    18, 12, 8, 4, 2, 2, 54, 72, 76, 57, 35, 22, 15, 9, 4, 2, 2, 2, 2, 2, 2, 2, 2,
                    2, 2, 53, 78, 73, 66, 49, 34, 21, 12, 49, 68, 74, 82, 59, 57, 58,
                ]),
            },
            Metric {
                section: Section::Battery,
                title: "Battery",
                color: theme::GREEN,
                caption: "On Battery".into(),
                value: "57".into(),
                unit: "%",
                status: None,
                statistics: [
                    Stat::new("Remaining", "2h 20m"),
                    Stat::new("Power Draw", "17.0 W"),
                    Stat::new("Health", "76%"),
                ],
                chart: CardChart::Bars(vec![53; 44]),
            },
            Metric {
                section: Section::Neural,
                title: "Neural Engine",
                color: theme::PURPLE,
                caption: "16-core Neural Engine".into(),
                value: "45".into(),
                unit: "%",
                status: None,
                statistics: [
                    Stat::new("Memory Traffic", "178.9 MB/s"),
                    Stat::new("Average", "18%"),
                    Stat::new("Peak", "45%"),
                ],
                chart: CardChart::Bars(vec![
                    28, 34, 30, 27, 24, 12, 5, 2, 2, 2, 14, 29, 31, 16, 7, 4, 2, 15, 29, 38, 22,
                    10, 4, 2, 2, 2, 2, 13, 18, 22, 23, 35, 27, 24, 17, 9, 5, 4, 4, 15, 28, 31, 28,
                    36,
                ]),
            },
            Metric {
                section: Section::Agents,
                title: "Agents",
                color: theme::VIOLET,
                caption: "Claude · Max 20x · This week".into(),
                value: "72%".into(),
                unit: "left",
                status: None,
                statistics: [
                    Stat::new("Resets", "4d 2h"),
                    Stat::new("Sessions", "2"),
                    Stat::new("Tokens Today", "1.78M"),
                ],
                chart: CardChart::Providers,
            },
        ];
        let item = |label, display_value: &'static str, amount, color, icon| BreakdownItem {
            label,
            display_value: display_value.into(),
            amount,
            color,
            icon,
        };
        let breakdowns = vec![
            Breakdown {
                section: Section::Memory,
                title: "Memory by Type",
                icon: IconName::MemoryStick,
                color: theme::BLUE,
                center_value: "82%".into(),
                center_caption: "in use",
                items: vec![
                    item("App", "22.15 GB", 22.15, theme::BLUE, None),
                    item("Wired", "5.67 GB", 5.67, 0xf46932, None),
                    item("Compressed", "24.54 GB", 24.54, theme::TEAL, None),
                    item("Cached", "10.32 GB", 10.32, 0xa1a1a8, None),
                    item("Free", "1.32 GB", 1.32, theme::TRACK, None),
                ],
            },
            Breakdown {
                section: Section::Memory,
                title: "Memory by App",
                icon: IconName::LayoutGrid,
                color: theme::PURPLE,
                center_value: "59.25 GB".into(),
                center_caption: "all apps",
                items: vec![
                    item(
                        "Google Chrome",
                        "21.23 GB",
                        21.23,
                        0x5040ad,
                        Some(IconName::Globe),
                    ),
                    item("VS Code", "8.17 GB", 8.17, 0x6b59ba, Some(IconName::Code)),
                    item(
                        "Terminal",
                        "3.86 GB",
                        3.86,
                        0x897bc9,
                        Some(IconName::Terminal),
                    ),
                    item("Safari", "2.69 GB", 2.69, 0xa498d3, Some(IconName::Compass)),
                    item("Other", "23.30 GB", 23.30, theme::TRACK, None),
                ],
            },
            Breakdown {
                section: Section::Battery,
                title: "Power by App",
                icon: IconName::Zap,
                color: theme::GREEN,
                center_value: "1.7 W".into(),
                center_caption: "all apps",
                items: vec![
                    item(
                        "GitHub Desktop",
                        "427 mW",
                        427.,
                        theme::GREEN,
                        Some(IconName::Github),
                    ),
                    item(
                        "Google Chrome",
                        "367 mW",
                        367.,
                        0x29992c,
                        Some(IconName::Globe),
                    ),
                    item("Zoom", "145 mW", 145., 0x50ab50, Some(IconName::Video)),
                    item("Terminal", "86 mW", 86., 0x80bf80, Some(IconName::Terminal)),
                    item("Other", "650 mW", 650., theme::TRACK, None),
                ],
            },
        ];
        Self {
            metrics,
            breakdowns,
        }
    }
}
