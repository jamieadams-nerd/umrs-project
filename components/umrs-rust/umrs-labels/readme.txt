dnf install clippy rustfmt

 1304  cargo clippy
 1305  cargo clippy -- -W clippy::all
 1307  cargo clippy -- -W clippy::all


That confirms it is read as a file path, not embedded data.

Example usage:

umrs-labels --metadata ./umrs-metadata.json --list


Or with short option:

umrs-labels -m /etc/umrs/metadata/us.json -l


And lookups:

umrs-labels -m metadata.json -s S4
umrs-labels -m metadata.json -c PRIV_GEN


If the path is wrong or unreadable, this line will trigger a clear error:

Failed to read "<path>"


If you want to improve UX later (optional ideas):

Default metadata path (e.g. /etc/umrs/metadata.json)
