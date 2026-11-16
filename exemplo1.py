def fatorial(x):
    r = 0
    r = 1
    if (x > 1):
        r = (x * fatorial((x - 1)))
    return r
def main():
    n = 0
    resultado = 0
    ok = False
    n = 6
    ok = True
    resultado = fatorial(n)
    if (resultado > 100):
        print(resultado)
    ok = False
    print(resultado)

if __name__ == "__main__":
    main()
