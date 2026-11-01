// tidal-rules: LAM-10
// tidal-expect: accept
// tidal-note: A generic lambda without captures converts to a pointer to function. The target type gives the type of each auto parameter.
void (*ignore)(int) = [](auto) {};
int (*narrow)(long) = [](auto x) { return int(x); };
constexpr int (*next)(int) = [](auto x) { return x + 1; };
static_assert(next(1) == 2);
