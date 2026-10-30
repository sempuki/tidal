// tidal-rules: UN-18
// tidal-expect: accept
void may_throw();
void never() noexcept;
int i;
static_assert(noexcept(i + 1));
static_assert(!noexcept(may_throw()));
static_assert(noexcept(never()));
static_assert(noexcept(throw 1) == false);
