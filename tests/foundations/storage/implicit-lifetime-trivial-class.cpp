// tidal-rules: STG-7
// tidal-expect: run
// tidal-flags: -O0 -fsanitize=address,undefined -fno-sanitize-recover=all
class Celsius {
  double degrees;
public:
  Celsius() = default;
  explicit Celsius(double d) : degrees(d) {}
  void set(double d) { degrees = d; }
  double get() const { return degrees; }
};
int main() {
  void* raw = ::operator new(sizeof(Celsius));
  Celsius* reading = static_cast<Celsius*>(raw);   // trivial default constructor and destructor
  reading->set(21.5);
  bool ok = reading->get() == 21.5;
  ::operator delete(raw);
  return ok ? 0 : 1;
}
