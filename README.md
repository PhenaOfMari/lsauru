# lsauru -- ls AUR Updates

This is a simple rust binary crate to list available updates for installed AUR packages. It
checks pacman for installed foreign packages and cross-references the result with a query
to the AUR RPC interface. Any packages with a version greater than installed are listed.
If an installed foreign package is not found in the AUR, it will also list this finding.

### Required system packages
- pacman
- curl
