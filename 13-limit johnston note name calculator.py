def note_name(x,y):
  #### turns a ratio x/y into a 13-limit Johnston note name
  #### returns the prime limit larger than 13 implicated for such ratios
  #### see Fonville, John. “Ben Johnston's Extended Just Intonation: A Guide for Interpreters.” Perspectives of New Music 29/2 (1991): 106–137.
  #### https://www.jstor.org/stable/833435
  ##factor x and y
  i,j = 2,2
  factorsx, factorsy = [], []
  while i * i <= x:
    if x % i:
      i += 1
    else:
      x //= i
      factorsx.append(i)
  if x > 1:
    factorsx.append(x)
  px = {x:factorsx.count(x) for x in factorsx}
  while j * j <= y:
    if y % j:
      j += 1
    else:
      y //= j
      factorsy.append(j)
  if y > 1:
    factorsy.append(y)
  py = {y:factorsy.count(y) for y in factorsy}
  ##calculate prime limit
  limita = max(px.items())
  limitb = max(py.items())
  limit = max(limita, limitb)
  limit_num = limit[0]
  if limit_num > 13:
    limitmessage = str(limit_num) +'-limit pitch!'
    print('Sorry - that is a', limitmessage)
  else:
  ##determine 3-limit sf, pm
    x3, y3 = px.get(3,0), py.get(3,0)
    n3 = x3 - y3
    qn3 = n3
    qnn3 = n3
    pm3 = 0
    if n3 > 0:
      while qn3 > 0:
        k = qn3 - 1
        k = k % 7
        if k % 3 == 2:
          pm3 += 1
        qn3 -= 1
    elif n3 == 0:
      pass
    else:
      while qn3 < 0:
        k = qn3 + 1
        k = k % 7
        if k != 0:
          if k % 3 == 0:
            pm3 -= 1
        qn3 += 1
    sf3 = 0
    if n3 > 0:
      while qnn3 > 0:
        k = qnn3 - 1
        if k != 0:
          k = k % 7
          if k == 5:
            sf3+= 1
        qnn3 -= 1
    elif n3 == 0:
      pass
    else:
      while qnn3 < 0 :
        k = qnn3 % 7
        if k == 5:
          sf3-= 1
        qnn3 += 1
    ##determine 5-limit note name, sf, pm
    x5, y5 = px.get(5,0), py.get(5,0)
    n5 = x5 - y5
    shift3 = n3 % 7
    pn5 = 4*n5 % 7
    base_letters = 'CGDAEBF'
    letters = base_letters[shift3:]+base_letters[:shift3]
    s = list('xxxxxxx')
    for i in range(0,7):
      j = 2*i % 7
      s[j]=letters[i]
    sfs5 = ''.join(s)
    sf5, pm5 = 0, 0
    qn5 = n5
    if n5 > 0:
      while qn5 > 0:
        k = qn5 - 1
        k = k % 7
        if sfs5[k] == 'A' or sfs5[k] == 'E' or sfs5[k] == 'B':
          sf5 += 1
        elif sfs5[k] == 'D':
          sf5 += 1
          pm5 += 1
        qn5 -= 1 
    elif n5 == 0:
      pass
    else:
      while qn5 < 0:
        k = qn5 + 1
        k = k % 7
        if sfs5[k] == 'G' or sfs5[k] == 'C' or sfs5[k] == 'D':
          sf5 -= 1
        elif sfs5[k] == 'F':
          sf5 -= 1
          pm5 -= 1
        qn5 += 1
    ##determine 7-limit note name, sf, pm, and sL
    x7, y7 = px.get(7,0), py.get(7,0)
    n7 = x7 - y7
    pn7 = n7 % 7
    base_7letters = 'CBAGFED'
    preshift7 = {'C':0,'B':1,'A':2,'G':3,'F':4,'E':5,'D':6}
    shift7 = preshift7[letters[pn5]]
    letters7 = base_7letters[shift7:]+base_7letters[:shift7]
    sf7, pm7, sL = 0, 0, 0
    qn7 = n7
    if n7 > 0:
      while qn7 > 0:
        k = qn7 - 1
        k = k % 7
        if letters7[k] == 'G' or letters7[k] == 'B' or letters7[k] == 'D':
          pm7 += 1
          sL +=1
        elif letters7[k] == 'C' or letters7[k] == 'F':
          sf7 -= 1
          sL += 1
        else:
          sL += 1
        qn7 -= 1 
    elif n7 == 0:
      pass
    else:
      while qn7 < 0:
        k = qn7 + 1
        k = k % 7
        if letters7[k] == 'C' or letters7[k] == 'F' or letters7[k] == 'A':
          pm7 -= 1
          sL -=1
        elif letters7[k] == 'B' or letters7[k] == 'E':
          sf7 += 1
          sL -= 1
        else:
          sL -= 1
        qn7 += 1 
    ##determine 11-limit note name, sf, pm, and ud
    x11, y11 = px.get(11,0), py.get(11,0)
    n11 = x11 - y11
    pn11 = n11 % 7
    base_11letters = 'CFBEADG'
    preshift11 = {'C':0,'F':1,'B':2,'E':3,'A':4,'D':5,'G':6}
    shift11 = preshift11[letters7[pn7]]
    letters11 = base_11letters[shift11:]+base_11letters[:shift11]
    sf11, pm11, ud = 0, 0, 0
    qn11 = n11
    if n11 > 0:
      while qn11 > 0:
        k = qn11 - 1
        k = k % 7
        if letters11[k] == 'A':
          pm11 -= 1
          ud += 1
        elif letters11[k] == 'F':
          sf11 -= 1
          pm11 -= 1
          ud += 1
        else:
          ud += 1
        qn11 -= 1 
    elif n11 == 0:
      pass
    else:
      while qn11 < 0:
        k = qn11 + 1
        k = k % 7
        if letters11[k] == 'D':
          pm11 += 1
          ud -= 1
        elif letters11[k] == 'B':
          sf11 += 1
          pm11 += 1
          ud -= 1
        else:
          ud -= 1
        qn11 += 1 
    ##determine 13-limit note name, sf, pm, and e3
    x13, y13 = px.get(13,0), py.get(13,0)
    n13 = x13 - y13
    pn13 = n13 % 7
    base_13letters = 'CAFDBGE'
    preshift13 = {'C':0,'A':1,'F':2,'D':3,'B':4,'G':5,'E':6}
    shift13 = preshift13[letters11[pn11]]
    letters13 = base_13letters[shift13:]+base_13letters[:shift13]
    sf13, pm13, e3 = 0, 0, 0
    qn13 = n13
    if n13 > 0:
      while qn13 > 0:
        k = qn13 - 1
        k = k % 7
        if letters13[k] == 'C' or letters13[k] == 'D' or letters13[k] == 'G':
          sf13 -= 1
          e3 += 1
        elif letters13[k] == 'F':
          sf13 -= 1
          pm13 -= 1
          e3 += 1
        else:
          e3 += 1
        qn13 -= 1 
    elif n13 == 0:
      pass
    else:
      while qn13 < 0:
        k = qn13 + 1
        k = k % 7
        if letters13[k] == 'E' or letters13[k] == 'A' or letters13[k] == 'B':
          sf13 += 1
          e3 -= 1
        elif letters13[k] == 'D':
          sf13 += 1
          pm13 += 1
          e3 -= 1
        else:
          e3 -= 1
        qn13 += 1 
    ##produce a string of the note name, sf, pm, sL, ud, and e3 
    name = letters13[pn13]
    sf = sf3 + sf5 + sf7 + sf11 + sf13
    pm = pm3 + pm5 + pm7 + pm11 + pm13
    esf = sf
    epm = pm
    esL = sL
    eud = ud
    ee3 = e3
    while sf > 0:
      name += '#'
      sf -= 1
    while sf < 0:
      name += 'b'
      sf += 1
    while sL > 0:
      name += '7'
      sL -= 1
    while sL < 0:
      name += 'L'
      sL += 1
    while ud > 0:
      name += '^'
      ud -= 1
    while ud < 0:
      name += 'v'
      ud += 1
    while e3 > 0:
      name += '3'
      e3 -= 1
    while e3 < 0:
      name += 'e'
      e3 += 1
    while pm > 0:
      name += '+'
      pm -= 1
    while pm < 0:
      name += '-'
      pm += 1
    print(name)
    if abs(esf) > 4 or abs(epm) > 4 or abs(esL) > 4 or abs(eud) > 4 or abs(ee3) > 4:
      aka = '(' + letters13[pn13] + ' ' + str(esf) + ' ' + str(esL) + ' ' + str(eud) + ' ' + str(ee3) + ' ' + str(epm) +')'
      print(aka)
