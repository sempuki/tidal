// tidal-rules: IF-25
// tidal-expect: run
// tidal-note: The initializer of a static local is a constant evaluation only if it is a constant expression. If it reads a variable that is not a constant, the whole initializer runs at run time, and if consteval takes the else branch everywhere in it. A static local of class type follows the same rule.
constexpr int where() {
  if consteval { return 1; } else { return 2; }
}
struct Pair {
  int first;
  int second;
};
struct Box {
  int value;
  constexpr Box(int v) : value(v) {}
};
int offset = 0;
int main() {
  static int once = where();
  static int late = where() + offset;
  static Pair both{where(), where()};
  static Pair mixed{where(), offset};
  static Box built{where()};
  static Box built_late{where() + offset};
  return once == 1 && late == 2 && both.first == 1 && both.second == 1 &&
                 mixed.first == 2 && mixed.second == 0 && built.value == 1 &&
                 built_late.value == 2
             ? 0
             : 1;
}
