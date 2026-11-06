// A subscript of an array member of a union with no active member.
// The front end reports "unsupported at line 6: pointer arithmetic".
union U { int items[2]; U() {} };
int main() {
  U u;
  int* p = &u.items[0];
  return p == nullptr;
}
