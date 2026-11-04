// tidal-rules: ODR-15
// tidal-expect: run
// tidal-note: Each extern declaration without an initializer only declares count, so the unit can repeat it. int count; is the one definition, and a variable with static storage duration starts at zero. Box::made is declared in the class and defined after it. Box::seen and Box::top are defined in the class.
extern int count;
extern int count;
int count;
extern int count;
struct Box {
  static int made;
  static inline int seen = 2;
  static constexpr int top = 3;
};
int Box::made = 1;
int main() { return count + Box::made + Box::seen + Box::top == 6 ? 0 : 1; }
