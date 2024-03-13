// Cree un programa en hack que lea un numero de la posición 
// de memoria indicada en la ram 0
// Y lo almacene en la posición de memoria r13, luego, vaya a la posición de 
// memoria siguiente a la indicada en la ram0 y súmele ese valor a r13
// Por último, almacene el resultado en la posición indicada por ram(0)
  @R0
  A=M
  D=M
  @R13
  M=D
  @R0
  A=M+1
  D=M
  @R13
  M=M+D
  D=M
  @R0
  A=M
  M=D
(END)
  @END
  0;JMP
  
