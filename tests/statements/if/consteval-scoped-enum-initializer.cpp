// tidal-rules: IF-25 IF-18
// tidal-expect: run
// tidal-note: Write suite W19, finding W19-5 (reports/clean-room/W19/report.md). The initializer of a local const variable of scoped enumeration type is a constant evaluation, so if consteval takes its first branch. A local variable that is not const runs at run time.
enum class Mode { compile, run };
constexpr Mode mode() {
  if consteval { return Mode::compile; } else { return Mode::run; }
}
int main() {
  const Mode cm = mode();
  Mode m = mode();
  return cm == Mode::compile && m == Mode::run ? 0 : 1;
}
