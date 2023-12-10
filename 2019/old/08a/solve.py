#!/usr/bin/env python2

with open("input.txt") as f:
	inputs = f.readline().strip()

layer_size = 25*6
layers = [inputs[i*layer_size:(i+1)*layer_size] for i in range(len(inputs)/layer_size)]

maximum = layers[0]

for layer in layers:
	if layer.count("0") < maximum.count("0"):
		maximum = layer

print maximum.count("1")*maximum.count("2")