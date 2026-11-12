// tidal-rules: TSP-32 UNI-17
// tidal-expect: accept
// tidal-note: Front-end ledger row G182. A union template has a partial specialization, one marked final, and an explicit specialization, each with the key union.
template <class T, class U> union Either { T left; U right; };
template <class T> union Either<T, int> { T left; int right; char extra[16]; };
template <class T> union Either<T*, T*> final { T* one; long count; };
template <> union Either<char, char> { char c; long long wide; };
constexpr Either<short, int> mixed{4};
static_assert(mixed.left == 4);
static_assert(sizeof(Either<short, int>) >= 16);
static_assert(sizeof(Either<int*, int*>) >= sizeof(long));
static_assert(sizeof(Either<char, char>) == sizeof(long long));
