#[cfg(test)]
mod tests {
    use crate::project::{Project, ProjectSettings};
    use tempfile::TempDir;

    #[test]
    fn test_query_hover_on_variable() {
        let tmpdir = TempDir::new().unwrap();
        let source = r#"
pub fn example() {
    let x: i32 = 42;
    //  ^?
}
"#
        .trim();

        let settings = ProjectSettings {
            project_name: "test-project",
            tmpdir: &tmpdir,
            cargo_toml: None,
            target_dir: None,
        };

        let project = Project::scaffold_with_code(settings, source).unwrap();
        let result = project.twoslasher().unwrap();

        assert_eq!(result.queries.len(), 1);
        let query = &result.queries[0];
        
        assert!(query.text.is_some());
        let text = query.text.as_ref().unwrap();
        assert!(text.contains("i32"), "Expected hover to contain 'i32', got: {}", text);
    }

    #[test]
    fn test_query_hover_on_function() {
        let tmpdir = TempDir::new().unwrap();
        let source = r#"
fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn example() {
    let result = add(1, 2);
    //           ^?
}
"#
        .trim();

        let settings = ProjectSettings {
            project_name: "test-project",
            tmpdir: &tmpdir,
            cargo_toml: None,
            target_dir: None,
        };

        let project = Project::scaffold_with_code(settings, source).unwrap();
        let result = project.twoslasher().unwrap();

        assert_eq!(result.queries.len(), 1);
        let query = &result.queries[0];
        assert!(query.text.is_some());
        let text = query.text.as_ref().unwrap();
        assert!(text.contains("fn add"), "Expected hover to contain 'fn add', got: {}", text);
    }

    #[test]
    fn test_query_hover_on_type() {
        let tmpdir = TempDir::new().unwrap();
        let source = r#"
pub struct Point {
    x: f64,
    y: f64,
}

pub fn example() {
    let p = Point { x: 1.0, y: 2.0 };
    //  ^?
}
"#
        .trim();

        let settings = ProjectSettings {
            project_name: "test-project",
            tmpdir: &tmpdir,
            cargo_toml: None,
            target_dir: None,
        };

        let project = Project::scaffold_with_code(settings, source).unwrap();
        let result = project.twoslasher().unwrap();

        assert_eq!(result.queries.len(), 1);
        let query = &result.queries[0];
        assert!(query.text.is_some());
        let text = query.text.as_ref().unwrap();
        assert!(text.contains("Point"), "Expected hover to contain 'Point', got: {}", text);
    }

    #[test]
    fn test_multiple_queries() {
        let tmpdir = TempDir::new().unwrap();
        let source = r#"
pub fn example() {
    let x: i32 = 1;
    //  ^?
    let y: u64 = 2;
    //  ^?
}
"#
        .trim();

        let settings = ProjectSettings {
            project_name: "test-project",
            tmpdir: &tmpdir,
            cargo_toml: None,
            target_dir: None,
        };

        let project = Project::scaffold_with_code(settings, source).unwrap();
        let result = project.twoslasher().unwrap();

        assert_eq!(result.queries.len(), 2);
        
        let first = &result.queries[0];
        assert!(first.text.as_ref().unwrap().contains("i32"), 
            "Expected i32, got: {}", first.text.as_ref().unwrap());
        
        let second = &result.queries[1];
        assert!(second.text.as_ref().unwrap().contains("u64"),
            "Expected u64, got: {}", second.text.as_ref().unwrap());
    }

    #[test]
    fn test_query_position_info() {
        let tmpdir = TempDir::new().unwrap();
        let source = r#"
pub fn example() {
    let foo = 123;
    //  ^?
}
"#
        .trim();

        let settings = ProjectSettings {
            project_name: "test-project",
            tmpdir: &tmpdir,
            cargo_toml: None,
            target_dir: None,
        };

        let project = Project::scaffold_with_code(settings, source).unwrap();
        let result = project.twoslasher().unwrap();

        assert_eq!(result.queries.len(), 1);
        let query = &result.queries[0];
        
        assert_eq!(query.line, 2); // line 2 (0-indexed line 1 + 1)
        assert!(query.length > 0, "Expected length > 0");
    }

