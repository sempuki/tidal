// tidal-rules: PRIM-5 PRIM-7 LOOP-1 JMP-2 SW-1 ATTR-7 CX-3 RFOR-1
// tidal-expect: run
// tidal-note: Write suite W3, program tokenize.cpp (reports/clean-room/W3/report.md). The report has no construct-to-rule notes, so the rules are the ones the program's comments name.
// tidal-note: A program test is evidence from use and decides no rule alone (decision 0054).
// Tokenizer for arithmetic text such as "12 + 3*(40 - 5)".
// Rules used: PRIM-5 (char literals), PRIM-7 (string literal is an array
// of const char with a terminating null), LOOP-1 (while), JMP-2 (continue
// inside a switch goes to the loop), SW-1 (grouped case labels),
// ATTR-7 (labels directly followed by labels do not fall through),
// ENUM-P2 (scoped enum), CX-3 (constexpr function at both times).

enum class Tok { Num, Plus, Minus, Star, Slash, LParen, RParen, Error };

struct Token {
  Tok kind = Tok::Error;
  int value = 0;  // number value, or offset of the bad character
};

constexpr int max_tokens = 32;

struct Tokens {
  Token t[max_tokens] = {};
  int n = 0;
  bool ok = true;
};

constexpr bool is_digit(char c) { return c >= '0' && c <= '9'; }

constexpr Tokens tokenize(const char* text) {
  Tokens out;
  int i = 0;
  while (text[i] != '\0') {
    char c = text[i];
    if (out.n == max_tokens) {
      out.ok = false;
      break;
    }
    Token tk;
    switch (c) {
      case ' ':
      case '\t':
      case '\n':
        ++i;
        continue;  // JMP-2: goes to the while condition
      case '+': tk.kind = Tok::Plus; break;
      case '-': tk.kind = Tok::Minus; break;
      case '*': tk.kind = Tok::Star; break;
      case '/': tk.kind = Tok::Slash; break;
      case '(': tk.kind = Tok::LParen; break;
      case ')': tk.kind = Tok::RParen; break;
      default:
        if (is_digit(c)) {
          int v = 0;
          while (is_digit(text[i])) {
            v = v * 10 + (text[i] - '0');
            ++i;
          }
          tk.kind = Tok::Num;
          tk.value = v;
          out.t[out.n++] = tk;
          continue;  // i already points past the number
        }
        tk.kind = Tok::Error;
        tk.value = i;
        out.ok = false;
        break;
    }
    out.t[out.n++] = tk;
    ++i;
    if (!out.ok) break;
  }
  return out;
}

// Encodes the token kinds as one digit each, for easy checks.
constexpr long long shape(const Tokens& ts) {
  long long s = 0;
  for (int k = 0; k < ts.n; ++k) s = s * 10 + static_cast<int>(ts.t[k].kind);
  return s;
}

constexpr Tokens a = tokenize("12 + 3*(40 - 5)");
static_assert(a.ok && a.n == 9);
static_assert(a.t[0].value == 12 && a.t[2].value == 3 && a.t[5].value == 40);
// Num Plus Num Star LParen Num Minus Num RParen
static_assert(shape(a) == 10350206);  // leading Num (0) drops out

constexpr Tokens b = tokenize("7 % 2");
static_assert(!b.ok && b.n == 2 && b.t[1].kind == Tok::Error && b.t[1].value == 2);

static_assert(tokenize("").n == 0 && tokenize("   ").ok);

int main() {
  char buf[] = "100/(2+3)";
  Tokens r = tokenize(buf);
  if (!r.ok) return 1;
  if (r.n != 7) return 2;
  if (r.t[0].value != 100) return 3;
  if (r.t[1].kind != Tok::Slash) return 4;
  int sum = 0;
  for (const Token& t : r.t) {  // RFOR-1: all 32 slots, unused ones are Error
    if (t.kind == Tok::Num) sum += t.value;
  }
  if (sum != 105) return 5;
  return 0;
}
