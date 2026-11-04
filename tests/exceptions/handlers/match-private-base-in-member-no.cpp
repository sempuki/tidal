// tidal-rules: TRY-2
// tidal-expect: run
// tidal-note: Inside a member function of the derived class, the private and protected bases are accessible. A handler for the base still does not match, because only a public base matches.
struct B {};
struct D : private B { int f() { try { throw D(); } catch (B&) { return 1; } catch (...) { return 0; } return 2; } };
struct P : protected B { int f() { try { throw P(); } catch (B&) { return 1; } catch (...) { return 0; } return 2; } };
int main() { return D().f() + P().f(); }
