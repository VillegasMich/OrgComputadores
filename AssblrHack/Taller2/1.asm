// El numero de entrada se ingresa en la posicion RAM[16]

  @16
  D=A
  @num //RAM[16]
  M=M+D
  A=M
  D=M //RAM[num+16]
  @R5
  M=D
  @R0
  A=M
  M=D
  @R0
  M=M+1
(END)
  @END
  0;JMP