    #[test]
    fn test_static_quick_infos() {
        let tmpdir = TempDir::new().unwrap();
        let source = r#"
pub fn example() {
    let x: i32 = 42;
}
"#
        .trim();

        let settings = ProjectSettings {
            project_name: "test-project",
            tmpdir: &tmpdir,
            cargo_toml: None,
            target_dir: None,
        };

        let project = Project::scaffold_with_code(settings, source).unwrap();
        let result = project.twoslasher().unwrap();

        // Should have hover info for identifiers
        assert!(!result.static_quick_infos.is_empty(), "Expected some static quick infos");
    }

    #[test]
    fn test_completions_query() {
        let tmpdir = TempDir::new().unwrap();
        // The ^| marker should be positioned under the location where completions are desired
        // The caret points to the character position on the line above
        let source = r#"
pub struct Foo {
    pub bar: i32,
    pub baz: i32,
}

pub fn example() {
    let f = Foo { bar: 1, baz: 2 };
    f.bar
//      ^|
}
"#
        .trim();

        let settings = ProjectSettings {
            project_name: "test-project",
            tmpdir: &tmpdir,
            cargo_toml: None,
            target_dir: None,
        };

        let project = Project::scaffold_with_code(settings, source).unwrap();
        let result = project.twoslasher().unwrap();

        assert_eq!(result.queries.len(), 1);
        let query = &result.queries[0];
        
        // Completions query should have completions, not text
        assert!(query.completions.is_some(), "Expected completions");
        let completions = query.completions.as_ref().unwrap();
        assert!(!completions.is_empty(), "Expected at least one completion");
        
        // Should include field completions - bar and baz should be among the first suggestions
        let names: Vec<&str> = completions.iter().map(|c| c.name.as_str()).collect();
        assert!(names.contains(&"bar"), "Expected 'bar' in completions, got: {:?}", names);
        assert!(names.contains(&"baz"), "Expected 'baz' in completions, got: {:?}", names);
    }

    #[test]
    fn test_completions_on_method() {
        let tmpdir = TempDir::new().unwrap();
        let source = r#"
pub struct Counter {
    value: i32,
}

impl Counter {
    pub fn increment(&mut self) {
        self.value += 1;
    }
    pub fn get(&self) -> i32 {
        self.value
    }
}

pub fn example() {
    let mut c = Counter { value: 0 };
    c.get
//      ^|
}
"#
        .trim();

        let settings = ProjectSettings {
            project_name: "test-project",
            tmpdir: &tmpdir,
            cargo_toml: None,
            target_dir: None,
        };

        let project = Project::scaffold_with_code(settings, source).unwrap();
        let result = project.twoslasher().unwrap();

        assert_eq!(result.queries.len(), 1);
        let query = &result.queries[0];
        
        assert!(query.completions.is_some(), "Expected completions");
        let completions = query.completions.as_ref().unwrap();
        
        // Should include method completions
        let names: Vec<&str> = completions.iter().map(|c| c.name.as_str()).collect();
        assert!(names.contains(&"get"), "Expected 'get' in completions, got: {:?}", names);
        assert!(names.contains(&"increment"), "Expected 'increment' in completions, got: {:?}", names);
        assert!(names.contains(&"value"), "Expected 'value' (field) in completions, got: {:?}", names);
    }

    #[test]
    fn test_cut_removes_imports_but_keeps_types() {
        let tmpdir = TempDir::new().unwrap();
        let source = r#"
pub struct Config {
    pub name: String,
    pub value: i32,
}
// ---cut---
pub fn example() {
    let cfg = Config { name: String::new(), value: 42 };
    //  ^?
}
"#
        .trim();

        let settings = ProjectSettings {
            project_name: "test-project",
            tmpdir: &tmpdir,
            cargo_toml: None,
            target_dir: None,
        };

        let project = Project::scaffold_with_code(settings, source).unwrap();
        let result = project.twoslasher().unwrap();

        // The output code should NOT contain the struct definition (it's cut)
        assert!(!result.code.contains("pub struct Config"), 
            "Expected struct definition to be cut from output, got: {}", result.code);
        
        // But we should still get type information for Config
        assert_eq!(result.queries.len(), 1);
        let query = &result.queries[0];
        assert!(query.text.is_some(), "Expected hover text");
        let text = query.text.as_ref().unwrap();
        assert!(text.contains("Config"), "Expected hover to contain 'Config', got: {}", text);
    }
}
