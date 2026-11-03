// tidal-rules: OBJ-9
// tidal-expect: run
// tidal-note: The full expression reads a member of the temporary. The temporary ends after the read and before the next statement.
int trace = 0;
void mark(int d) { trace = trace * 10 + d; }
struct T { int id; T(int i) : id(i) { mark(i); } ~T() { mark(id + 2); } };
int main() {
  int id = T(1).id;
  mark(9);
  return id == 1 && trace == 139 ? 0 : 1;
}
