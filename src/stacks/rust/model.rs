const TOKIO: Dep = dep("tokio", &["full"]);
const SERDE: Dep = dep("serde", &["derive"]);
const TRACING: Dep = dep("tracing", &[]);
const TRACING_SUB: Dep = dep("tracing-subscriber", &[]);

const H1_DEPS: &[Dep] = &[TOKIO, dep("axum", &[]), SERDE, TRACING, TRACING_SUB];
const H2_DEPS: &[Dep] = &[TOKIO, dep("axum", &["http2"]), SERDE, TRACING, TRACING_SUB];
const H3_DEPS: &[Dep] = &[
    TOKIO,
    dep("quinn", &[]),
    dep("h3", &[]),
    dep("h3-quinn", &[]),
    TRACING,
    TRACING_SUB,
];

pub const EXTRA_LIBS: &[(Dep, &str)] = &[
    (
        dep("sqlx", &["runtime-tokio", "postgres"]),
        "PostgreSQL, Migrations",
    ),
    (dep("redis", &["tokio-comp"]), "cache, sessions"),
    (
        dep("tower-http", &["cors", "trace"]),
        "CORS, Request Logging",
    ),
    (dep("validator", &["derive"]), "DTO validation"),
    (dep("thiserror", &[]), "common errors"),
    (dep("anyhow", &[]), "Common Mistakes in the main Function"),
    (dep("dotenvy", &[]), "Loading .env"),
    (dep("uuid", &["v4", "serde"]), "identifiers"),
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Dep {
    pub name: &'static str,
    pub features: &'static [&'static str],
}

const fn dep(name: &'static str, features: &'static [&'static str]) -> Dep {
    Dep { name, features }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HttpVersions {
    #[default]
    H1,
    H2,
    H3,
}

impl HttpVersions {
    pub fn label(self) -> &'static str {
        match self {
            Self::H1 => "HTTP/1.1",
            Self::H2 => "HTTP/2",
            Self::H3 => "HTTP/3",
        }
    }

    pub fn base_deps(self) -> &'static [Dep] {
        match self {
            Self::H1 => H1_DEPS,
            Self::H2 => H2_DEPS,
            Self::H3 => H3_DEPS,
        }
    }
}

#[derive(Debug, Default)]
pub struct RustAnswers {
    pub http: HttpVersions,
    pub extra_deps: Vec<Dep>,
}

impl RustAnswers {
    pub fn all_deps(&self) -> impl Iterator<Item = &Dep> {
        self.http.base_deps().iter().chain(&self.extra_deps)
    }

    pub fn summary(&self) -> String {
        let deps: Vec<&str> = self.all_deps().map(|d| d.name).collect();
        format!(
            "HTTP   {}\nCretaceous   {}",
            self.http.label(),
            deps.join(", ")
        )
    }
}
