pub mod prefix {
    use oxigraph::model::NamedNode;

    pub fn dock(suffix: &str) -> NamedNode {
        cat("https://dock.io/rdf/alpha/", suffix)
    }

    pub fn rdf(suffix: &str) -> NamedNode {
        cat("http://www.w3.org/1999/02/22-rdf-syntax-ns#", suffix)
    }

    pub fn rdfs(suffix: &str) -> NamedNode {
        cat("http://www.w3.org/2000/01/rdf-schema#", suffix)
    }

    fn cat(pre: &str, suff: &str) -> NamedNode {
        let ret = format!("{}{}", pre, suff);
        if ![
            "https://dock.io/rdf/alpha/claims",
            "https://dock.io/rdf/alpha/allowSubjects",
            "https://dock.io/rdf/alpha/allowPredicates",
            "https://dock.io/rdf/alpha/allowObjects",
            "https://dock.io/rdf/alpha/ANYTHING",
            "https://dock.io/rdf/alpha/mayClaim",
            "https://dock.io/rdf/alpha/dereferencesTo",
            "http://www.w3.org/1999/02/22-rdf-syntax-ns#subject",
            "http://www.w3.org/1999/02/22-rdf-syntax-ns#predicate",
            "http://www.w3.org/1999/02/22-rdf-syntax-ns#object",
            "http://www.w3.org/1999/02/22-rdf-syntax-ns#type",
            "http://www.w3.org/2000/01/rdf-schema#member",
        ]
        .contains(&ret.as_str())
        {
            panic!("{} is not in the allowlist", ret);
        }
        NamedNode::new(ret).unwrap()
    }
}
