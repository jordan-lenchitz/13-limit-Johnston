export function getPrimeFactors(n: number): Record<number, number> {
    const factors: Record<number, number> = {};
    let d = 2;
    let temp = Math.abs(n);
    if (temp === 0) return factors;
    while (d * d <= temp) {
        while (temp % d === 0) {
            factors[d] = (factors[d] || 0) + 1;
            temp /= d;
        }
        d++;
    }
    if (temp > 1) {
        factors[temp] = (factors[temp] || 0) + 1;
    }
    return factors;
}

export interface JohnstonComponents {
    letter: string;
    sf: number;
    pm: number;
    sL: number;
    ud: number;
    e3: number;
}

export function getJohnstonComponents(x: number, y: number): JohnstonComponents | string {
    if (y === 0) return "denominator cannot be zero";

    const px = getPrimeFactors(x);
    const py = getPrimeFactors(y);

    const allPrimes = new Set([...Object.keys(px), ...Object.keys(py)].map(Number));
    let limitNum = 1;
    if (allPrimes.size > 0) {
        limitNum = Math.max(...allPrimes);
    }

    if (limitNum > 13) {
        return `Sorry - that is a ${limitNum}-limit pitch!`;
    }

    const diff = (p: number) => (px[p] || 0) - (py[p] || 0);

    // 3-limit
    const n3 = diff(3);
    let pm3 = 0, sf3 = 0;
    
    let qn = n3;
    if (n3 > 0) {
        while (qn > 0) {
            if ((qn - 1) % 7 % 3 === 2) pm3++;
            qn--;
        }
    } else if (n3 < 0) {
        while (qn < 0) {
            let k = (qn + 1) % 7;
            if (k < 0) k += 7;
            if (k !== 0 && k % 3 === 0) pm3--;
            qn++;
        }
    }

    qn = n3;
    if (n3 > 0) {
        while (qn > 0) {
            if ((qn - 1) % 7 === 5) sf3++;
            qn--;
        }
    } else if (n3 < 0) {
        while (qn < 0) {
            let k = qn % 7;
            if (k < 0) k += 7;
            if (k === 5) sf3--;
            qn++;
        }
    }

    // 5-limit
    const n5 = diff(5);
    let shift3 = n3 % 7;
    if (shift3 < 0) shift3 += 7;
    const baseLetters = 'CGDAEBF';
    const letters = baseLetters.slice(shift3) + baseLetters.slice(0, shift3);

    const s = new Array(7);
    for (let i = 0; i < 7; i++) {
        s[(2 * i) % 7] = letters[i];
    }
    const sfs5 = s.join('');

    let pm5 = 0, sf5 = 0;
    qn = n5;
    if (n5 > 0) {
        while (qn > 0) {
            let k = (qn - 1) % 7;
            if (k < 0) k += 7;
            if ('AEB'.includes(sfs5[k])) {
                sf5++;
            } else if (sfs5[k] === 'D') {
                sf5++;
                pm5++;
            }
            qn--;
        }
    } else if (n5 < 0) {
        while (qn < 0) {
            let k = (qn + 1) % 7;
            if (k < 0) k += 7;
            if ('GCD'.includes(sfs5[k])) {
                sf5--;
            } else if (sfs5[k] === 'F') {
                sf5--;
                pm5--;
            }
            qn++;
        }
    }

    // 7-limit
    const n7 = diff(7);
    let pn5 = (4 * n5) % 7;
    if (pn5 < 0) pn5 += 7;
    
    const base7Letters = 'CBAGFED';
    const preshift7: Record<string, number> = { 'C': 0, 'B': 1, 'A': 2, 'G': 3, 'F': 4, 'E': 5, 'D': 6 };
    const shift7 = preshift7[letters[pn5]];
    const letters7 = base7Letters.slice(shift7) + base7Letters.slice(0, shift7);

    let sf7 = 0, pm7 = 0, sL = 0;
    qn = n7;
    if (n7 > 0) {
        while (qn > 0) {
            let k = (qn - 1) % 7;
            if (k < 0) k += 7;
            if ('GBD'.includes(letters7[k])) {
                pm7++;
                sL++;
            } else if ('CF'.includes(letters7[k])) {
                sf7--;
                sL++;
            } else {
                sL++;
            }
            qn--;
        }
    } else if (n7 < 0) {
        while (qn < 0) {
            let k = (qn + 1) % 7;
            if (k < 0) k += 7;
            if ('CFA'.includes(letters7[k])) {
                pm7--;
                sL--;
            } else if ('BE'.includes(letters7[k])) {
                sf7++;
                sL--;
            } else {
                sL--;
            }
            qn++;
        }
    }

    // 11-limit
    const n11 = diff(11);
    let pn7 = n7 % 7;
    if (pn7 < 0) pn7 += 7;
    const base11Letters = 'CFBEADG';
    const preshift11: Record<string, number> = { 'C': 0, 'F': 1, 'B': 2, 'E': 3, 'A': 4, 'D': 5, 'G': 6 };
    const shift11 = preshift11[letters7[pn7]];
    const letters11 = base11Letters.slice(shift11) + base11Letters.slice(0, shift11);

    let sf11 = 0, pm11 = 0, ud = 0;
    qn = n11;
    if (n11 > 0) {
        while (qn > 0) {
            let k = (qn - 1) % 7;
            if (k < 0) k += 7;
            if (letters11[k] === 'A') {
                pm11--;
                ud++;
            } else if (letters11[k] === 'F') {
                sf11--;
                pm11--;
                ud++;
            } else {
                ud++;
            }
            qn--;
        }
    } else if (n11 < 0) {
        while (qn < 0) {
            let k = (qn + 1) % 7;
            if (k < 0) k += 7;
            if (letters11[k] === 'D') {
                pm11++;
                ud--;
            } else if (letters11[k] === 'B') {
                sf11++;
                pm11++;
                ud--;
            } else {
                ud--;
            }
            qn++;
        }
    }

    // 13-limit
    const n13 = diff(13);
    let pn11 = n11 % 7;
    if (pn11 < 0) pn11 += 7;
    const base13Letters = 'CAFDBGE';
    const preshift13: Record<string, number> = { 'C': 0, 'A': 1, 'F': 2, 'D': 3, 'B': 4, 'G': 5, 'E': 6 };
    const shift13 = preshift13[letters11[pn11]];
    const letters13 = base13Letters.slice(shift13) + base13Letters.slice(0, shift13);

    let sf13 = 0, pm13 = 0, e3 = 0;
    qn = n13;
    if (n13 > 0) {
        while (qn > 0) {
            let k = (qn - 1) % 7;
            if (k < 0) k += 7;
            if ('CDG'.includes(letters13[k])) {
                sf13--;
                e3++;
            } else if (letters13[k] === 'F') {
                sf13--;
                pm13--;
                e3++;
            } else {
                e3++;
            }
            qn--;
        }
    } else if (n13 < 0) {
        while (qn < 0) {
            let k = (qn + 1) % 7;
            if (k < 0) k += 7;
            if ('EAB'.includes(letters13[k])) {
                sf13++;
                e3--;
            } else if (letters13[k] === 'D') {
                sf13++;
                pm13++;
                e3--;
            } else {
                e3--;
            }
            qn++;
        }
    }

    let pn13 = n13 % 7;
    if (pn13 < 0) pn13 += 7;
    const nameBase = letters13[pn13];
    const sf = sf3 + sf5 + sf7 + sf11 + sf13;
    const pm = pm3 + pm5 + pm7 + pm11 + pm13;

    return {
        letter: nameBase,
        sf,
        pm,
        sL,
        ud,
        e3
    };
}

