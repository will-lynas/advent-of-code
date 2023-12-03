from hashlib import md5

key = "ckczppom"
i = 0
while True:
    res = md5((key+str(i)).encode("utf-8")).hexdigest()
    if all(c == "0" for c in res[:5]):
        print(i)
        break
    i += 1
