struct Error { int code; };
struct Result {
  bool ok;
  union { int value; Error error; };
  Result(int v) : ok(true), value(v) {}
};
int main() {
  Result r(4);
  return r.error.code;  // the active member is value
}
