// tidal-rules: LSP-6
// tidal-expect: accept
extern "C" typedef void CFunc();
typedef void CppFunc();
void take(CFunc*);
void take(CppFunc*);
