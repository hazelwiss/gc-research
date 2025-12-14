import random

inputs_cnt = 10000
output = "inputsgen.c"

print("started generating")

string = ""
string += "#include \"fuzzer.h\"\n"
string += "#include <stddef.h>\n"
string += "struct state inputs[] = {\n"

for i in range(0,inputs_cnt):
    string += "(struct state) {{"
    for j in range(0,32):
        string += str(random.randint(0, 65535))
        string += ","
    string += "}},\n"

string += "};\n"
string += "size_t inputs_cnt = sizeof(inputs) / sizeof(*inputs);"

print("done generating")

with open(output, "w") as f:
    f.write(string)

print('wrote to file {}'.format(output))
