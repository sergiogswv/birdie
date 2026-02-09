use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Contexto de tarea para buscar pestañas y extraer información
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ContextTask {
    pub app_name: String,              // Nombre de la app (ej: "Google Meet")
    pub search_targets: Vec<String>,    // Nombres de pestañas a buscar (ej: ["Meet", "Google Meet"])
    pub url_patterns: Vec<String>,      // Patrones de URL (ej: ["meet.google.com"])
    pub css_selector: String,           // Selector CSS básico para extraer contenido
    pub priority: TaskPriority,         // Prioridad de procesamiento
    pub enabled: bool,                  // Si está habilitado
}

/// Nivel de prioridad para procesamiento
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TaskPriority {
    /// Deshabilitado - solo mostrar notificación visual
    Disabled,
    /// Bajo - procesar si hay recursos disponibles
    Low,
    /// Normal - procesar siempre
    Normal,
    /// Alto - prioridad sobre otros
    High,
    /// Crítico - máxima prioridad
    Critical,
}

/// Mapeo centralizado de apps con sus contextos
pub struct ContextMapper {
    tasks: HashMap<String, ContextTask>,
}

impl ContextMapper {
    /// Crea un nuevo mapeo con configuración por defecto
    pub fn new() -> Self {
        Self {
            tasks: Self::default_mappings(),
        }
    }

    /// Definiciones de mapeos por defecto
    fn default_mappings() -> HashMap<String, ContextTask> {
        let mut tasks = HashMap::new();

        // Google Meet
        tasks.insert(
            "Google Meet".to_string(),
            ContextTask {
                app_name: "Google Meet".to_string(),
                search_targets: vec!["Meet".to_string(), "Google Meet".to_string()],
                url_patterns: vec!["meet.google.com".to_string()],
                css_selector: r#"
                    (function() {
                        let content = [];
                        const messages = document.querySelectorAll('[data-is-own-message="false"]');
                        messages.forEach(msg => {
                            const text = msg.textContent?.trim();
                            if (text) content.push(text);
                        });
                        return content.join('\n') || 'No messages';
                    })()
                "#
                .to_string(),
                priority: TaskPriority::High,
                enabled: true,
            },
        );

        // Microsoft Teams
        tasks.insert(
            "Microsoft Teams".to_string(),
            ContextTask {
                app_name: "Microsoft Teams".to_string(),
                search_targets: vec!["Teams".to_string(), "Microsoft Teams".to_string()],
                url_patterns: vec!["teams.microsoft.com".to_string()],
                css_selector: r#"
                    (function() {
                        let content = [];
                        const messages = document.querySelectorAll('[data-testid="message-content"]');
                        messages.forEach(msg => {
                            const text = msg.textContent?.trim();
                            if (text) content.push(text);
                        });
                        return content.join('\n') || 'No messages';
                    })()
                "#
                .to_string(),
                priority: TaskPriority::High,
                enabled: true,
            },
        );

        // Discord
        tasks.insert(
            "Discord".to_string(),
            ContextTask {
                app_name: "Discord".to_string(),
                search_targets: vec!["Discord".to_string()],
                url_patterns: vec!["discord.com".to_string()],
                css_selector: r#"
                    (function() {
                        let content = [];
                        const messages = document.querySelectorAll('[data-testid="message-content"]');
                        messages.forEach(msg => {
                            const text = msg.textContent?.trim();
                            if (text) content.push(text);
                        });
                        return content.join('\n') || 'No messages';
                    })()
                "#
                .to_string(),
                priority: TaskPriority::High,
                enabled: true,
            },
        );

        // Slack
        tasks.insert(
            "Slack".to_string(),
            ContextTask {
                app_name: "Slack".to_string(),
                search_targets: vec!["Slack".to_string(), "app.slack.com".to_string()],
                url_patterns: vec!["app.slack.com".to_string(), "slack.com".to_string()],
                css_selector: r#"
                    (function() {
                        let content = [];
                        const messages = document.querySelectorAll('[data-qa="virtual_list_item"]');
                        messages.forEach(msg => {
                            const text = msg.textContent?.trim();
                            if (text) content.push(text);
                        });
                        return content.join('\n') || 'No messages';
                    })()
                "#
                .to_string(),
                priority: TaskPriority::High,
                enabled: true,
            },
        );

        // WhatsApp Web
        tasks.insert(
            "WhatsApp".to_string(),
            ContextTask {
                app_name: "WhatsApp".to_string(),
                search_targets: vec!["WhatsApp".to_string(), "WhatsApp Web".to_string()],
                url_patterns: vec!["web.whatsapp.com".to_string()],
                css_selector: r#"
                    (function() {
                        let content = [];
                        const messages = document.querySelectorAll('[data-testid="msg-container"]');
                        messages.forEach(msg => {
                            const text = msg.textContent?.trim();
                            if (text && text.length < 1000) content.push(text);
                        });
                        return content.join('\n') || 'No messages';
                    })()
                "#
                .to_string(),
                priority: TaskPriority::Normal,
                enabled: true,
            },
        );

        // Telegram Web
        tasks.insert(
            "Telegram".to_string(),
            ContextTask {
                app_name: "Telegram".to_string(),
                search_targets: vec!["Telegram".to_string(), "Telegram Web".to_string()],
                url_patterns: vec!["web.telegram.org".to_string()],
                css_selector: r#"
                    (function() {
                        let content = [];
                        const messages = document.querySelectorAll('.message-content');
                        messages.forEach(msg => {
                            const text = msg.textContent?.trim();
                            if (text) content.push(text);
                        });
                        return content.join('\n') || 'No messages';
                    })()
                "#
                .to_string(),
                priority: TaskPriority::Normal,
                enabled: true,
            },
        );

        // Google Chat
        tasks.insert(
            "Google Chat".to_string(),
            ContextTask {
                app_name: "Google Chat".to_string(),
                search_targets: vec!["Chat".to_string(), "Google Chat".to_string()],
                url_patterns: vec!["chat.google.com".to_string()],
                css_selector: r#"
                    (function() {
                        let content = [];
                        const messages = document.querySelectorAll('[data-message-id]');
                        messages.forEach(msg => {
                            const text = msg.textContent?.trim();
                            if (text) content.push(text);
                        });
                        return content.join('\n') || 'No messages';
                    })()
                "#
                .to_string(),
                priority: TaskPriority::Normal,
                enabled: true,
            },
        );

        tasks
    }

