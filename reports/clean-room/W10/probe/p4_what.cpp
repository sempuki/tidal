// A message through what(), a member the page leaves to the library.
namespace std {
class exception {
public:
  exception() noexcept {}
  virtual ~exception();
  virtual const char* what() const noexcept;
};
}
struct E : std::exception { const char* what() const noexcept override { return "bad"; } };
int main() {
  try { throw E{}; } catch (const std::exception& e) { return e.what()[0] == 'b' ? 0 : 1; }
}
