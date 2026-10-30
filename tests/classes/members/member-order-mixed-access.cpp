// tidal-rules: CLS-10
// tidal-expect: run
// tidal-note: Gap review 2026-10-30.
class S {
public:
  int a = 0;
private:
  int b = 0;
public:
  bool ordered() const { return &a < &b; }
};
int main() { return S{}.ordered() ? 0 : 1; }
