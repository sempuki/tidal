// tidal-rules: DEL-2
// tidal-expect: reject
// tidal-expect-gcc-14: accept
// tidal-note: Q-0023. GCC 14 accepts a deleted main; GCC 16, Clang, and EDG reject it.
int main() = delete;
