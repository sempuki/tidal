// tidal-rules: TYN-1
// tidal-expect: run
using T = int;
namespace n { using T = double; }
int main() { using T = char; T c = 'a'; n::T d = 0.5; ::T i = 2; return sizeof(c) == 1 && d == 0.5 && i == 2 ? 0 : 1; }
