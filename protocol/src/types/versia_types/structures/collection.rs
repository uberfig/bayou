use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Collection<T: Clone + PartialEq> {
    /// Author of the collection. Usually the user who owns the collection.
    /// Can be set to null to represent the instance.
    pub author: Option<Url>,
    /// URI to the last page of the collection. Query parameters are allowed.
    pub first: Url,
    /// URI to the last page of the collection. Query parameters are allowed.
    ///
    /// If the collection only has one page, this should be the same as first
    pub last: Url,
    pub total: u64,
    pub next: Option<Url>,
    pub previous: Option<Url>,
    pub items: Vec<T>,
}

impl<T: Clone + PartialEq + Serialize + for<'a> Deserialize<'a>> Collection<T> {
    /// current page should never be 0
    ///
    /// https://{instance_domain}/{path}?page={}
    pub fn new(
        items: Vec<T>,
        total: u64,
        pagnation_size: u64,
        current_page: u64,
        author: Option<Url>,
        instance_domain: &str,
        path: &str,
    ) -> Collection<T> {
        let first = Url::parse(&format!("https://{}/{}?page=1", instance_domain, path)).unwrap();
        let last_page = total.div_ceil(pagnation_size);
        let last = match total > pagnation_size {
            true => Url::parse(&format!(
                "https://{}/{}?page={}",
                instance_domain, path, last_page
            ))
            .unwrap(),
            false => first.clone(),
        };
        let next = match current_page.eq(&last_page) {
            true => None,
            false => Some(
                Url::parse(&format!(
                    "https://{}/{}?page={}",
                    instance_domain,
                    path,
                    current_page + 1
                ))
                .unwrap(),
            ),
        };
        let previous = match current_page.eq(&1) {
            true => None,
            false => Some(
                Url::parse(&format!(
                    "https://{}/{}?page={}",
                    instance_domain,
                    path,
                    current_page - 1
                ))
                .unwrap(),
            ),
        };
        Collection {
            author,
            first,
            last,
            total,
            next,
            previous,
            items,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::super::postable::VersiaPostable;
    use super::*;

    #[test]
    fn test_deserialize() -> Result<(), String> {
        //taken from the versia protocol examples
        let collection = r#"
{
    "author": "https://versia.social/users/018ec082-0ae1-761c-b2c5-22275a611771",
    "first": "https://versia.social/users/018ec082-0ae1-761c-b2c5-22275a611771/outbox?page=1",
    "last": "https://versia.social/users/018ec082-0ae1-761c-b2c5-22275a611771/outbox?page=3",
    "total": 46,
    "next": "https://versia.social/users/018ec082-0ae1-761c-b2c5-22275a611771/outbox?page=2",
    "previous": null,
    "items": [
        {
            "id": "456df8ed-daf1-4062-abab-491071c7b8dd",
            "type": "Note",
            "uri": "https://versia.social/notes/456df8ed-daf1-4062-abab-491071c7b8dd",
            "created_at": "2024-04-09T01:38:51.743Z",
            "author": "https://versia.social/users/018eb863-753f-76ff-83d6-fd590de7740a",
            "content": {
                "text/plain": {
                    "content": "Hello, world!"
                }
            }
        }
    ]
}
"#;
        let deserialized: Result<Collection<VersiaPostable>, serde_json::Error> =
            serde_json::from_str(collection);
        match deserialized {
            Ok(_) => Ok(()),
            Err(x) => Err(format!("collection deserialize failed: {}", x)),
        }
    }
}
