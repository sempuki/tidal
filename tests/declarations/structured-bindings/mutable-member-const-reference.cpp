// tidal-rules: SB-9
// tidal-expect: run
// tidal-note: The hidden variable is a const reference to an object. A change through the binding changes the object's mutable member.
struct M { mutable int m; int n; };
int main() {
  M obj{1, 2};
  const auto& [x, y] = obj;
  x = 7;
  return obj.m == 7 && y == 2 ? 0 : 1;
}
