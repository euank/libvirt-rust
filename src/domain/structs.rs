use std::io::Write;
use xml;
use xml::writer::events::XmlEvent;

#[derive(PartialEq, Debug)]
pub struct Domain<'a> {
    // attributes
    pub type_: &'a str,
    pub id: Option<i32>,

    // fields
    pub name: &'a str,
    pub uuid: &'a str,
    pub title: Option<&'a str>,
    pub description: Option<&'a str>,
    pub metadata: Vec<&'a str>, // TODO, represent better
}

fn write_el<W: Write>(w: &mut xml::writer::EventWriter<W>, name: &str, val: &str) {
    w.write(XmlEvent::start_element(name));
    w.write(val);
    w.write(XmlEvent::end_element());
}

impl<'a> Domain<'a> {
    pub fn xml(&self) -> String {
        let mut b = Vec::new();
        {
            let idstr = self.id.map(|e| e.to_string());

            let mut w = xml::EmitterConfig::default()
                .perform_indent(true)
                .create_writer(&mut b);
            let mut domain_ev = XmlEvent::start_element("domain").attr("type", self.type_);

            domain_ev = match idstr {
                Some(ref id) => domain_ev.attr("id", id),
                _ => domain_ev,
            };

            w.write(domain_ev);
            write_el(&mut w, "name", self.name);
            write_el(&mut w, "uuid", self.uuid);
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
        let d = Domain {
            type_: "xen",
            id: Some(3),
            name: "bob",
            uuid: "1234",
            title: None,
            description: None,
            metadata: Vec::new(),
        };
        let s = r#"
<?xml version="1.0" encoding="utf-8"?>
<domain type="xen" id="3">
  <name>bob</name>
  <uuid>1234</uuid>
</domain>"#;

        assert_eq!(s.trim(), d.xml().trim());
    }
}
