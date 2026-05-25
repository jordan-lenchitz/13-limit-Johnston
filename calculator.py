import argparse
import sys
from collections import Counter
from typing import Dict, Tuple, List, Union

def get_prime_factors(n: int) -> Dict[int, int]:
    """returns dictionary of prime factors & their counts"""
    factors = []
    d = 2
    temp = abs(n)
    if temp == 0:
        return {}
    while d * d <= temp:
        while temp % d == 0:
            factors.append(d)
            temp //= d
        d += 1
    if temp > 1:
        factors.append(temp)
    return dict(Counter(factors))

def get_johnston_components(x: int, y: int) -> Union[str, Tuple[str, int, int, int, int, int]]:
    """
    calculates the Johnston notation components for ratio x/y
    returns either a message string if limit > 13, or a tuple of (letter, sf, pm, sL, ud, e3)
    """
    if y == 0:
        return "denominator cannot be zero"
    
    px = get_prime_factors(x)
    py = get_prime_factors(y)

    all_primes = set(px.keys()) | set(py.keys())
    if not all_primes:
        limit_num = 1
    else:
        # filter out 1 if it's there just in case
        limit_num = max(all_primes)

    if limit_num > 13:
        return f"Sorry - that is a {limit_num}-limit pitch!"

    # 3-limit
    n3 = px.get(3, 0) - py.get(3, 0)
    
    def calc_3_limit(n3):
        pm, sf = 0, 0
        # pm3
        qn = n3
        if n3 > 0:
            while qn > 0:
                if (qn - 1) % 7 % 3 == 2:
                    pm += 1
                qn -= 1
        elif n3 < 0:
            while qn < 0:
                k = (qn + 1) % 7
                if k != 0 and k % 3 == 0:
                    pm -= 1
                qn += 1

        # sf3
        qn = n3
        if n3 > 0:
            while qn > 0:
                if (qn - 1) % 7 == 5:
                    sf += 1
                qn -= 1
        elif n3 < 0:
            while qn < 0:
                if qn % 7 == 5:
                    sf -= 1
                qn += 1
        return pm, sf

    pm3, sf3 = calc_3_limit(n3)

    # 5-limit
    n5 = px.get(5, 0) - py.get(5, 0)
    shift3 = n3 % 7
    base_letters = 'CGDAEBF'
    letters = base_letters[shift3:] + base_letters[:shift3]

    s = [''] * 7
    for i in range(7):
        s[(2 * i) % 7] = letters[i]
    sfs5 = ''.join(s)

    pm5, sf5 = 0, 0
    qn = n5
    if n5 > 0:
        while qn > 0:
            k = (qn - 1) % 7
            if sfs5[k] in 'AEB':
                sf5 += 1
            elif sfs5[k] == 'D':
                sf5 += 1
                pm5 += 1
            qn -= 1
    elif n5 < 0:
        while qn < 0:
            k = (qn + 1) % 7
            if sfs5[k] in 'GCD':
                sf5 -= 1
            elif sfs5[k] == 'F':
                sf5 -= 1
                pm5 -= 1
            qn += 1

    # 7-limit
    n7 = px.get(7, 0) - py.get(7, 0)
    pn5 = (4 * n5) % 7
    
    base_7letters = 'CBAGFED'
    preshift7 = {'C':0,'B':1,'A':2,'G':3,'F':4,'E':5,'D':6}
    shift7 = preshift7[letters[pn5]]
    letters7 = base_7letters[shift7:] + base_7letters[:shift7]

    sf7, pm7, sL = 0, 0, 0
    qn = n7
    if n7 > 0:
        while qn > 0:
            k = (qn - 1) % 7
            if letters7[k] in 'GBD':
                pm7 += 1
                sL += 1
            elif letters7[k] in 'CF':
                sf7 -= 1
                sL += 1
            else:
                sL += 1
            qn -= 1
    elif n7 < 0:
        while qn < 0:
            k = (qn + 1) % 7
            if letters7[k] in 'CFA':
                pm7 -= 1
                sL -= 1
            elif letters7[k] in 'BE':
                sf7 += 1
                sL -= 1
            else:
                sL -= 1
            qn += 1

    # 11-limit
    n11 = px.get(11, 0) - py.get(11, 0)
    pn7 = n7 % 7
    base_11letters = 'CFBEADG'
    preshift11 = {'C':0,'F':1,'B':2,'E':3,'A':4,'D':5,'G':6}
    shift11 = preshift11[letters7[pn7]]
    letters11 = base_11letters[shift11:] + base_11letters[:shift11]

    sf11, pm11, ud = 0, 0, 0
    qn = n11
    if n11 > 0:
        while qn > 0:
            k = (qn - 1) % 7
            if letters11[k] == 'A':
                pm11 -= 1
                ud += 1
            elif letters11[k] == 'F':
                sf11 -= 1
                pm11 -= 1
                ud += 1
            else:
                ud += 1
            qn -= 1
    elif n11 < 0:
        while qn < 0:
            k = (qn + 1) % 7
            if letters11[k] == 'D':
                pm11 += 1
                ud -= 1
            elif letters11[k] == 'B':
                sf11 += 1
                pm11 += 1
                ud -= 1
            else:
                ud -= 1
            qn += 1

    # 13-limit
    n13 = px.get(13, 0) - py.get(13, 0)
    pn11 = n11 % 7
    base_13letters = 'CAFDBGE'
    preshift13 = {'C':0,'A':1,'F':2,'D':3,'B':4,'G':5,'E':6}
    shift13 = preshift13[letters11[pn11]]
    letters13 = base_13letters[shift13:] + base_13letters[:shift13]

    sf13, pm13, e3 = 0, 0, 0
    qn = n13
    if n13 > 0:
        while qn > 0:
            k = (qn - 1) % 7
            if letters13[k] in 'CDG':
                sf13 -= 1
                e3 += 1
            elif letters13[k] == 'F':
                sf13 -= 1
                pm13 -= 1
                e3 += 1
            else:
                e3 += 1
            qn -= 1
    elif n13 < 0:
        while qn < 0:
            k = (qn + 1) % 7
            if letters13[k] in 'EAB':
                sf13 += 1
                e3 -= 1
            elif letters13[k] == 'D':
                sf13 += 1
                pm13 += 1
                e3 -= 1
            else:
                e3 -= 1
            qn += 1

    pn13 = n13 % 7
    name_base = letters13[pn13]
    sf = sf3 + sf5 + sf7 + sf11 + sf13
    pm = pm3 + pm5 + pm7 + pm11 + pm13

    return name_base, sf, pm, sL, ud, e3

