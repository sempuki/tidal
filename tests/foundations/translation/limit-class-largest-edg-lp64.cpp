// tidal-rules: TR-15
// tidal-expect: reject
// tidal-expect-clang-18: reject
// tidal-error-clang-18: static assertion failed
// tidal-expect-edg: accept
// tidal-error-clang: structure 'Block' is too large
// tidal-error-gcc: size of type .Block. is too large
// tidal-note: Q-0012. The size of the class is the largest value of std::size_t. EDG accepts it. Clang 22 and GCC reject it. Clang 18 accepts the class and gives sizeof a wrong value, so the static_assert fails (Q-0350).
static_assert(sizeof(void*) == 8);
struct Block {
  char a[(1ull << 61) - 1], b[(1ull << 61) - 1], c[(1ull << 61) - 1], d[(1ull << 61) - 1];
  char e[(1ull << 61) - 1], f[(1ull << 61) - 1], g[(1ull << 61) - 1], h[(1ull << 61) - 1];
  char rest[7];
};
Block largest;
static_assert(sizeof(Block) == ~0ull);
