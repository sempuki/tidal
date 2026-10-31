// tidal-rules: DEL-3
// tidal-expect: reject
// tidal-error-clang: with a message is a C\+\+2c extension|expected .;. after delete
// tidal-error-gcc: reason only available with|expected .,. or .;. before .\(.
// tidal-error-edg: expected a .;.
void f() = delete("use g");
void g() { f(); }
