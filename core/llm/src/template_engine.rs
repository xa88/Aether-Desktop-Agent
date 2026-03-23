use serde::{Deserialize, Serialize};
use handlebars::Handlebars;
use std::collections::HashMap;
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum IntentTaxonomy {
    Dev,
    Research,
    Debug,
    Optimize,
    Refactor,
    Unknown,
}

pub struct LocalTemplateEngine {
    handlebars: Handlebars<'static>,
}

impl LocalTemplateEngine {
    pub fn new() -> Self {
        let mut handlebars = Handlebars::new();
        
        // Register some basic templates
        handlebars.register_template_string("dev", 
            "Task: {{goal}}\nIntent: Development\nPlease provide a step-by-step implementation plan for the requested feature."
        ).unwrap();
        
        handlebars.register_template_string("debug", 
            "Task: {{goal}}\nIntent: Debugging\nPlease analyze the provided error context and suggest a fix."
        ).unwrap();
        
        handlebars.register_template_string("research", 
            "Task: {{goal}}\nIntent: Researching\nPlease gather information about the requested topic and summarize the findings."
        ).unwrap();

        Self { handlebars }
    }

    pub fn render(&self, intent: IntentTaxonomy, data: &HashMap<String, String>) -> Result<String> {
        let template_name = match intent {
            IntentTaxonomy::Dev => "dev",
            IntentTaxonomy::Debug => "debug",
            IntentTaxonomy::Research => "research",
            _ => "dev", // Default to dev for now
        };

        self.handlebars.render(template_name, data).map_err(|e| anyhow::anyhow!("Template error: {}", e))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_templates() {
        let engine = LocalTemplateEngine::new();
        let mut data = HashMap::new();
        data.insert("goal".to_string(), "test goal".to_string());

        // Test dev template
        let result = engine.render(IntentTaxonomy::Dev, &data).unwrap();
        assert!(result.contains("Intent: Development"));
        assert!(result.contains("test goal"));

        // Test debug template
        let result = engine.render(IntentTaxonomy::Debug, &data).unwrap();
        assert!(result.contains("Intent: Debugging"));

        // Test research template
        let result = engine.render(IntentTaxonomy::Research, &data).unwrap();
        assert!(result.contains("Intent: Researching"));
    }
}
