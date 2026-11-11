// A conditional noexcept that depends on a member's noexcept.
struct Safe { void run() noexcept {} };
struct Risky { void run() {} };
template <class T> void call(T& t) noexcept(noexcept(t.run())) { t.run(); }
int main() {
  Safe s; Risky r;
  static_assert(noexcept(call(s)) && !noexcept(call(r)));
  call(s); call(r);
  return 0;
}
