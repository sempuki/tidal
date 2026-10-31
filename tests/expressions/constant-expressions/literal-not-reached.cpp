// tidal-rules: CE-23 CE-17
// tidal-expect: accept
struct Logged {
  int value;
  ~Logged() {}
};
constexpr int pick(bool use_log) {
  if (use_log) {
    Logged logged{1};
    return logged.value;
  }
  return 0;
}
static_assert(pick(false) == 0);
