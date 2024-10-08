# Blinking the darn LED

You will have to use a local installation of probe-rs for this.

1) Clone probe-rs
2) Use target-gen to add the chip via the CMSIS pack
3) add the generated .yml to /targets in probe-rs root dir. In this case
its going to be called MCXC444VLH.yml.
4) rebuild and install locally via `cargo install --path probe-rs-tools --force`

# TODO

- add better instructions
- actually blink the LED

