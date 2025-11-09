def greet(string):
    return f"Hello, {string}!"

def farewell(string):
    return f"Goodbye, {string}!"

# 将函数赋值给变量
my_function = greet

# 通过变量调用函数
message = my_function("Alice")
print(message)  # 输出: Hello, Alice!

# 改变变量指向的函数
my_function = farewell
message = my_function("Bob")
print(message)  # 输出: Goodbye, Bob!
