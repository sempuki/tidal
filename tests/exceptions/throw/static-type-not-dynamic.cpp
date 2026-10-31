// tidal-rules: THR-4
// tidal-expect: run
struct B { virtual int id() const { return 1; } virtual ~B() {} };
struct D : B { int id() const override { return 2; } };
int main() { D d; B& b = d; try { throw b; } catch (D&) { return 1; } catch (B& e) { return e.id() == 1 ? 0 : 1; } return 2; }
