 use umrs_core::console;

 fn main() {
     console::init();

     console::info(“Starting umrs-state”);

     console::status(“FIPS mode enabled”, true);
     console::status(“SELinux enforcing”, false);

     console::warn(“AIDE database is stale (last update: 8 days ago)”);
     console::error(“Audit backlog exceeded threshold”);

     console::debug(“Audit backlog observed during summary check”);
 }
