// tidal-rules: ACC-20
// tidal-expect: run
// tidal-expect-edg: reject
// tidal-error-edg: inaccessible
// tidal-note: Gap review 2026-11-05 (reports/review/2026-11-05-gap-review-phase-9-classes-overloading-exceptions.md). The friend declaration names f of A<T*>, and A<int> does not match that pattern. The review reads that A<int>::f is no friend. Clang and GCC let it read the private member, as Tidal keeps (Q-0404). EDG rejects the access, as the review reads.
template <class T> struct A { int f(); };
template <class T> struct A<T*> { int f(); };
class C {
  static int secret() { return 7; }
  template <class T> friend int A<T*>::f();
};
template <class T> int A<T>::f() { return C::secret(); }
int main() { return A<int>{}.f() == 7 ? 0 : 1; }
