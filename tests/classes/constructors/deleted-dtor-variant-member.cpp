// tidal-rules: DTOR-3
// tidal-expect: reject
// tidal-error-clang: attempt to use a deleted function
// tidal-error-gcc: use of deleted function .*~Holder
// tidal-error-edg: ~Holder\(\).* deleted function
// tidal-note: A variant member counts as a member for DTOR-3. The destructor of Pinned is deleted, so the implicit destructor of the union Holder is deleted, and a local variable of type Holder is not valid. The deleted destructor of Pinned is trivial, so UNI-3 alone does not delete it.
struct Pinned { int value; ~Pinned() = delete; };
union Holder {
  Pinned pinned;
  int number;
  Holder() : number(0) {}
};
void use() { Holder holder; }
