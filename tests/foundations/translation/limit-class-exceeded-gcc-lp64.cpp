// tidal-rules: TR-15
// tidal-expect: reject
// tidal-expect-clang-18: accept
// tidal-expect-edg: accept
// tidal-error-clang: structure 'Block' is too large
// tidal-error-gcc: size of type .Block. is too large
// tidal-note: Q-0012. GCC's largest class on x86-64 has 2^63 - 1 bytes. EDG accepts a class of 2^63 bytes. Clang 18 accepts the class, though it computes a wrong size for it (Q-0350).
static_assert(sizeof(void*) == 8);
struct Block {
  char a[(1ull << 61) - 1], b[(1ull << 61) - 1], c[(1ull << 61) - 1], d[(1ull << 61) - 1];
  char rest[4];
};
Block past;
