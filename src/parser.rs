use regex::Regex;
use serde_json::{json, Value};
use std::collections::HashMap;
use anyhow::{Result, anyhow};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
enum Action {
    Modern {
        command: String,
        #[serde(default)]
        params: Vec<String>,
        parse_rules: ParseRules,
        #[serde(default)]
        pre_exec: Vec<String>,
        #[serde(default)]
        post_exec: Vec<String>,
    },
    Legacy {
        command: String,
        #[serde(default)]
        params: Vec<String>,
        #[serde(default)]
        success_exit_code: Option<i32>,
        parse_output: Option<String>,
        #[serde(default)]
        pre_exec: Vec<String>,
        #[serde(default)]
        post_exec: Vec<String>,
    }
}

#[derive(Debug, Deserialize, Clone)]
struct ParseRules {
    #[serde(rename = "type")]
    parse_type: String,
    separator: Option<String>,
    patterns: Option<HashMap<String, PatternRule>>,
    property_pattern: Option<String>,
    array_key: Option<String>,
    related_patterns: Option<HashMap<String, RelatedPattern>>,
    mappings: Option<HashMap<String, MappingRule>>,
    array_patterns: Option<HashMap<String, ArrayPattern>>,
    format_type: Option<String>,  // for table parsing
    transformers: Option<HashMap<String, String>>,  // for table parsing
}

#[derive(Debug, Deserialize, Clone,PartialEq, Eq,)]
struct PatternRule {
    regex: String,
    group: Option<usize>,
    transform: Option<String>,
    multi_match: Option<bool>,
    object: Option<HashMap<String, ObjectRule>>,
    optional: Option<bool>,
}

#[derive(Debug, Deserialize, Clone,PartialEq, Eq, PartialOrd, Ord, Hash)]
struct RelatedPattern {
    pattern: String,
    group: Option<usize>,
    match_value: Option<String>,
    transform: Option<String>,
    optional: Option<bool>,
}

#[derive(Debug, Deserialize, Clone,PartialEq, Eq, PartialOrd, Ord, Hash)]
struct MappingRule {
    #[serde(rename = "type")]
    mapping_type: Option<String>,
    key: Option<String>,
    transform: Option<String>,
    separator: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
struct ArrayPattern {
    key_pattern: String,
    fields: HashMap<String, FieldRule>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
enum FieldRule {
    Simple(String),
    Complex {
        key: Option<String>,
        group: Option<usize>,
        transform: Option<String>,
        optional: Option<bool>,
    },
}

#[derive(Debug, Deserialize, Clone,PartialEq, Eq, PartialOrd, Ord, Hash)]
struct ObjectRule {
    group: usize,
    transform: Option<String>,
    regex: Option<String>,
    optional: Option<bool>,
}

pub struct OutputParser {
    rules: Option<ParseRules>,
    table_formats: HashMap<String, TableFormat>,
}

impl OutputParser {
    pub fn new(action: &Action, table_formats: HashMap<String, TableFormat>) -> Self {
        let rules = match action {
            Action::Modern { parse_rules, .. } => Some(parse_rules.clone()),
            Action::Legacy { .. } => None,
        };
        Self { rules, table_formats }
    }

    pub fn parse(&self, output: &str, action: &Action) -> Result<Value> {
        if let Some(rules) = &self.rules {
            match rules.parse_type.as_str() {
                "object" => self.parse_object(output),
                "array" => self.parse_array(output),
                "properties" => self.parse_properties(output),
                "table" => self.parse_table(output),
                _ => Err(anyhow!("Unsupported parse type")),
            }
        } else {
            match action {
                Action::Legacy { success_exit_code, parse_output, .. } => {
                    if success_exit_code.is_some() {
                        Ok(json!({
                            "error": "Legacy CPI action does not support output parsing via API. Success/failure is determined by exit code only.",
                            "exit_code_required": success_exit_code
                        }))
                    } else if parse_output.is_some() {
                        Ok(json!({
                            "error": "Legacy CPI action uses grep-based output parsing which is not supported via API. Please update to new parse_rules format.",
                            "grep_command": parse_output
                        }))
                    } else {
                        Ok(json!({
                            "error": "Legacy CPI action does not support output parsing via API.",
                            "raw_output": output
                        }))
                    }
                },
                _ => unreachable!()
            }
        }
    }

    fn parse_properties(&self, output: &str) -> Result<Value> {
        let mut result = json!({});
        let rules = self.rules.as_ref().unwrap();
        
        let property_pattern = rules.property_pattern.as_deref()
            .unwrap_or("^([^=]+)=\"(.*)\"$");
        let regex = Regex::new(property_pattern)?;

        let mut properties = HashMap::new();
        for line in output.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            if let Some(captures) = regex.captures(line) {
                if captures.len() >= 3 {
                    let key = captures[1].to_string();
                    let value = captures[2].to_string();
                    properties.insert(key, value);
                }
            }
        }

