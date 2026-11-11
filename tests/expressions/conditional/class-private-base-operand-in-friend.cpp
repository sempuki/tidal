// tidal-rules: COND-16 ACC-12
// tidal-expect: run
// tidal-note: G153. Inside pick, a friend of Hidden, the private base Root is accessible. Step 1 binds a Root& to the Hidden lvalue directly, so the result is a Root lvalue, and the assignment changes the Root subobject of h.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
struct Root { int v = 0; };
struct Hidden : private Root {
  friend int pick(bool, Hidden&, Root&);
  int read() const { return v; }
};
int pick(bool b, Hidden& h, Root& r) {
  static_assert(same<decltype(b ? h : r), Root&>);
  (b ? h : r).v = 7;
  return h.read();
}
int main() {
  Hidden h;
  Root r;
  return pick(true, h, r) == 7 && r.v == 0 ? 0 : 1;
}
