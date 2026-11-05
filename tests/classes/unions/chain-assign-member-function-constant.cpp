// tidal-rules: UNI-13
// tidal-expect: accept
// tidal-note: The constant-evaluation form of chain-assign-member-function.cpp. Neither union member is active before the member function writes an element of the array.
struct Small {
  union { char buf[4]; char* far; };
  constexpr Small() {}
  constexpr void put(int i, char c) { buf[i] = c; }
};
constexpr char second() {
  Small s;
  s.put(1, 'q');
  return s.buf[1];
}
static_assert(second() == 'q');
