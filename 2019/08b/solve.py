#!/usr/bin/env python2

with open("input.txt") as f:
	inputs = f.readline().strip()

layer_size = 25*6
layers = [list(inputs[i*layer_size:(i+1)*layer_size]) for i in range(len(inputs)/layer_size)]

pixels = layers[0]

for layer in layers:
	for i in range(len(pixels)):
		if pixels[i] == "2":
			pixels[i] = layer[i]

pixels = [pixels[i*25:(i+1)*25] for i in range(6)]
print "\n".join(["".join(row).replace("0"," ") for row in pixels]) #CJZLP