    /// Obtener contexto por nombre de app (exact match)
    pub fn get_context(&self, app_name: &str) -> Option<ContextTask> {
        self.tasks.get(app_name).cloned()
    }

    /// Buscar contexto por nombre aproximado (case-insensitive)
    pub fn find_context(&self, app_name: &str) -> Option<ContextTask> {
        let app_lower = app_name.to_lowercase();

        self.tasks
            .iter()
            .find(|(key, _)| key.to_lowercase().contains(&app_lower))
            .map(|(_, task)| task.clone())
    }

    /// Obtener todos los contextos habilitados
    pub fn get_enabled_contexts(&self) -> Vec<ContextTask> {
        self.tasks
            .values()
            .filter(|task| task.enabled)
            .cloned()
            .collect()
    }

    /// Obtener contextos por prioridad
    pub fn get_by_priority(&self, min_priority: TaskPriority) -> Vec<ContextTask> {
        self.tasks
            .values()
            .filter(|task| task.enabled && task.priority >= min_priority)
            .cloned()
            .collect()
    }

    /// Registrar una app personalizada
    pub fn register_app(
        &mut self,
        app_name: String,
        search_targets: Vec<String>,
        url_patterns: Vec<String>,
        css_selector: String,
        priority: TaskPriority,
    ) {
        let task = ContextTask {
            app_name: app_name.clone(),
            search_targets,
            url_patterns,
            css_selector,
            priority,
            enabled: true,
        };
        self.tasks.insert(app_name, task);
    }

    /// Habilitar o deshabilitar una app
    pub fn set_enabled(&mut self, app_name: &str, enabled: bool) {
        if let Some(task) = self.tasks.get_mut(app_name) {
            task.enabled = enabled;
        }
    }

