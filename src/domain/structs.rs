use ::xml;
use ::xml::writer::events::XmlEvent;

#[derive(PartialEq, Debug)]
pub struct Domain {
    pub type_: String,
    pub id: String,
}

impl Domain {
    pub fn xml(&self) -> String {
        let mut b = Vec::new();
        {
            let mut w = xml::EmitterConfig::default().perform_indent(true).create_writer(&mut b);
            w.write(XmlEvent::start_element("domain").attr("type", &self.type_).attr("id", &self.id));
            w.write(XmlEvent::end_element());
        }
        String::from_utf8_lossy(&b).into_owned()
    }
}


#[cfg(test)]
mod test {
    use super::Domain;

    #[test]
    fn domain_attributes() {
        let d = Domain{type_: "xen".to_string(), id: "3".to_string()};
        let s = r#"
<?xml version="1.0" encoding="utf-8"?>
<domain type="xen" id="3" />"#;

        assert_eq!(s.trim(), d.xml().trim());
    }
}
