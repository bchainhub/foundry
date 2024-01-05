// Repros of fmt issues

// https://github.com/foxar-rs/foxar/issues/4403
function errorIdentifier() {
    bytes memory error = bytes("");
    if (error.length > 0) {}
}
