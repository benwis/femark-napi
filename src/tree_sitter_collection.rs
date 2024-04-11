use tree_sitter_highlight::HighlightConfiguration;
pub struct TreeSitterCollection {
    pub conf: HighlightConfiguration,
}

impl TreeSitterCollection {
    pub fn rust() -> TreeSitterCollection {
        let rust_conf = HighlightConfiguration::new(
            tree_sitter_rstml::language_rust_with_rstml(),
            "rust",
            tree_sitter_rstml::HIGHLIGHTS_QUERY,
            "",
            "",
            true,
        )
        .unwrap();

        TreeSitterCollection { conf: rust_conf }
    }
    pub fn typescript() -> TreeSitterCollection {
        let mut highlights = tree_sitter_typescript::HIGHLIGHT_QUERY.to_owned();
        highlights.push_str(tree_sitter_javascript::HIGHLIGHT_QUERY);

        let mut locals = tree_sitter_typescript::LOCALS_QUERY.to_owned();
        locals.push_str(tree_sitter_javascript::LOCALS_QUERY);

        let conf = HighlightConfiguration::new(
            tree_sitter_typescript::language_typescript(),
            "typescript",
            &highlights,
            tree_sitter_javascript::INJECTION_QUERY,
            &locals,
            true,
        )
        .unwrap();

        TreeSitterCollection { conf }
    }

    pub fn tsx() -> TreeSitterCollection {
        let mut highlights = tree_sitter_javascript::JSX_HIGHLIGHT_QUERY.to_owned();
        highlights.push_str(tree_sitter_typescript::HIGHLIGHT_QUERY);
        highlights.push_str(tree_sitter_javascript::HIGHLIGHT_QUERY);

        let mut locals = tree_sitter_typescript::LOCALS_QUERY.to_owned();
        locals.push_str(tree_sitter_javascript::LOCALS_QUERY);

        let conf = HighlightConfiguration::new(
            tree_sitter_typescript::language_tsx(),
            "tsx",
            &highlights,
            tree_sitter_javascript::INJECTION_QUERY,
            &locals,
            true,
        )
        .unwrap();

        TreeSitterCollection { conf }
    }
    pub fn javascript() -> TreeSitterCollection {
        let conf = HighlightConfiguration::new(
            tree_sitter_javascript::language(),
            "javascript",
            tree_sitter_javascript::HIGHLIGHT_QUERY,
            tree_sitter_javascript::INJECTION_QUERY,
            tree_sitter_javascript::LOCALS_QUERY,
            true,
        )
        .unwrap();

        TreeSitterCollection { conf }
    }
    pub fn jsx() -> TreeSitterCollection {
        let conf = HighlightConfiguration::new(
            tree_sitter_javascript::language(),
            "jsx",
            tree_sitter_javascript::JSX_HIGHLIGHT_QUERY,
            tree_sitter_javascript::INJECTION_QUERY,
            "",
            true,
        )
        .unwrap();

        TreeSitterCollection { conf }
    }
    pub fn go() -> TreeSitterCollection {
        let conf = HighlightConfiguration::new(
            tree_sitter_go::language(),
            "go",
            tree_sitter_go::HIGHLIGHT_QUERY,
            "",
            "",
            true,
        )
        .unwrap();

        TreeSitterCollection { conf }
    }
    pub fn c() -> TreeSitterCollection {
        let conf = HighlightConfiguration::new(
            tree_sitter_c::language(),
            "c",
            tree_sitter_c::HIGHLIGHT_QUERY,
            "",
            "",
            true,
        )
        .unwrap();

        TreeSitterCollection { conf }
    }
    pub fn html() -> TreeSitterCollection {
        let conf = HighlightConfiguration::new(
            tree_sitter_html::language(),
            "html",
            tree_sitter_html::HIGHLIGHT_QUERY,
            tree_sitter_html::INJECTION_QUERY,
            "",
            true,
        )
        .unwrap();

        TreeSitterCollection { conf }
    }
    pub fn toml() -> TreeSitterCollection {
        let conf = HighlightConfiguration::new(
            tree_sitter_toml::language(),
            "toml",
            tree_sitter_toml::HIGHLIGHT_QUERY,
            "",
            "",
            true,
        )
        .unwrap();

        TreeSitterCollection { conf }
    }
    pub fn python() -> TreeSitterCollection {
        let conf = HighlightConfiguration::new(
            tree_sitter_python::language(),
            "python",
            tree_sitter_python::HIGHLIGHT_QUERY,
            "",
            "",
            true,
        )
        .unwrap();

        TreeSitterCollection { conf }
    }
    pub fn dockerfile() -> TreeSitterCollection {
        let conf = HighlightConfiguration::new(
            tree_sitter_dockerfile::language(),
            "dockerfile",
            "",
            "",
            "",
            true,
        )
        .unwrap();

        TreeSitterCollection { conf }
    }
    pub fn json() -> TreeSitterCollection {
        let conf = HighlightConfiguration::new(
            tree_sitter_json::language(),
            "json",
            tree_sitter_json::HIGHLIGHT_QUERY,
            "",
            "",
            true,
        )
        .unwrap();

        TreeSitterCollection { conf }
    }

    // pub fn nix() -> TreeSitterCollection {
    //   let lang = unsafe { tree_sitter_nix() };
    //   let conf = HighlightConfiguration::new(lang, "", "", "").unwrap();
    //   TreeSitterCollection { conf }
    // }
    pub fn nix() -> TreeSitterCollection {
        let conf = HighlightConfiguration::new(
            tree_sitter_nix::language(),
            "nix",
            tree_sitter_nix::HIGHLIGHTS_QUERY,
            "",
            "",
            true,
        )
        .unwrap();
        TreeSitterCollection { conf }
    }
}
