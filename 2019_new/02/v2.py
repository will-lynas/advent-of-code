with open("input") as f:
    original_memory = [int(n) for n in f.read().strip().split(",")]

found = False
for noun in range(100):
    for verb in range(100):
        memory = original_memory[:]
        memory[1] = noun
        memory[2] = verb
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
        if memory[0] == 19690720:
            print(100*noun+verb)
            found = True
            break
    if found:
        break