        // Handle array patterns
        if let Some(array_patterns) = &rules.array_patterns {
            for (field, pattern) in array_patterns {
                let key_regex = Regex::new(&pattern.key_pattern)?;
                let mut items = Vec::new();

                for (key, value) in &properties {
                    if let Some(captures) = key_regex.captures(key) {
                        let mut item = json!({});
                        for (field_name, field_rule) in &pattern.fields {
                            match field_rule {
                                FieldRule::Simple(value_type) => {
                                    if value_type == "value" {
                                        item[field_name] = json!(value);
                                    }
                                },
                                FieldRule::Complex { key, group, transform, optional } => {
                                    if let Some(field_key) = key {
                                        let expanded_key = field_key.replace("\\1", &captures[1]);
                                        if let Some(field_value) = properties.get(&expanded_key) {
                                            item[field_name] = self.transform_value(field_value, transform)?;
                                        } else if optional.unwrap_or(false) {
                                            item[field_name] = json!(null);
                                        }
                                    } else if let Some(g) = group {
                                        if let Some(field_value) = captures.get(*g) {
                                            item[field_name] = self.transform_value(field_value.as_str(), transform)?;
                                        }
                                    }
                                }
                            }
                        }
                        items.push(item);
                    }
                }
                result[field] = json!(items);
            }
        }

        // Handle mappings
        if let Some(mappings) = &rules.mappings {
            for (field, mapping) in mappings {
                if let Some(key) = &mapping.key {
                    if let Some(value) = properties.get(key) {
                        result[field] = self.transform_value(value, &mapping.transform)?;
                    }
                }
            }
        }

        Ok(result)
    }

    fn parse_object(&self, output: &str) -> Result<Value> {
        let mut result = json!({});

        if let Some(patterns) = &self.rules.as_ref().unwrap().patterns {
            for (field, pattern) in patterns {
                let regex = Regex::new(&pattern.regex)?;
                
                if pattern.multi_match.unwrap_or(false) {
                    let mut array = Vec::new();
                    for captures in regex.captures_iter(output) {
                        if let Some(obj_rules) = &pattern.object {
                            let mut obj = json!({});
                            for (key, rule) in obj_rules {
                                if let Some(value) = captures.get(rule.group) {
                                    obj[key] = self.transform_value(value.as_str(), &rule.transform)?;
                                } else if rule.optional.unwrap_or(false) {
                                    obj[key] = json!(null);
                                } else {
                                    return Err(anyhow!("Required group not found for field {}", key));
                                }
                            }
                            array.push(obj);
                        }
                    }
                    result[field] = json!(array);
                } else {
                    if let Some(captures) = regex.captures(output) {
                        let group = pattern.group.unwrap_or(1);
                        if let Some(value) = captures.get(group) {
                            result[field] = self.transform_value(value.as_str(), &pattern.transform)?;
                        } else if pattern.optional.unwrap_or(false) {
                            result[field] = json!(null);
                        } else {
                            return Err(anyhow!("Required group not found for field {}", field));
                        }
                    }
                }
            }
        }

        Ok(result)
    }

    fn parse_array(&self, output: &str) -> Result<Value> {
        let separator = self.rules.as_ref().unwrap().separator.as_deref().unwrap_or("\n\n");
        let items = output.split(separator)
            .filter(|s| !s.trim().is_empty())
            .map(|item| self.parse_object(item))
            .collect::<Result<Vec<_>>>()?;
        
        Ok(json!(items))
    }

    fn parse_table(&self, output: &str) -> Result<Value> {
        let rules = self.rules.as_ref().unwrap();
        let format_type = rules.format_type.as_ref()
            .ok_or_else(|| anyhow!("Format type required for table parsing"))?;
        
        let format = self.table_formats.get(format_type)
            .ok_or_else(|| anyhow!("Table format '{}' not found", format_type))?;

        let mut result = Vec::new();
        for line in output.lines().skip(format.skip_lines) {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            let fields: Vec<&str> = line.split(&format.delimiter)
                .map(str::trim)
                .collect();

            if fields.len() == format.headers.len() {
                let mut row = json!({});
                for (i, field) in fields.iter().enumerate() {
                    let header = &format.headers[i];
                    if let Some(transformer) = rules.transformers.as_ref().and_then(|t| t.get(header)) {
                        row[header] = self.transform_value(field, &Some(transformer.clone()))?;
                    } else {
                        row[header] = json!(field.to_string());
                    }
                }
                result.push(row);
            }
        }

        Ok(json!(result))
    }

