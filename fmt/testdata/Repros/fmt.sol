// Repros of fmt issues

function errorIdentifier() {
    bytes memory error = bytes("");
    if (error.length > 0) {}
}
