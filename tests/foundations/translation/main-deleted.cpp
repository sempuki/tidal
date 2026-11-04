// tidal-rules: TR-3
// tidal-expect: reject
// tidal-expect-gcc-14: accept
// tidal-error-clang: not allowed to be deleted
// tidal-error-gcc: cannot be deleted
// tidal-error-edg: cannot be deleted
// tidal-note: GCC 14 accepts a deleted main. GCC 16 rejects it.
int main() = delete;
