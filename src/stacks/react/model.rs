use super::manager_packet::PackageManager;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Architecture {
    #[default]
    Modular,
    Fsd,
    FeatureOriented,
    AtomicDesign,
    Classic,
}

impl Architecture {
    pub const ALL: [Self; 5] = [
        Self::Modular,
        Self::Fsd,
        Self::FeatureOriented,
        Self::AtomicDesign,
        Self::Classic,
    ];

    pub fn label(self) -> &'static str {
        match self {
            Self::Modular => "Modular",
            Self::Fsd => "Fsd",
            Self::FeatureOriented => "FeatureOriented",
            Self::AtomicDesign => "AtomicDesign",
            Self::Classic => "Classic",
        }
    }

    pub fn hint(self) -> &'static str {
        match self {
            Self::Modular => "modules/ + core, libs; inside the domain/store/ui module",
            Self::Fsd => "app → pages → widgets → features → entities → shared",
            Self::FeatureOriented => "features/ based on business logic",
            Self::AtomicDesign => "atoms → molecules → organisms → templates",
            Self::Classic => "components / containers / store, as shown in the tutorials",
        }
    }

    pub fn files(self) -> &'static [(&'static str, &'static str)] {
        match self {
            Self::Modular => &[
                (
                    "src/core/index.ts",
                    "// App core: providers, router, global config\nexport {};\n",
                ),
                (
                    "src/shared/ui/index.ts",
                    "// UI kit shared between modules\nexport {};\n",
                ),
                (
                    "src/libs/index.ts",
                    "// Wrappers over third-party libraries\nexport {};\n",
                ),
                (
                    "src/modules/example/domain/index.ts",
                    "// Types and pure business logic\nexport {};\n",
                ),
                (
                    "src/modules/example/store/index.ts",
                    "// Module state\nexport {};\n",
                ),
                (
                    "src/modules/example/ui/index.ts",
                    "// Module components\nexport {};\n",
                ),
                (
                    "src/modules/example/index.ts",
                    "// Public API of the module: export only what other modules may use\nexport {};\n",
                ),
            ],
            Self::Fsd => &[
                (
                    "src/app/index.ts",
                    "// App layer: providers, routing, global styles\nexport {};\n",
                ),
                (
                    "src/pages/index.ts",
                    "// Pages: compose widgets and features\nexport {};\n",
                ),
                (
                    "src/widgets/index.ts",
                    "// Widgets: large self-contained UI blocks\nexport {};\n",
                ),
                (
                    "src/features/index.ts",
                    "// Features: user scenarios with business value\nexport {};\n",
                ),
                (
                    "src/entities/index.ts",
                    "// Entities: business objects\nexport {};\n",
                ),
                ("src/shared/ui/index.ts", "// UI kit\nexport {};\n"),
                ("src/shared/api/index.ts", "// API client\nexport {};\n"),
                ("src/shared/lib/index.ts", "// Helpers\nexport {};\n"),
            ],
            Self::FeatureOriented => &[
                (
                    "src/features/index.ts",
                    "// Features grouped by business logic\nexport {};\n",
                ),
                (
                    "src/shared/components/index.ts",
                    "// Reusable components\nexport {};\n",
                ),
                (
                    "src/shared/hooks/index.ts",
                    "// Reusable hooks\nexport {};\n",
                ),
                ("src/api/index.ts", "// API client\nexport {};\n"),
            ],
            Self::AtomicDesign => &[
                (
                    "src/components/atoms/index.ts",
                    "// Atoms: buttons, inputs, labels\nexport {};\n",
                ),
                (
                    "src/components/molecules/index.ts",
                    "// Molecules: groups of atoms\nexport {};\n",
                ),
                (
                    "src/components/organisms/index.ts",
                    "// Organisms: sections of the interface\nexport {};\n",
                ),
                (
                    "src/components/templates/index.ts",
                    "// Templates: page layouts\nexport {};\n",
                ),
                (
                    "src/pages/index.ts",
                    "// Pages: templates filled with data\nexport {};\n",
                ),
            ],
            Self::Classic => &[
                ("src/api/index.ts", "// API client\nexport {};\n"),
                (
                    "src/components/index.ts",
                    "// Presentational components\nexport {};\n",
                ),
                (
                    "src/containers/index.ts",
                    "// Containers: components connected to state\nexport {};\n",
                ),
                ("src/store/index.ts", "// Global state\nexport {};\n"),
                ("src/pages/index.ts", "// Pages\nexport {};\n"),
            ],
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NpmLib {
    pub label: &'static str,
    pub packages: &'static [&'static str],
    pub hint: &'static str,
}

const fn npm(label: &'static str, packages: &'static [&'static str], hint: &'static str) -> NpmLib {
    NpmLib {
        label,
        packages,
        hint,
    }
}

pub const NPM_LIBS: &[NpmLib] = &[
    npm("react-router", &["react-router"], "routing"),
    npm(
        "tanstack-query",
        &["@tanstack/react-query"],
        "API requests, cache",
    ),
    npm("axios", &["axios"], "HTTP-client"),
    npm("zustand", &["zustand"], "easy state"),
    npm(
        "redux-toolkit",
        &["@reduxjs/toolkit", "react-redux"],
        "Redux",
    ),
    npm(
        "react-hook-form",
        &["react-hook-form", "zod", "@hookform/resolvers"],
        "Forms + Validation",
    ),
    npm("clsx", &["clsx"], "склейка CSS-классов"),
];

#[derive(Debug, Default)]
pub struct ReactAnswers {
    pub pm: PackageManager,
    pub arch: Architecture,
    pub libs: Vec<NpmLib>,
}

impl ReactAnswers {
    pub fn summary(&self) -> String {
        let libs: Vec<&str> = self.libs.iter().map(|l| l.label).collect();
        let libs = if libs.is_empty() {
            "—".to_string()
        } else {
            libs.join(", ")
        };
        format!(
            "Manager {}\nArch.   {}\nLib. {}",
            self.pm.label(),
            self.arch.label(),
            libs
        )
    }
}
