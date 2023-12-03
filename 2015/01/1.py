file_name = "input"
with open(file_name) as f:
    line = f.read().strip()
    print(line.count("(")-line.count(")"))
