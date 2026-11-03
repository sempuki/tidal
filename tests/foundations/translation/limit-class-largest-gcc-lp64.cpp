// tidal-rules: TR-15
// tidal-expect: accept
// tidal-expect-clang: reject
// tidal-expect-clang-18: reject
// tidal-error-clang-18: static assertion failed
// tidal-error-clang: structure 'Block' is too large
// tidal-note: Q-0012. A class of 2^63 - 1 bytes is the largest that GCC accepts on x86-64. EDG accepts it. Clang 22 rejects it. Clang 18 accepts the class and gives sizeof a wrong value, so the static_assert fails (Q-0350).
static_assert(sizeof(void*) == 8);
struct Block {
  char a[(1ull << 61) - 1], b[(1ull << 61) - 1], c[(1ull << 61) - 1], d[(1ull << 61) - 1];
  char rest[3];
};
Block largest;
static_assert(sizeof(Block) == (1ull << 63) - 1);
