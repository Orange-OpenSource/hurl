use crate::diff::core::diff;

fn json_text1() -> String {
    r#"{
  "first_name": "John",
  "last_name": "Smith",
  "is_alive": true,
  "age": 27,
  "address": {
    "street_address": "21 2nd Street",
    "city": "New York",
    "state": "NY",
    "postal_code": "10021-3100"
  },
  "phone_numbers": [
    {
      "type": "home",
      "number": "212 555-1234"
    },
    {
      "type": "office",
      "number": "646 555-4567"
    }
  ],
  "children": [
    "Catherine",
    "Thomas",
    "Trevor"
  ],
  "spouse": null
}"#
    .to_string()
}

fn json_text2() -> String {
    r#"{
  "first_name": "John",
  "last_name": "Smith",
  "is_alive": true,
  "age": 28,
  "address": {
    "street_address": "21 2nd Street",
    "city": "New York",
    "state": "NY",
    "postal_code": "10021-3100"
  },
  "phone_numbers": [
    {
      "type": "home",
      "number": "212 555-1234"
    },
    {
      "type": "office",
      "number": "646 555-4567"
    }
  ],
  "children": [
    "Catherine",
    "Thomas",
    "Trevor"
  ],
  "spouse": null
}"#
    .to_string()
}

#[test]
fn test1() {
    assert_eq!(
        diff(&json_text1(), &json_text2()),
        vec![
            r#"@@ -2,7 +2,7 @@"#,
            r#"   "first_name": "John""#,
            r#"   "last_name": "Smith""#,
            r#"   "is_alive": true"#,
            r#"-  "age": 27"#,
            r#"+  "age": 28"#,
            r#"   "address": {"#,
            r#"   "street_address": "21 2nd Street""#,
            r#"   "city": "New York""#,
        ]
    )
}
