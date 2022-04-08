struct Foo {
    value: Option<Bar>
}

impl Foo {
    fn new(value: Option<Bar>) -> Self {
        Foo { value }
    }

    fn to_bar(&self) -> Result<&Bar, &'static str> {
        match &self.value {
            Some(value) => Ok(value),
            None => Err("Foo didn't have a value")
        }
    }
}

struct Bar {
    value: String
}

impl Bar {
    fn new(value: String) -> Self {
        Bar { value }
    }

    fn to_value(&self) -> Result<String, &'static str> {
        match self.value.as_str() {
            "bar" => Ok(String::from(&self.value)),
            _ => Err("Value was not bar")
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::experiments::result_mapping::{Foo, Bar};

    #[test]
    fn all_good() {
        let test_vec = vec![
            Foo::new(Some(Bar::new("bar".into()))),
            Foo::new(Some(Bar::new("bar".into()))),
            Foo::new(Some(Bar::new("bar".into()))),
            Foo::new(Some(Bar::new("bar".into())))
        ];
    
        let result = test_vec.iter()
            .map(|i| i.to_bar())
            .map(|i| i?.to_value())
            .collect::<Result<Vec<_>, _>>();

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), vec![
            "bar".to_string(),
            "bar".to_string(),
            "bar".to_string(),
            "bar".to_string()
        ]);
    }

    #[test]
    fn foo_no_value() {
        let test_vec = vec![
            Foo::new(Some(Bar::new("bar".into()))),
            Foo::new(None),
            Foo::new(Some(Bar::new("bar".into()))),
            Foo::new(Some(Bar::new("bar".into())))
        ];
    
        let result = test_vec.iter()
            .map(|i| i.to_bar())
            .map(|i| i?.to_value())
            .collect::<Result<Vec<_>, _>>();

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Foo didn't have a value");
    }

    #[test]
    fn bar_wrong_value() {
        let test_vec = vec![
            Foo::new(Some(Bar::new("bar".into()))),
            Foo::new(Some(Bar::new("foo".into()))),
            Foo::new(Some(Bar::new("bar".into()))),
            Foo::new(Some(Bar::new("bar".into())))
        ];
    
        let result = test_vec.iter()
            .map(|i| i.to_bar())
            .map(|i| i?.to_value())
            .collect::<Result<Vec<_>, _>>();

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Value was not bar");
    }

    #[test]
    fn vec_from_iter_keeps_err() {
        let test_vec = vec![
            Foo::new(Some(Bar::new("bar".into()))),
            Foo::new(None),
            Foo::new(Some(Bar::new("bar".into()))),
            Foo::new(Some(Bar::new("bar".into())))
        ];
    
        let foobar_iter = test_vec.iter()
            .map(|i| i.to_bar())
            .map(|i| i?.to_value());

        let result = Vec::from_iter(foobar_iter);

        assert_eq!(result, vec![
            Ok("bar".to_string()),
            Err("Foo didn't have a value"),
            Ok("bar".to_string()),
            Ok("bar".to_string())
        ]);
    }
}