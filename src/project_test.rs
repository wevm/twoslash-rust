#[cfg(test)]
mod tests {
    use crate::project::{Project, ProjectSettings};
    use crate::twoslash::TwoSlash;
    use insta::assert_snapshot;
    use tempfile::TempDir;

    fn twoslash(source: &str) -> TwoSlash {
        let tmpdir = TempDir::new().unwrap();
        let settings = ProjectSettings {
            project_name: "test-project",
            tmpdir: &tmpdir,
            cargo_toml: None,
            target_dir: None,
        };
        let project = Project::scaffold_with_code(settings, source.trim()).unwrap();
        project.twoslasher().unwrap()
    }

    fn snapshot(result: &TwoSlash) -> String {
        serde_json::to_string_pretty(result).unwrap()
    }

    #[test]
    fn test_query_hover_on_variable() {
        let result = twoslash(
            r#"
pub fn example() {
    let x: i32 = 42;
    //  ^?
}
"#,
        );

        assert_eq!(result.queries.len(), 1);
        let query = &result.queries[0];
        assert!(query.text.is_some());
        let text = query.text.as_ref().unwrap();
        assert!(
            text.contains("i32"),
            "Expected hover to contain 'i32', got: {}",
            text
        );

        assert_snapshot!(snapshot(&result), @r#"
        {
          "code": "pub fn example() {\n    let x: i32 = 42;\n}",
          "extension": ".rs",
          "highlights": [],
          "staticQuickInfos": [
            {
              "targetString": "pub fn example() {\n    let x: i32 = 42;\n}",
              "text": "extern crate test_project",
              "start": 0,
              "length": 41,
              "line": 0,
              "character": 0
            },
            {
              "targetString": "example",
              "text": "test_project\n\npub fn example()",
              "start": 7,
              "length": 7,
              "line": 0,
              "character": 7
            },
            {
              "targetString": "x",
              "text": "let x: i32",
              "start": 27,
              "length": 1,
              "line": 1,
              "character": 8
            },
            {
              "targetString": "i32",
              "text": "i32",
              "start": 30,
              "length": 3,
              "line": 1,
              "character": 11
            }
          ],
          "queries": [
            {
              "kind": "query",
              "line": 2,
              "offset": 8,
              "text": "let x: i32",
              "start": 27,
              "length": 1
            }
          ],
          "tags": [],
          "errors": [
            {
              "renderedMessage": "unused variable",
              "id": "unused_variables",
              "category": "Warning",
              "code": 0,
              "start": 27,
              "length": 1,
              "line": 1,
              "character": 8
            }
          ],
          "playgroundURL": "https://play.rust-lang.org"
        }
        "#);
    }

    #[test]
    fn test_query_hover_on_function() {
        let result = twoslash(
            r#"
fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn example() {
    let result = add(1, 2);
    //           ^?
}
"#,
        );

        assert_eq!(result.queries.len(), 1);
        let query = &result.queries[0];
        assert!(query.text.is_some());
        let text = query.text.as_ref().unwrap();
        assert!(
            text.contains("fn add"),
            "Expected hover to contain 'fn add', got: {}",
            text
        );

        assert_snapshot!(snapshot(&result), @r#"
        {
          "code": "fn add(a: i32, b: i32) -> i32 {\n    a + b\n}\n\npub fn example() {\n    let result = add(1, 2);\n}",
          "extension": ".rs",
          "highlights": [],
          "staticQuickInfos": [
            {
              "targetString": "fn add(a: i32, b: i32) -> i32 {\n    a + b\n}\n\npub fn example() {\n    let result = add(1, 2);\n}",
              "text": "extern crate test_project",
              "start": 0,
              "length": 93,
              "line": 0,
              "character": 0
            },
            {
              "targetString": "add",
              "text": "test_project\n\nfn add(a: i32, b: i32) -> i32",
              "start": 3,
              "length": 3,
              "line": 0,
              "character": 3
            },
            {
              "targetString": "add",
              "text": "test_project\n\nfn add(a: i32, b: i32) -> i32",
              "start": 81,
              "length": 3,
              "line": 5,
              "character": 17
            },
            {
              "targetString": "a",
              "text": "a: i32",
              "start": 7,
              "length": 1,
              "line": 0,
              "character": 7
            },
            {
              "targetString": "a",
              "text": "a: i32",
              "start": 36,
              "length": 1,
              "line": 1,
              "character": 4
            },
            {
              "targetString": "i32",
              "text": "i32",
              "start": 10,
              "length": 3,
              "line": 0,
              "character": 10
            },
            {
              "targetString": "i32",
              "text": "i32",
              "start": 18,
              "length": 3,
              "line": 0,
              "character": 18
            },
            {
              "targetString": "i32",
              "text": "i32",
              "start": 26,
              "length": 3,
              "line": 0,
              "character": 26
            },
            {
              "targetString": "b",
              "text": "b: i32",
              "start": 15,
              "length": 1,
              "line": 0,
              "character": 15
            },
            {
              "targetString": "b",
              "text": "b: i32",
              "start": 40,
              "length": 1,
              "line": 1,
              "character": 8
            },
            {
              "targetString": "example",
              "text": "test_project\n\npub fn example()",
              "start": 52,
              "length": 7,
              "line": 4,
              "character": 7
            },
            {
              "targetString": "result",
              "text": "let result: i32",
              "start": 72,
              "length": 6,
              "line": 5,
              "character": 8
            }
          ],
          "queries": [
            {
              "kind": "query",
              "line": 6,
              "offset": 17,
              "text": "test_project\n\nfn add(a: i32, b: i32) -> i32",
              "start": 81,
              "length": 3
            }
          ],
          "tags": [],
          "errors": [
            {
              "renderedMessage": "unused variable",
              "id": "unused_variables",
              "category": "Warning",
              "code": 0,
              "start": 72,
              "length": 6,
              "line": 5,
              "character": 8
            }
          ],
          "playgroundURL": "https://play.rust-lang.org"
        }
        "#);
    }

    #[test]
    fn test_query_hover_on_type() {
        let result = twoslash(
            r#"
pub struct Point {
    x: f64,
    y: f64,
}

pub fn example() {
    let p = Point { x: 1.0, y: 2.0 };
    //  ^?
}
"#,
        );

        assert_eq!(result.queries.len(), 1);
        let query = &result.queries[0];
        assert!(query.text.is_some());
        let text = query.text.as_ref().unwrap();
        assert!(
            text.contains("Point"),
            "Expected hover to contain 'Point', got: {}",
            text
        );

        assert_snapshot!(snapshot(&result), @r#"
        {
          "code": "pub struct Point {\n    x: f64,\n    y: f64,\n}\n\npub fn example() {\n    let p = Point { x: 1.0, y: 2.0 };\n}",
          "extension": ".rs",
          "highlights": [],
          "staticQuickInfos": [
            {
              "targetString": "pub struct Point {\n    x: f64,\n    y: f64,\n}\n\npub fn example() {\n    let p = Point { x: 1.0, y: 2.0 };\n}",
              "text": "extern crate test_project",
              "start": 0,
              "length": 104,
              "line": 0,
              "character": 0
            },
            {
              "targetString": "Point",
              "text": "test_project\n\npub struct Point {\n    x: f64,\n    y: f64,\n}",
              "start": 11,
              "length": 5,
              "line": 0,
              "character": 11
            },
            {
              "targetString": "Point",
              "text": "test_project\n\npub struct Point {\n    x: f64,\n    y: f64,\n}",
              "start": 77,
              "length": 5,
              "line": 6,
              "character": 12
            },
            {
              "targetString": "x",
              "text": "test_project::Point\n\nx: f64",
              "start": 23,
              "length": 1,
              "line": 1,
              "character": 4
            },
            {
              "targetString": "x",
              "text": "test_project::Point\n\nx: f64",
              "start": 85,
              "length": 1,
              "line": 6,
              "character": 20
            },
            {
              "targetString": "f64",
              "text": "f64",
              "start": 26,
              "length": 3,
              "line": 1,
              "character": 7
            },
            {
              "targetString": "f64",
              "text": "f64",
              "start": 38,
              "length": 3,
              "line": 2,
              "character": 7
            },
            {
              "targetString": "y",
              "text": "test_project::Point\n\ny: f64",
              "start": 35,
              "length": 1,
              "line": 2,
              "character": 4
            },
            {
              "targetString": "y",
              "text": "test_project::Point\n\ny: f64",
              "start": 93,
              "length": 1,
              "line": 6,
              "character": 28
            },
            {
              "targetString": "example",
              "text": "test_project\n\npub fn example()",
              "start": 53,
              "length": 7,
              "line": 5,
              "character": 7
            },
            {
              "targetString": "p",
              "text": "let p: Point",
              "start": 73,
              "length": 1,
              "line": 6,
              "character": 8
            }
          ],
          "queries": [
            {
              "kind": "query",
              "line": 7,
              "offset": 8,
              "text": "let p: Point",
              "start": 73,
              "length": 1
            }
          ],
          "tags": [],
          "errors": [
            {
              "renderedMessage": "unused variable",
              "id": "unused_variables",
              "category": "Warning",
              "code": 0,
              "start": 73,
              "length": 1,
              "line": 6,
              "character": 8
            }
          ],
          "playgroundURL": "https://play.rust-lang.org"
        }
        "#);
    }

    #[test]
    fn test_multiple_queries() {
        let result = twoslash(
            r#"
pub fn example() {
    let x: i32 = 1;
    //  ^?
    let y: u64 = 2;
    //  ^?
}
"#,
        );

        assert_eq!(result.queries.len(), 2);

        let first = &result.queries[0];
        assert!(
            first.text.as_ref().unwrap().contains("i32"),
            "Expected i32, got: {}",
            first.text.as_ref().unwrap()
        );

        let second = &result.queries[1];
        assert!(
            second.text.as_ref().unwrap().contains("u64"),
            "Expected u64, got: {}",
            second.text.as_ref().unwrap()
        );

        assert_snapshot!(snapshot(&result), @r#"
        {
          "code": "pub fn example() {\n    let x: i32 = 1;\n    let y: u64 = 2;\n}",
          "extension": ".rs",
          "highlights": [],
          "staticQuickInfos": [
            {
              "targetString": "pub fn example() {\n    let x: i32 = 1;\n    let y: u64 = 2;\n}",
              "text": "extern crate test_project",
              "start": 0,
              "length": 60,
              "line": 0,
              "character": 0
            },
            {
              "targetString": "example",
              "text": "test_project\n\npub fn example()",
              "start": 7,
              "length": 7,
              "line": 0,
              "character": 7
            },
            {
              "targetString": "x",
              "text": "let x: i32",
              "start": 27,
              "length": 1,
              "line": 1,
              "character": 8
            },
            {
              "targetString": "i32",
              "text": "i32",
              "start": 30,
              "length": 3,
              "line": 1,
              "character": 11
            },
            {
              "targetString": "y",
              "text": "let y: u64",
              "start": 47,
              "length": 1,
              "line": 2,
              "character": 8
            },
            {
              "targetString": "u64",
              "text": "u64",
              "start": 50,
              "length": 3,
              "line": 2,
              "character": 11
            }
          ],
          "queries": [
            {
              "kind": "query",
              "line": 2,
              "offset": 8,
              "text": "let x: i32",
              "start": 27,
              "length": 1
            },
            {
              "kind": "query",
              "line": 3,
              "offset": 8,
              "text": "let y: u64",
              "start": 47,
              "length": 1
            }
          ],
          "tags": [],
          "errors": [
            {
              "renderedMessage": "unused variable",
              "id": "unused_variables",
              "category": "Warning",
              "code": 0,
              "start": 27,
              "length": 1,
              "line": 1,
              "character": 8
            },
            {
              "renderedMessage": "unused variable",
              "id": "unused_variables",
              "category": "Warning",
              "code": 0,
              "start": 47,
              "length": 1,
              "line": 2,
              "character": 8
            }
          ],
          "playgroundURL": "https://play.rust-lang.org"
        }
        "#);
    }

    #[test]
    fn test_query_position_info() {
        let result = twoslash(
            r#"
pub fn example() {
    let foo = 123;
    //  ^?
}
"#,
        );

        assert_eq!(result.queries.len(), 1);
        let query = &result.queries[0];
        assert_eq!(query.line, 2);
        assert!(query.length > 0, "Expected length > 0");

        assert_snapshot!(snapshot(&result), @r#"
        {
          "code": "pub fn example() {\n    let foo = 123;\n}",
          "extension": ".rs",
          "highlights": [],
          "staticQuickInfos": [
            {
              "targetString": "pub fn example() {\n    let foo = 123;\n}",
              "text": "extern crate test_project",
              "start": 0,
              "length": 39,
              "line": 0,
              "character": 0
            },
            {
              "targetString": "example",
              "text": "test_project\n\npub fn example()",
              "start": 7,
              "length": 7,
              "line": 0,
              "character": 7
            },
            {
              "targetString": "foo",
              "text": "let foo: i32",
              "start": 27,
              "length": 3,
              "line": 1,
              "character": 8
            }
          ],
          "queries": [
            {
              "kind": "query",
              "line": 2,
              "offset": 8,
              "text": "let foo: i32",
              "start": 27,
              "length": 3
            }
          ],
          "tags": [],
          "errors": [
            {
              "renderedMessage": "unused variable",
              "id": "unused_variables",
              "category": "Warning",
              "code": 0,
              "start": 27,
              "length": 3,
              "line": 1,
              "character": 8
            }
          ],
          "playgroundURL": "https://play.rust-lang.org"
        }
        "#);
    }

    #[test]
    fn test_static_quick_infos() {
        let result = twoslash(
            r#"
pub fn example() {
    let x: i32 = 42;
}
"#,
        );

        assert!(
            !result.static_quick_infos.is_empty(),
            "Expected some static quick infos"
        );

        assert_snapshot!(snapshot(&result), @r#"
        {
          "code": "pub fn example() {\n    let x: i32 = 42;\n}",
          "extension": ".rs",
          "highlights": [],
          "staticQuickInfos": [
            {
              "targetString": "pub fn example() {\n    let x: i32 = 42;\n}",
              "text": "extern crate test_project",
              "start": 0,
              "length": 41,
              "line": 0,
              "character": 0
            },
            {
              "targetString": "example",
              "text": "test_project\n\npub fn example()",
              "start": 7,
              "length": 7,
              "line": 0,
              "character": 7
            },
            {
              "targetString": "x",
              "text": "let x: i32",
              "start": 27,
              "length": 1,
              "line": 1,
              "character": 8
            },
            {
              "targetString": "i32",
              "text": "i32",
              "start": 30,
              "length": 3,
              "line": 1,
              "character": 11
            }
          ],
          "queries": [],
          "tags": [],
          "errors": [
            {
              "renderedMessage": "unused variable",
              "id": "unused_variables",
              "category": "Warning",
              "code": 0,
              "start": 27,
              "length": 1,
              "line": 1,
              "character": 8
            }
          ],
          "playgroundURL": "https://play.rust-lang.org"
        }
        "#);
    }

    #[test]
    fn test_completions_query() {
        let result = twoslash(
            r#"
pub struct Foo {
    pub bar: i32,
    pub baz: i32,
}

pub fn example() {
    let f = Foo { bar: 1, baz: 2 };
    f.bar
//      ^|
}
"#,
        );

        assert_eq!(result.queries.len(), 1);
        let query = &result.queries[0];
        assert!(query.completions.is_some(), "Expected completions");
        let completions = query.completions.as_ref().unwrap();
        assert!(!completions.is_empty(), "Expected at least one completion");

        let names: Vec<&str> = completions.iter().map(|c| c.name.as_str()).collect();
        assert!(
            names.contains(&"bar"),
            "Expected 'bar' in completions, got: {:?}",
            names
        );
        assert!(
            names.contains(&"baz"),
            "Expected 'baz' in completions, got: {:?}",
            names
        );

        assert_snapshot!(snapshot(&result), @r#"
        {
          "code": "pub struct Foo {\n    pub bar: i32,\n    pub baz: i32,\n}\n\npub fn example() {\n    let f = Foo { bar: 1, baz: 2 };\n    f.bar\n}",
          "extension": ".rs",
          "highlights": [],
          "staticQuickInfos": [
            {
              "targetString": "pub struct Foo {\n    pub bar: i32,\n    pub baz: i32,\n}\n\npub fn example() {\n    let f = Foo { bar: 1, baz: 2 };\n    f.bar\n}",
              "text": "extern crate test_project",
              "start": 0,
              "length": 122,
              "line": 0,
              "character": 0
            },
            {
              "targetString": "Foo",
              "text": "test_project\n\npub struct Foo {\n    pub bar: i32,\n    pub baz: i32,\n}",
              "start": 11,
              "length": 3,
              "line": 0,
              "character": 11
            },
            {
              "targetString": "Foo",
              "text": "test_project\n\npub struct Foo {\n    pub bar: i32,\n    pub baz: i32,\n}",
              "start": 87,
              "length": 3,
              "line": 6,
              "character": 12
            },
            {
              "targetString": "bar",
              "text": "test_project::Foo\n\npub bar: i32",
              "start": 25,
              "length": 3,
              "line": 1,
              "character": 8
            },
            {
              "targetString": "bar",
              "text": "test_project::Foo\n\npub bar: i32",
              "start": 93,
              "length": 3,
              "line": 6,
              "character": 18
            },
            {
              "targetString": "bar",
              "text": "test_project::Foo\n\npub bar: i32",
              "start": 117,
              "length": 3,
              "line": 7,
              "character": 6
            },
            {
              "targetString": "i32",
              "text": "i32",
              "start": 30,
              "length": 3,
              "line": 1,
              "character": 13
            },
            {
              "targetString": "i32",
              "text": "i32",
              "start": 48,
              "length": 3,
              "line": 2,
              "character": 13
            },
            {
              "targetString": "baz",
              "text": "test_project::Foo\n\npub baz: i32",
              "start": 43,
              "length": 3,
              "line": 2,
              "character": 8
            },
            {
              "targetString": "baz",
              "text": "test_project::Foo\n\npub baz: i32",
              "start": 101,
              "length": 3,
              "line": 6,
              "character": 26
            },
            {
              "targetString": "example",
              "text": "test_project\n\npub fn example()",
              "start": 63,
              "length": 7,
              "line": 5,
              "character": 7
            },
            {
              "targetString": "f",
              "text": "let f: Foo",
              "start": 83,
              "length": 1,
              "line": 6,
              "character": 8
            },
            {
              "targetString": "f",
              "text": "let f: Foo",
              "start": 115,
              "length": 1,
              "line": 7,
              "character": 4
            }
          ],
          "queries": [
            {
              "kind": "query",
              "line": 7,
              "offset": 6,
              "start": 117,
              "length": 3,
              "completions": [
                {
                  "name": "bar"
                },
                {
                  "name": "baz"
                },
                {
                  "name": "ref"
                },
                {
                  "name": "refm"
                },
                {
                  "name": "deref"
                },
                {
                  "name": "box"
                },
                {
                  "name": "dbg"
                },
                {
                  "name": "dbgr"
                },
                {
                  "name": "call"
                },
                {
                  "name": "let"
                },
                {
                  "name": "letm"
                },
                {
                  "name": "match"
                },
                {
                  "name": "unsafe"
                },
                {
                  "name": "const"
                },
                {
                  "name": "return"
                }
              ],
              "completionsPrefix": "bar"
            }
          ],
          "tags": [],
          "errors": [
            {
              "renderedMessage": "expected (), found i32",
              "id": "E0308",
              "category": "Error",
              "code": 0,
              "start": 117,
              "length": 3,
              "line": 7,
              "character": 6
            }
          ],
          "playgroundURL": "https://play.rust-lang.org"
        }
        "#);
    }

    #[test]
    fn test_completions_on_method() {
        let result = twoslash(
            r#"
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
"#,
        );

        assert_eq!(result.queries.len(), 1);
        let query = &result.queries[0];
        assert!(query.completions.is_some(), "Expected completions");
        let completions = query.completions.as_ref().unwrap();

        let names: Vec<&str> = completions.iter().map(|c| c.name.as_str()).collect();
        assert!(
            names.contains(&"get"),
            "Expected 'get' in completions, got: {:?}",
            names
        );
        assert!(
            names.contains(&"increment"),
            "Expected 'increment' in completions, got: {:?}",
            names
        );
        assert!(
            names.contains(&"value"),
            "Expected 'value' (field) in completions, got: {:?}",
            names
        );

        assert_snapshot!(snapshot(&result), @r#"
        {
          "code": "pub struct Counter {\n    value: i32,\n}\n\nimpl Counter {\n    pub fn increment(&mut self) {\n        self.value += 1;\n    }\n    pub fn get(&self) -> i32 {\n        self.value\n    }\n}\n\npub fn example() {\n    let mut c = Counter { value: 0 };\n    c.get\n}",
          "extension": ".rs",
          "highlights": [],
          "staticQuickInfos": [
            {
              "targetString": "pub struct Counter {\n    value: i32,\n}\n\nimpl Counter {\n    pub fn increment(&mut self) {\n        self.value += 1;\n    }\n    pub fn get(&self) -> i32 {\n        self.value\n    }\n}\n\npub fn example() {\n    let mut c = Counter { value: 0 };\n    c.get\n}",
              "text": "extern crate test_project",
              "start": 0,
              "length": 247,
              "line": 0,
              "character": 0
            },
            {
              "targetString": "Counter",
              "text": "test_project\n\npub struct Counter {\n    value: i32,\n}",
              "start": 11,
              "length": 7,
              "line": 0,
              "character": 11
            },
            {
              "targetString": "Counter",
              "text": "test_project\n\npub struct Counter {\n    value: i32,\n}",
              "start": 45,
              "length": 7,
              "line": 4,
              "character": 5
            },
            {
              "targetString": "Counter",
              "text": "test_project\n\npub struct Counter {\n    value: i32,\n}",
              "start": 214,
              "length": 7,
              "line": 14,
              "character": 16
            },
            {
              "targetString": "value",
              "text": "test_project::Counter\n\nvalue: i32",
              "start": 25,
              "length": 5,
              "line": 1,
              "character": 4
            },
            {
              "targetString": "value",
              "text": "test_project::Counter\n\nvalue: i32",
              "start": 102,
              "length": 5,
              "line": 6,
              "character": 13
            },
            {
              "targetString": "value",
              "text": "test_project::Counter\n\nvalue: i32",
              "start": 164,
              "length": 5,
              "line": 9,
              "character": 13
            },
            {
              "targetString": "value",
              "text": "test_project::Counter\n\nvalue: i32",
              "start": 224,
              "length": 5,
              "line": 14,
              "character": 26
            },
            {
              "targetString": "i32",
              "text": "i32",
              "start": 32,
              "length": 3,
              "line": 1,
              "character": 11
            },
            {
              "targetString": "i32",
              "text": "i32",
              "start": 145,
              "length": 3,
              "line": 8,
              "character": 25
            },
            {
              "targetString": "increment",
              "text": "test_project::Counter\n\npub fn increment(&mut self)",
              "start": 66,
              "length": 9,
              "line": 5,
              "character": 11
            },
            {
              "targetString": "self",
              "text": "self: &mut Counter",
              "start": 81,
              "length": 4,
              "line": 5,
              "character": 26
            },
            {
              "targetString": "self",
              "text": "self: &mut Counter",
              "start": 97,
              "length": 4,
              "line": 6,
              "character": 8
            },
            {
              "targetString": "get",
              "text": "test_project::Counter\n\npub fn get(&self) -> i32",
              "start": 131,
              "length": 3,
              "line": 8,
              "character": 11
            },
            {
              "targetString": "get",
              "text": "test_project::Counter\n\npub fn get(&self) -> i32",
              "start": 242,
              "length": 3,
              "line": 15,
              "character": 6
            },
            {
              "targetString": "self",
              "text": "self: &Counter",
              "start": 136,
              "length": 4,
              "line": 8,
              "character": 16
            },
            {
              "targetString": "self",
              "text": "self: &Counter",
              "start": 159,
              "length": 4,
              "line": 9,
              "character": 8
            },
            {
              "targetString": "example",
              "text": "test_project\n\npub fn example()",
              "start": 186,
              "length": 7,
              "line": 13,
              "character": 7
            },
            {
              "targetString": "c",
              "text": "let mut c: Counter",
              "start": 210,
              "length": 1,
              "line": 14,
              "character": 12
            },
            {
              "targetString": "c",
              "text": "let mut c: Counter",
              "start": 240,
              "length": 1,
              "line": 15,
              "character": 4
            }
          ],
          "queries": [
            {
              "kind": "query",
              "line": 15,
              "offset": 6,
              "start": 242,
              "length": 3,
              "completions": [
                {
                  "name": "value"
                },
                {
                  "name": "get"
                },
                {
                  "name": "increment"
                },
                {
                  "name": "ref"
                },
                {
                  "name": "refm"
                },
                {
                  "name": "deref"
                },
                {
                  "name": "box"
                },
                {
                  "name": "dbg"
                },
                {
                  "name": "dbgr"
                },
                {
                  "name": "call"
                },
                {
                  "name": "let"
                },
                {
                  "name": "letm"
                },
                {
                  "name": "match"
                },
                {
                  "name": "unsafe"
                },
                {
                  "name": "const"
                },
                {
                  "name": "return"
                }
              ],
              "completionsPrefix": "get"
            }
          ],
          "tags": [],
          "errors": [
            {
              "renderedMessage": "no field `get` on type `Counter`, but a method with a similar name exists",
              "id": "E0559",
              "category": "Error",
              "code": 0,
              "start": 242,
              "length": 3,
              "line": 15,
              "character": 6
            },
            {
              "renderedMessage": "expected (), found i32",
              "id": "E0308",
              "category": "Error",
              "code": 0,
              "start": 242,
              "length": 3,
              "line": 15,
              "character": 6
            },
            {
              "renderedMessage": "cannot move `i32` out of reference",
              "id": "E0507",
              "category": "Error",
              "code": 0,
              "start": 159,
              "length": 10,
              "line": 9,
              "character": 8
            }
          ],
          "playgroundURL": "https://play.rust-lang.org"
        }
        "#);
    }

    #[test]
    fn test_no_errors_suppresses_diagnostics() {
        let result = twoslash(
            r#"
// @noErrors
fn main() {
    let x = 42;
    let message = "Hello, Rust!";
}
"#,
        );

        assert!(
            result.errors.is_empty(),
            "Expected no errors with @noErrors, got: {:?}",
            result
                .errors
                .iter()
                .map(|e| &e.rendered_message)
                .collect::<Vec<_>>()
        );

        assert_snapshot!(snapshot(&result), @r#"
        {
          "code": "fn main() {\n    let x = 42;\n    let message = \"Hello, Rust!\";\n}",
          "extension": ".rs",
          "highlights": [],
          "staticQuickInfos": [
            {
              "targetString": "fn main() {\n    let x = 42;\n    let message = \"Hello, Rust!\";\n}",
              "text": "extern crate test_project",
              "start": 0,
              "length": 63,
              "line": 0,
              "character": 0
            },
            {
              "targetString": "main",
              "text": "test_project\n\nfn main()",
              "start": 3,
              "length": 4,
              "line": 0,
              "character": 3
            },
            {
              "targetString": "x",
              "text": "let x: i32",
              "start": 20,
              "length": 1,
              "line": 1,
              "character": 8
            },
            {
              "targetString": "message",
              "text": "let message: &'static str",
              "start": 36,
              "length": 7,
              "line": 2,
              "character": 8
            }
          ],
          "queries": [],
          "tags": [],
          "errors": [],
          "playgroundURL": "https://play.rust-lang.org"
        }
        "#);
    }

    #[test]
    fn test_cut_removes_imports_but_keeps_types() {
        let result = twoslash(
            r#"
pub struct Config {
    pub name: String,
    pub value: i32,
}
// ---cut---
pub fn example() {
    let cfg = Config { name: String::new(), value: 42 };
    //  ^?
}
"#,
        );

        assert!(
            !result.code.contains("pub struct Config"),
            "Expected struct definition to be cut from output, got: {}",
            result.code
        );

        assert_eq!(result.queries.len(), 1);
        let query = &result.queries[0];
        assert!(query.text.is_some(), "Expected hover text");
        let text = query.text.as_ref().unwrap();
        assert!(
            text.contains("Config"),
            "Expected hover to contain 'Config', got: {}",
            text
        );

        assert_snapshot!(snapshot(&result), @r#"
        {
          "code": "pub fn example() {\n    let cfg = Config { name: String::new(), value: 42 };\n}",
          "extension": ".rs",
          "highlights": [],
          "staticQuickInfos": [
            {
              "targetString": "Config",
              "text": "test_project\n\npub struct Config {\n    pub name: {unknown},\n    pub value: i32,\n}",
              "start": 33,
              "length": 6,
              "line": 1,
              "character": 14
            },
            {
              "targetString": "name",
              "text": "test_project::Config\n\npub name: {unknown}",
              "start": 42,
              "length": 4,
              "line": 1,
              "character": 23
            },
            {
              "targetString": "value",
              "text": "test_project::Config\n\npub value: i32",
              "start": 63,
              "length": 5,
              "line": 1,
              "character": 44
            },
            {
              "targetString": "example",
              "text": "test_project\n\npub fn example()",
              "start": 7,
              "length": 7,
              "line": 0,
              "character": 7
            },
            {
              "targetString": "cfg",
              "text": "let cfg: Config",
              "start": 27,
              "length": 3,
              "line": 1,
              "character": 8
            }
          ],
          "queries": [
            {
              "kind": "query",
              "line": 2,
              "offset": 8,
              "text": "let cfg: Config",
              "start": 27,
              "length": 3
            }
          ],
          "tags": [],
          "errors": [],
          "playgroundURL": "https://play.rust-lang.org"
        }
        "#);
    }
}
