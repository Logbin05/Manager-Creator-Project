use crate::templates::{
  generated::{self, TemplateAnswer}, structures::Template, template,
};

#[derive(Debug, Default)]
pub struct ProjectSpec {
    pub template: usize,
    pub name: String,
    pub answers: TemplateAnswer,
}

impl ProjectSpec {
    pub fn template(&self) -> &'static Template {
        &template::ALL[self.template]
    }

    pub fn summary(&self) -> String {
        self.answers.summary(self.template())
    }

    pub fn run_hint(&self) -> String {
        generated::run_hint(self.template(), &self.name, &self.answers)
    }
}