    /// Cambiar prioridad de una app
    pub fn set_priority(&mut self, app_name: &str, priority: TaskPriority) {
        if let Some(task) = self.tasks.get_mut(app_name) {
            task.priority = priority;
        }
    }
}

impl Default for ContextMapper {
    fn default() -> Self {
        Self::new()
    }
}

/// Validador de contexto - determina si procesar una notificación
pub struct ContextValidator {
    mapper: ContextMapper,
}

impl ContextValidator {
    pub fn new() -> Self {
        Self {
            mapper: ContextMapper::new(),
        }
    }

    /// Validar si una notificación debe procesarse
    pub fn should_process_notification(
        &self,
        app_name: &str,
        min_priority: TaskPriority,
    ) -> bool {
        match self.mapper.find_context(app_name) {
            Some(task) => task.enabled && task.priority >= min_priority,
            None => false,
        }
    }

    /// Validar y obtener contexto con validación de prioridad
    pub fn validate_and_get_context(
        &self,
        app_name: &str,
    ) -> Result<ContextTask, String> {
        match self.mapper.find_context(app_name) {
            Some(task) if task.enabled => Ok(task),
            Some(_) => Err(format!(
                "Aplicación '{}' está deshabilitada para procesamiento CDP",
                app_name
            )),
            None => Err(format!(
                "No hay mapeo de contexto para la aplicación '{}'",
                app_name
            )),
        }
    }

    /// Obtener primer target de búsqueda sugerido
    pub fn get_first_search_target(&self, app_name: &str) -> Option<String> {
        self.mapper
            .find_context(app_name)
            .and_then(|task| task.search_targets.first().cloned())
    }

    /// Obtener todos los targets sugeridos
    pub fn get_all_search_targets(&self, app_name: &str) -> Vec<String> {
        self.mapper
            .find_context(app_name)
            .map(|task| task.search_targets)
            .unwrap_or_default()
    }
}

impl Default for ContextValidator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_context_mapper_creation() {
        let mapper = ContextMapper::new();
        assert!(mapper.get_context("Google Meet").is_some());
        assert!(mapper.get_context("Microsoft Teams").is_some());
        assert!(mapper.get_context("Discord").is_some());
    }

    #[test]
    fn test_find_context_case_insensitive() {
        let mapper = ContextMapper::new();
        assert!(mapper.find_context("google").is_some());
        assert!(mapper.find_context("TEAMS").is_some());
        assert!(mapper.find_context("discord").is_some());
    }

    #[test]
    fn test_get_enabled_contexts() {
        let mapper = ContextMapper::new();
        let enabled = mapper.get_enabled_contexts();
        assert!(!enabled.is_empty());
        assert!(enabled.iter().all(|t| t.enabled));
    }

    #[test]
    fn test_priority_filtering() {
        let mapper = ContextMapper::new();
        let high_priority = mapper.get_by_priority(TaskPriority::High);
        assert!(!high_priority.is_empty());
        assert!(high_priority.iter().all(|t| t.priority >= TaskPriority::High));
    }

    #[test]
    fn test_validator_should_process() {
        let validator = ContextValidator::new();
        assert!(validator.should_process_notification("Google Meet", TaskPriority::Normal));
        assert!(!validator.should_process_notification(
            "Unknown App",
            TaskPriority::Normal
        ));
    }

    #[test]
    fn test_validator_get_search_targets() {
        let validator = ContextValidator::new();
        let targets = validator.get_all_search_targets("Slack");
        assert!(!targets.is_empty());
        assert!(targets.contains(&"Slack".to_string()));
    }

    #[test]
    fn test_custom_app_registration() {
        let mut mapper = ContextMapper::new();
        mapper.register_app(
            "Custom App".to_string(),
            vec!["Custom".to_string()],
            vec!["custom.com".to_string()],
            "document.body.innerText".to_string(),
            TaskPriority::Low,
        );
        assert!(mapper.get_context("Custom App").is_some());
    }
}
