// tidal-rules: EXS-1
// tidal-expect: accept
void a() noexcept;
void b() noexcept(true);
void c() noexcept(false);
constexpr bool safe = true;
void d() noexcept(safe && sizeof(int) == 4);
static_assert(noexcept(a()) && noexcept(b()) && !noexcept(c()) && noexcept(d()));
