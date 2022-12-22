import webbrowser
from ast import literal_eval
from copy import deepcopy
get_input = input


def is_str_float(f):
    try:
        float(f)
        return True
    except:
        try:
            return type(eval(f)) is float
        except:
            return False


input = "{\n"
with open("input.txt") as f:
    for l in f.readlines():
        input += f'"{l.split(": ")[0].strip()}": "{l.split(":")[1].strip()}",\n'
input += "}"

input = literal_eval(input)

for k, v in input.items():
    try:
        input[k] = float(v)
    except:
        pass

jobs = deepcopy(input)
while not all(map(lambda k: type(jobs[k]) is float, jobs.keys())):
    for k, v in jobs.items():
        try:
            for pk in v.split(" "):
                if pk in jobs.keys():
                    jobs[k] = v.replace(pk, f"( {jobs[pk]} )")
        except:
            pass

        try:
            jobs[k] = float(eval(v))
        except:
            pass

print(f'1: {int(jobs["root"])}')

input["root"] = input["root"].replace("+", "=")
del input["humn"]

while not all(map(lambda i: type(i[1]) is float or i[0] == "root" or is_str_float(str(i[1]).replace("humn", "0")), input.items())):
    for k, v in input.items():
        if k in ["root", "humn"]:
            continue

        try:
            for pk in v.split(" "):
                if pk in input.keys():
                    input[k] = v.replace(pk, f"( {input[pk]} )")
        except:
            pass

        try:
            input[k] = float(eval(v))
        except:
            pass

query = (input["root"].strip()
         .replace(str(input["root"].strip().split(" ")[0].strip()), str(input[input["root"].strip().split(" ")[0].strip()]))
         .replace(str(input["root"].strip().split(" ")[2].strip()), str(input[input["root"].strip().split(" ")[2].strip()]))
         .replace(" ", "")
         .replace(".0", "")
         .replace("humn", "x"))

print(f"2: Please paste the following into 'https://www.mathpapa.com/equation-solver/' and click 'Calculate It!':")
print(query)
webbrowser.open_new_tab("https://www.mathpapa.com/equation-solver/")
res = get_input("Answer: ")
print(f"2: {int(eval(res))}")