export function noteName(x: number, y: number, quiet: boolean = false): string {
    const result = getJohnstonComponents(x, y);
    if (typeof result === 'string') {
        if (!quiet) console.log(result);
        return result;
    }

    const { letter, sf, pm, sL, ud, e3 } = result;
    
    let parts: string[] = [letter];
    if (sf > 0) parts.push('#'.repeat(sf));
    else if (sf < 0) parts.push('b'.repeat(Math.abs(sf)));
    if (sL > 0) parts.push('7'.repeat(sL));
    else if (sL < 0) parts.push('L'.repeat(Math.abs(sL)));
    if (ud > 0) parts.push('^'.repeat(ud));
    else if (ud < 0) parts.push('v'.repeat(Math.abs(ud)));
    if (e3 > 0) parts.push('3'.repeat(e3));
    else if (e3 < 0) parts.push('e'.repeat(Math.abs(e3)));
    if (pm > 0) parts.push('+'.repeat(pm));
    else if (pm < 0) parts.push('-'.repeat(Math.abs(pm)));
    
    let name = parts.join('');
    
    if (!quiet) {
        console.log(name);
    }
    
    if ([sf, pm, sL, ud, e3].some(v => Math.abs(v) > 4)) {
        const aka = `(${letter} ${sf} ${sL} ${ud} ${e3} ${pm})`;
        if (!quiet) console.log(aka);
        name += "\n" + aka;
    }
        
    return name;
}
