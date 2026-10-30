// tidal-rules: ACC-2
// tidal-expect: run
class C {
  int secret = 5;
public:
  int peek(const C& other) const { return other.secret + secret; }
};
int main() { C a, b; return a.peek(b) == 10 ? 0 : 1; }
