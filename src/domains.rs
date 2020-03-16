#[derive(Serialize, Deserialize, Clone)]
pub struct Item {
    pub domain: String,
    pub directive: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Collection {
    pub domains: Vec<Item>,
}

#[cfg(test)]
mod item_test {
    #[test]
    fn test_item_struct() {
        let directives: Vec<String> = vec![String::from("connect-src"), String::from("script-src")];

        let item = super::Item {
            domain: String::from("*.example.com"),
            directive: directives,
        };

        assert_eq!(item.domain, "*.example.com");
        assert_eq!(item.directive[1], "script-src");
    }

    #[test]
    fn test_collection() {
        let directives: Vec<String> = vec![String::from("connect-src"), String::from("script-src")];

        let item = super::Item {
            domain: String::from("*.example.com"),
            directive: directives,
        };

        let mut domains: Vec<super::Item> = vec![];
        domains.push(item);

        let collection = super::Collection{ domains };

        assert_eq!(collection.domains[0].domain, "*.example.com");
        assert_eq!(collection.domains[0].directive[1], "script-src");
    }
}
