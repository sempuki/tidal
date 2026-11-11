// tidal-rules: IF-13 CPT-26 LIST-12
// tidal-expect: run
// tidal-note: Roadmap round 14, programmer reduction if_constexpr_braced_initializers_nested_aggregate.cpp (from fields_first_try.cpp). The condition of if constexpr tests whether three values initialize T. The last member of Pair is an aggregate with one member, so the third value has no member (LIST-12) and the requirement does not hold for Pair (CPT-26). For Pair, the branch with a structured binding of three names is discarded and not instantiated (IF-13). For Triple, the other branch is discarded.
struct Inner { int r; };
struct Pair { short x; Inner in; };
struct Triple { int a, b, c; };
template <class T> constexpr int sum(const T& t) {
  if constexpr (requires { T{1, 2, 3}; }) {
    auto& [a, b, c] = t;
    return a + b + c;
  } else {
    auto& [a, b] = t;
    return a + b.r;
  }
}
static_assert(sum(Pair{1, {2}}) == 3 && sum(Triple{3, 4, 5}) == 12);
int main() {
  Pair p{1, {2}};
  Triple t{3, 4, 5};
  return sum(p) == 3 && sum(t) == 12 ? 0 : 1;
}
