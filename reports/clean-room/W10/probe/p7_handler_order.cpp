// Base handler before derived handler: TRY-4 says an implementation should warn.
struct Base {}; struct Derived : Base {};
int main() {
  try { throw Derived{}; } catch (const Base&) { return 0; } catch (const Derived&) { return 1; }
}
