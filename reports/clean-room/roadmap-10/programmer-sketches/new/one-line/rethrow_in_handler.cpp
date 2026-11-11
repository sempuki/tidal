// A handler that does partial work and rethrows with `throw;`; an outer handler catches it.
struct Error { int code; };
struct NotFound : Error {};
int steps = 0;
void inner() {
  try { throw NotFound{{404}}; }
  catch (Error&) { ++steps; throw; }
}
int main() {
  try { inner(); }
  catch (NotFound& e) { steps += e.code == 404; }
  catch (...) { return 2; }
  return steps == 2 ? 0 : 1;
}
