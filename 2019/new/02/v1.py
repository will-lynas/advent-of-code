with open("input") as f:
    memory = [int(n) for n in f.read().strip().split(",")]

memory[1] = 12
memory[2] = 2
ip = 0
while (opcode := memory[ip]) != 99:
    if opcode == 1:
        # Add
        memory[memory[ip+3]] = memory[memory[ip+1]] + memory[memory[ip+2]]
        ip += 4
    elif opcode == 2:
        # Multiply
        memory[memory[ip+3]] = memory[memory[ip+1]] * memory[memory[ip+2]]
        ip += 4
    else:
        raise
print(memory[0])
