// tidal-rules: LOOP-3
// tidal-expect: run
// tidal-expect-gcc-14: run-fail
// tidal-note: Gap review 2026-10-30. Q-0044. GCC 14 destroys the for condition variable before the iteration expression; GCC 16, Clang, and EDG after it.
char log[16];
int pos = 0;
struct Flag {
  bool on;
  ~Flag() { log[pos++] = 'd'; }
  explicit operator bool() const { return on; }
};
int main() {
  int i = 0;
  for (; Flag f{i < 2}; log[pos++] = 's', ++i) log[pos++] = 'b';
  const char* want = "bsdbsdd";
  for (int k = 0; want[k] || log[k]; ++k) if (want[k] != log[k]) return 1;
  return 0;
}
