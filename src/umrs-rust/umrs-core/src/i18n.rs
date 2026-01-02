use gettext::Catalog;
use std::sync::OnceLock;

static CATALOG: OnceLock<Option<Catalog>> = OnceLock::new();

pub fn tr(msgid: &str) -> String {
    let cat_opt = CATALOG.get_or_init(|| {
        // Domain must match installed .mo name: umrs-tool.mo
        // Locale dir is where .mo files live
        Catalog::new("umrs-tool", "/usr/share/locale", None).ok()
    });

    match cat_opt {
        Some(cat) => cat.gettext(msgid),
        None => msgid.to_string(),
    }
}
