// W12: a counter captured by a lambda.

template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;

// A counter factory: each call gives a closure with its own state.
constexpr auto make_counter(int start) {
  return [k = start]() mutable { return k++; };
}

constexpr int own_state() {
  auto c = [k = 0]() mutable { return ++k; };
  c();
  c();
  auto d = c;          // a copy carries the state at the time of the copy
  c();
  return c() * 10 + d();   // c: 4, d: 3
}
static_assert(own_state() == 43);

constexpr int shared_by_reference() {
  int n = 0;
  auto bump = [&n] { ++n; };    // no mutable needed (LAM-3)
  auto peek = [n] { return n; };// snapshot (LAM-4)
  bump();
  bump();
  bump();
  return n * 10 + peek();       // 30
}
static_assert(shared_by_reference() == 30);

constexpr int factory() {
  auto a = make_counter(5);
  auto b = make_counter(100);
  a();
  a();
  return a() + b();             // 7 + 100
}
static_assert(factory() == 107);

// Types seen inside the closure (LAM-18, DTY-1).
constexpr bool types_ok() {
  int x = 1;
  auto f = [x] {
    return same<decltype((x)), const int&> && same<decltype(x), int>;
  };
  auto g = [x]() mutable { return same<decltype((x)), int&>; };
  return f() && g();
}
static_assert(types_ok());

// Each lambda expression has its own type (LAM-2).
inline auto c1 = make_counter(0);
inline auto c2 = make_counter(0);
static_assert(same<decltype(c1), decltype(c2)>);   // one expression, one type
static_assert(!same<decltype([] {}), decltype([] {})>);

int main() {
  int failures = 0;
  if (own_state() != 43) ++failures;
  if (shared_by_reference() != 30) ++failures;
  if (factory() != 107) ++failures;
  if (!types_ok()) ++failures;
  auto c = make_counter(0);
  for (int i = 0; i < 10; ++i) c();
  if (c() != 10) ++failures;
  return failures;
}
