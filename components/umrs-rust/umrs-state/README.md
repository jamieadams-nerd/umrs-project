✅ How this helps for future keys

To add a new key, you only need to:

enum UmrsKey {
    Purpose,
    SystemType,
    Virtualization,
    NewKey,  // <--- just add this
}

impl UmrsKey {
    fn as_str(&self) -> &'static str {
        match self {
            UmrsKey::Purpose => "system_metadata.purpose",
            UmrsKey::SystemType => "system_metadata.system_type",
            UmrsKey::Virtualization => "system_metadata.virtualization",
            UmrsKey::NewKey => "system_metadata.new_key",  // <--- add here
        }
    }

    fn all() -> &'static [UmrsKey] {
        &[UmrsKey::Purpose, UmrsKey::SystemType, UmrsKey::Virtualization, UmrsKey::NewKey]
    }
}


get, set, and list-keys automatically work.

No repeated string literals, no typos.
