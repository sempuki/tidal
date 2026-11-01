// tidal-rules: CNS-7
// tidal-expect: accept
constexpr bool compile_time() {
  if consteval { return true; } else { return false; }
}
template <class T> concept Now = compile_time();
static_assert(Now<int>);
