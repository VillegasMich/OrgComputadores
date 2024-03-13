// Cree un programa en Hack que lea un numero
// de la posicion de memoria 0.
// Determine si el numero es par o impar.
// Si es par coloque un 0 en la posicion de 
// memoria 1, si es impar coloque un -1.

  @R0
  D=M
 (LOOP)
  @2
  D=D-A
  @PAR
  D;JEQ
  @IMPAR
  D;JLT
  @R0
  M=D
  @LOOP
  0;JMP
(PAR)
  @R1
  M=0
  @END
  0;JMP
(IMPAR)
  @R1
  M=-1
  @END
  0;JMP
(END)
  @END
  0;JMP