def note_name(x: int, y: int, quiet: bool = False) -> str:
    """
    returns the Johnston note name for ratio x/y
    
    :param x: Numerator
    :param y: Denominator
    :param quiet: If True, suppresses printing to stdout.
    :return: The note name string.
    """
    result = get_johnston_components(x, y)
    if isinstance(result, str):
        if not quiet:
            print(result)
        return result

    name_base, sf, pm, sL, ud, e3 = result
    
    esf, epm, esL, eud, ee3 = sf, pm, sL, ud, e3
    
    parts = [name_base]
    if sf > 0: parts.append('#' * sf)
    elif sf < 0: parts.append('b' * abs(sf))
    if sL > 0: parts.append('7' * sL)
    elif sL < 0: parts.append('L' * abs(sL))
    if ud > 0: parts.append('^' * ud)
    elif ud < 0: parts.append('v' * abs(ud))
    if e3 > 0: parts.append('3' * e3)
    elif e3 < 0: parts.append('e' * abs(e3))
    if pm > 0: parts.append('+' * pm)
    elif pm < 0: parts.append('-' * abs(pm))
    
    name = "".join(parts)
    
    if not quiet:
        print(name)
    
    if any(abs(v) > 4 for v in [esf, epm, esL, eud, ee3]):
        aka = f"({name_base} {esf} {esL} {eud} {ee3} {epm})"
        if not quiet:
            print(aka)
        name += "\n" + aka
        
    return name

def main():
    parser = argparse.ArgumentParser(description="Convert a ratio to a Ben Johnston note name.")
    parser.add_argument("ratio", type=str, help="The ratio to convert (e.g., '49/55' or '49 55')")
    parser.add_argument("denominator", type=int, nargs='?', help="The denominator if ratio is just the numerator")
    
    args = parser.parse_args()
    
    if args.denominator is not None:
        try:
            x = int(args.ratio)
            y = args.denominator
        except ValueError:
            print("Invalid input. Please provide two integers.")
            sys.exit(1)
    else:
        if '/' in args.ratio:
            try:
                x, y = map(int, args.ratio.split('/'))
            except ValueError:
                print("Invalid ratio format. Use x/y.")
                sys.exit(1)
        elif ' ' in args.ratio:
            try:
                x, y = map(int, args.ratio.split())
            except ValueError:
                print("Invalid ratio format. Use 'x y'.")
                sys.exit(1)
        else:
            print("Please provide a ratio (e.g., 49/55) or two integers.")
            sys.exit(1)
            
    note_name(x, y)

if __name__ == "__main__":
    main()
