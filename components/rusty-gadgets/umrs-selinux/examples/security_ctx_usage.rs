use std::str::FromStr;
use umrs_selinux::context::SecurityContext;
use umrs_selinux::user::SelinuxUser;
use umrs_selinux::role::SelinuxRole;
use umrs_selinux::type_id::SelinuxType;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // ---------------------------------------------------------------------
    // Construct via primitives
    // ---------------------------------------------------------------------

    let user = SelinuxUser::from_str("system_u")?;
    let role = SelinuxRole::from_str("system_r")?;
    let security_type = SelinuxType::from_str("sshd_t")?;

    //let ctx = SecurityContext::new(user, role, security_type);
    let ctx = SecurityContext::new(user, role, security_type, None);

    println!("Context: {}", ctx);

    // ---------------------------------------------------------------------
    // Parse from canonical string
    // ---------------------------------------------------------------------

    let parsed: SecurityContext =
        "system_u:system_r:sshd_t".parse()?;

    println!("Parsed: {}", parsed);

    // ---------------------------------------------------------------------
    // Equality comparison
    // ---------------------------------------------------------------------

    assert_eq!(ctx, parsed);

    Ok(())
}
