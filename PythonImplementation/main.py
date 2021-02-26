def try_parse_number(s):
    try:
        return float(s)
    except ValueError:
        return False


instructions = input().split()
op_codes = {"+": lambda a, b: a + b, "-": lambda a, b: a - b, "/": lambda a, b: a / b,
            "*": lambda a, b: a * b}
stack = []

for element in instructions:
    number = try_parse_number(element)
    if number:
        stack.append(float(element))
    action = op_codes.get(element, None)
    if action:
        b, a = stack.pop(), stack.pop()
        stack.append(action(a, b))

print("Last value on the stack:", stack[-1])
print("Stack dump:", stack)
