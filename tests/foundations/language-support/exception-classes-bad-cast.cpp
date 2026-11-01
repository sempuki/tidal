// tidal-rules: LSUP-8 VIRT-10
// tidal-expect: run
namespace std {
class exception {
public:
  exception() noexcept {}
  virtual ~exception();
};
class bad_cast : public exception {
public:
  bad_cast() noexcept {}
  ~bad_cast() override;
};
}
struct Shape { virtual ~Shape() {} };
struct Circle : Shape {};
int main() {
  Shape plain;
  Shape& s = plain;
  try { (void)dynamic_cast<Circle&>(s); return 1; }
  catch (std::bad_cast&) {}
  catch (...) { return 2; }
  try { (void)dynamic_cast<Circle&>(s); return 3; }
  catch (std::exception&) {}
  return 0;
}
