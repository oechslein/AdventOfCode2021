s = "{([(<{}[<>[]}>{[]{[(<()>"

import re

count = 1
while count > 0:
    for pair in (r"\(\)", r"\[\]", r"{}", r"<>"):
        (s, count) = re.subn(pair, "", s)
print([c for c in s if c in ")]}>"][0])

s = "[({(<(())[]>[[{[]{<()<>>"
count = 1
while count > 0:
    for pair in (r"\(\)", r"\[\]", r"{}", r"<>"):
        (s, count) = re.subn(pair, "", s)
print(s)
print([{'(': ')','[': ']','{': '}','<': '>'}[c] for c in reversed(s)])