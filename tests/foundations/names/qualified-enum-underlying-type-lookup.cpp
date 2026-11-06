// tidal-rules: NAM-21 NAM-8
// tidal-expect: reject
// tidal-note: Gap review 2026-11-05 (reports/review/2026-11-05-gap-review-phase-9-declarations-statements-foundations.md). Q-0410. The definition of N::E names its underlying type T, which only N declares. Clang does not look up T in N, and GCC does not parse the form. The review reads that T is looked up in N.
namespace N { using T = short; enum E : int; }
enum N::E : T { a };
