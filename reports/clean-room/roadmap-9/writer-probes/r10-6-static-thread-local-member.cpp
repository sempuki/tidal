// R10-6: is a static thread_local data member constructed with the other
// thread_local variables of its unit? Build with -DUSE_MEMBER to read the
// member first; otherwise main reads the namespace-scope variable first.
int trace = 0;
struct T { int id; T(int i) : id(i) { trace = trace * 10 + i; } };
thread_local T plain(1);
struct Holder { static thread_local T member; };
thread_local T Holder::member(2);
int main() {
  int before = trace;
#ifdef USE_MEMBER
  int id = Holder::member.id;
#else
  int id = plain.id;
#endif
  int after = trace;
  __builtin_printf("before=%d id=%d after=%d\n", before, id, after);
  return 0;
}