    fn transform_value(&self, value: &str, transform: &Option<String>) -> Result<Value> {
        match transform.as_deref() {
            Some("number") => Ok(json!(value.parse::<f64>()?)),
            Some("boolean") => Ok(json!(value.parse::<bool>()?)),
            Some("array") => {
                let items: Vec<String> = value.split(',')
                    .map(str::trim)
                    .filter(|s| !s.is_empty())
                    .map(String::from)
                    .collect();
                Ok(json!(items))
            }
            Some("mac") => Ok(json!(value.to_lowercase())),
            _ => Ok(json!(value.to_string())),
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
struct TableFormat {
    headers: Vec<String>,
    delimiter: String,
    skip_lines: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_legacy_cpi_success_exit() -> Result<()> {
        let action = Action::Legacy {
            command: "VBoxManage showvminfo test-vm".to_string(),
            params: vec!["test-vm".to_string()],
            success_exit_code: Some(0),
            parse_output: None,
            pre_exec: vec![],
            post_exec: vec![],
        };

        let table_formats = HashMap::new();
        let parser = OutputParser::new(&action, table_formats);
        let result = parser.parse("Some output", &action)?;

        assert_eq!(
            result["error"],
            "Legacy CPI action does not support output parsing via API. Success/failure is determined by exit code only."
        );
        assert_eq!(result["exit_code_required"], 0);
        Ok(())
    }

    #[test]
    fn test_parse_virtualbox_vm() -> Result<()> {
        let output = r#"
            Name:            test-vm
            UUID:           1234-5678
            Memory size:    2048MB
            Number of CPUs: 2
            State:          running
            NIC 1:         NAT
            NIC 2:         bridged
            SATA (0, 0):   /path/to/disk1.vdi
            SATA (0, 1):   /path/to/disk2.vdi
        "#;

        let action = Action::Modern {
            command: "command".to_string(),
            params: vec![],
            parse_rules: ParseRules {
                parse_type: "object".to_string(),
                separator: None,
                patterns: Some(HashMap::from([
                    ("name".to_string(), PatternRule {
                        regex: r"Name:\s+(.+)".to_string(),
                        group: Some(1),
                        transform: None,
                        multi_match: None,
                        object: None,
                        optional: None,
                    }),
                    ("id".to_string(), PatternRule {
                        regex: r"UUID:\s+(.+)".to_string(),
                        group: Some(1),
                        transform: None,
                        multi_match: None,
                        object: None,
                        optional: None,
                    }),
                    ("memory_mb".to_string(), PatternRule {
                        regex: r"Memory size:\s+(\d+)MB".to_string(),
                        group: Some(1),
                        transform: Some("number".to_string()),
                        multi_match: None,
                        object: None,
                        optional: None,
                    }),
                    ("cpu_count".to_string(), PatternRule {
                        regex: r"Number of CPUs:\s+(\d+)".to_string(),
                        group: Some(1),
                        transform: Some("number".to_string()),
                        multi_match: None,
                        object: None,
                        optional: None,
                    }),
                    ("state".to_string(), PatternRule {
                        regex: r"State:\s+(.+)".to_string(),
                        group: Some(1),
                        transform: None,
                        multi_match: None,
                        object: None,
                        optional: None,
                    }),
                    ("networks".to_string(), PatternRule {
                        regex: r"NIC (\d+):\s+(.+)".to_string(),
                        group: None,
                        transform: None,
                        multi_match: Some(true),
                        object: Some(HashMap::from([
                            ("index".to_string(), ObjectRule {
                                group: 1,
                                transform: Some("number".to_string()),
                                regex: None,
                                optional: None,
                            }),
                            ("type".to_string(), ObjectRule {
                                group: 2,
                                transform: None,
                                regex: None,
                                optional: None,
                            }),
                        ])),
                        optional: None,
                    }),
                ])),
                property_pattern: None,
                array_key: None,
                related_patterns: None,
                mappings: None,
                array_patterns: None,
                format_type: None,
                transformers: None,
            },
            pre_exec: vec![],
            post_exec: vec![],
        };

        let table_formats = HashMap::new();
        let parser = OutputParser::new(&action, table_formats);
        let result = parser.parse(output, &action)?;

        assert_eq!(result["name"], "test-vm");
        assert_eq!(result["id"], "1234-5678");
        assert_eq!(result["memory_mb"], 2048.0);
        assert_eq!(result["cpu_count"], 2.0);
        assert_eq!(result["state"], "running");
        
        let networks = result["networks"].as_array().unwrap();
        assert_eq!(networks.len(), 2);
        assert_eq!(networks[0]["index"], 1.0);
        assert_eq!(networks[0]["type"], "NAT");
        assert_eq!(networks[1]["index"], 2.0);
        assert_eq!(networks[1]["type"], "bridged");

        Ok(())
    }

    #[test]
    fn test_parse_blocks() -> Result<()> {
        let output = r#"
UUID: disk1
Location: /path/to/disk1.vdi
Capacity: 1024 MBytes
Format: VDI

UUID: disk2
Location: /path/to/disk2.vdi
Capacity: 2048 MBytes
Format: VDI
Parent UUID: parent-uuid
"#;

        let action = Action::Modern {
            command: "command".to_string(),
            params: vec![],
            parse_rules: ParseRules {
                parse_type: "array".to_string(),
                separator: Some("\n\n".to_string()),
                patterns: Some(HashMap::from([
                    ("id".to_string(), PatternRule {
                        regex: r"UUID:\s+(.+)".to_string(),
                        group: Some(1),
                        transform: None,
                        multi_match: None,
                        object: None,
                        optional: None,
                    }),
                    ("path".to_string(), PatternRule {
                        regex: r"Location:\s+(.+)".to_string(),
                        group: Some(1),
                        transform: None,
                        multi_match: None,
                        object: None,
                        optional: None,
                    }),
                    ("size_mb".to_string(), PatternRule {
                        regex: r"Capacity:\s+(\d+) MBytes".to_string(),
                        group: Some(1),
                        transform: Some("number".to_string()),
                        multi_match: None,
                        object: None,
                        optional: None,
                    }),
                    ("format".to_string(), PatternRule {
                        regex: r"Format:\s+(.+)".to_string(),
                        group: Some(1),
                        transform: None,
                        multi_match: None,
                        object: None,
                        optional: None,
                    }),
                    ("parent".to_string(), PatternRule {
                        regex: r"Parent UUID:\s+(.+)".to_string(),
                        group: Some(1),
                        transform: None,
                        multi_match: None,
                        object: None,
                        optional: Some(true),
                    }),
                ])),
                property_pattern: None,
                array_key: None,
                related_patterns: None,
                mappings: None,
                array_patterns: None,
                format_type: None,
                transformers: None,
            },
            pre_exec: vec![],
            post_exec: vec![],
        };

        let table_formats = HashMap::new();
        let parser = OutputParser::new(&action, table_formats);
        let result = parser.parse(output, &action)?;

        let disks = result.as_array().unwrap();
        assert_eq!(disks.len(), 2);
        
        assert_eq!(disks[0]["id"], "disk1");
        assert_eq!(disks[0]["path"], "/path/to/disk1.vdi");
        assert_eq!(disks[0]["size_mb"], 1024.0);
        assert_eq!(disks[0]["format"], "VDI");
        assert!(disks[0]["parent"].is_null());

        assert_eq!(disks[1]["id"], "disk2");
        assert_eq!(disks[1]["path"], "/path/to/disk2.vdi");
        assert_eq!(disks[1]["size_mb"], 2048.0);
        assert_eq!(disks[1]["format"], "VDI");
        assert_eq!(disks[1]["parent"], "parent-uuid");

        Ok(())
    }

    #[test]
    fn test_legacy_cpi_grep() -> Result<()> {
        let action = Action::Legacy {
            command: "VBoxManage list hdds".to_string(),
            params: vec![],
            success_exit_code: None,
            parse_output: Some("grep -A 3 'UUID:'".to_string()),
            pre_exec: vec![],
            post_exec: vec![],
        };

        let table_formats = HashMap::new();
        let parser = OutputParser::new(&action, table_formats);
        let result = parser.parse("Some output", &action)?;

        assert_eq!(
            result["error"],
            "Legacy CPI action uses grep-based output parsing which is not supported via API. Please update to new parse_rules format."
        );
        assert_eq!(result["grep_command"], "grep -A 3 'UUID:'");
        Ok(())
    }

    #[test]
    fn test_legacy_cpi_no_parsing() -> Result<()> {
        let action = Action::Legacy {
            command: "VBoxManage createvm".to_string(),
            params: vec![],
            success_exit_code: None,
            parse_output: None,
            pre_exec: vec![],
            post_exec: vec![],
        };

        let table_formats = HashMap::new();
        let parser = OutputParser::new(&action, table_formats);
        let result = parser.parse("VM Created", &action)?;

        assert_eq!(
            result["error"],
            "Legacy CPI action does not support output parsing via API."
        );
        assert_eq!(result["raw_output"], "VM Created");
        Ok(())
    }
}