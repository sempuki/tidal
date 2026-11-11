#!/usr/bin/env python3
"""Read-only scans of tests/ for the forms of this round's reductions.
Prints the count and the files for each form."""
import re, glob, os
root = os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', '..', '..', 'tests')
files = {f: open(f, errors='ignore').read() for f in glob.glob(root + '/**/*.cpp', recursive=True)}
def report(name, pred):
    hits = [os.path.relpath(f, root) for f, s in files.items() if pred(s)]
    print(f'{name}: {len(hits)}'); [print('  ' + h) for h in hits[:8]]

# 1. A member function template of a class template whose parameter list
#    expands the class's own pack after a deduced parameter.
def pack_after_deduced(s):
    for m in re.finditer(r'template\s*<\s*(?:class|typename)\s*\.\.\.\s*(\w+)\s*>\s*(?:struct|class)|template\s*<[^>]*(?:class|typename)\s*\.\.\.\s*(\w+)\s*>\s*(?:struct|class)', s):
        P = m.group(1) or m.group(2)
        body = s[m.end():]
        if re.search(r'template\s*<\s*(?:class|typename)\s+\w+\s*>[^;{]*\([^)]*,\s*' + P + r'\b[^)]*\.\.\.', body):
            return True
    return False
report('member template with the class pack after a deduced parameter', pack_after_deduced)

# 2. A pointer to member as a template argument of a function template call.
report('call f<&C::m>(...)', lambda s: re.search(r'\w+\s*<\s*&\s*\w+::\w+\s*>\s*\(', s) is not None)

# 3. An abbreviated function template (a parameter declared auto) whose body
#    holds a generic lambda.
def abbrev_generic_lambda(s):
    for m in re.finditer(r'^[\w:<> ]*\b\w+\s*\([^)]*\bauto\b[^)]*\)\s*(?:const\s*)?\{', s, re.M):
        body = s[m.end():m.end() + 400]
        if re.search(r'\]\s*\([^)]*\bauto\b', body):
            return True
    return False
report('abbreviated function template with a generic lambda', abbrev_generic_lambda)

# 4. Class template argument deduction with braces in a return statement or
#    an auto variable, inside a template: return Name{...} or = Name{...}
#    where Name is a class template used with no argument list.
def ctad_braced_in_template(s):
    names = set(re.findall(r'template\s*<[^>]*>\s*(?:struct|class)\s+(\w+)', s))
    for n in names:
        if re.search(r'(?:return|=)\s*' + n + r'\s*\{', s) and re.search(r'template\s*<[^>]*>\s*[^;{]*\([^)]*\)\s*\{[^}]*(?:return|=)\s*' + n + r'\s*\{', s):
            return True
    return False
report('return Name{...} with deduction inside a function template', ctad_braced_in_template)

# 5. An explicit constructor from a pointer to a base, called with a pointer
#    to a derived class, beside a constructor template of the same class.
report('Iter(Derived*) beside a constructor template (rough)',
       lambda s: 'explicit' in s and re.search(r'template\s*<\s*bool\s+\w+\s*=', s) is not None)
