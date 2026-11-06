void a() noexcept {}
void b() {}
template <class F> void call(F f) noexcept(noexcept(f())) { f(); }
static_assert(noexcept(call(a)) && !noexcept(call(b)));
int main() {}
