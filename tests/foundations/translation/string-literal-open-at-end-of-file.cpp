// tidal-rules: TR-12
// tidal-expect: reject
// tidal-error-clang: missing terminating
// tidal-error-gcc: missing terminating
// tidal-error-edg: missing closing quote
// tidal-note: Front-end ledger row G180. A string literal that is still open at the end of the file derives from no production of the grammar (TR-12). The file ends inside the literal, with no newline after it.
int main() { return 0; }
const char* text = "open at the end