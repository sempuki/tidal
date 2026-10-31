// tidal-rules: JMP-5
// tidal-expect: reject
// tidal-error-clang: redefinition of label .here
// tidal-error-gcc: duplicate label .here
// tidal-error-edg: label .here. has already been defined
void f() {
here:;
here:;
}
