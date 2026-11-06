// tidal-rules: PACK-13 PACK-14
// tidal-expect: run
// tidal-note: Roadmap round 8, programmer reduction base_template_id_mem_init_pack_argument.cpp. A pack expands in the expression list of an initializer of a base that a template-id names.
template <class... T> struct base {
  int n;
  base(T... v) : n((v + ...)) {}
};

template <class... T>
struct derived : base<T...> {
  derived(T... v) : base<T...>(v...) {}
};

int main() {
  derived<int, int> d(2, 3);
  return d.n == 5 ? 0 : 1;
}
