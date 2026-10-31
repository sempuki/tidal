// tidal-rules: NAM-27
// tidal-expect: accept
namespace n {
struct holder {
  template <class F> friend int probe(F) { return 1; }
  static constexpr auto act = [] { };
};
}
int read() { return probe(n::holder::act); }
