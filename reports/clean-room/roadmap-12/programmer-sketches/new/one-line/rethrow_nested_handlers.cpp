// A bare throw rethrows the current exception object, caught by a base reference.
struct Err { virtual int code() const { return 1; } virtual ~Err() = default; };
struct Io : Err { int code() const override { return 7; } };
int main() { try { try { throw Io{}; } catch (Err&) { throw; } } catch (const Err& e) { return e.code() - 7; } return 1; }
