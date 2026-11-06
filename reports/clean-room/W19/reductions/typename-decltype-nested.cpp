// `typename` before a nested name that starts with decltype(...)::.
// The grammar of expressions/primary.md (nested_name: decltype_spec "::")
// and templates/dependent-names.md (typename_specifier) derives this line.
// g++ and clang++ accept it, and the program exits 0.
// tidal-fe: rejected [TR-12] at line 8: expected identifier, found 'decltype'.
struct S { using type = int; };
S s;
typename decltype(s)::type x = 0;
int main() { return x; }
