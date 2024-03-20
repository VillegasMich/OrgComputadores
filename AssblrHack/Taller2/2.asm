// El numero de entrada se ingresa en la posicion RAM[16]

  @R0
  D=M
  M=M-1
  A=M
  D=M
  @R6
  M=D
  @num // RAM[16]
  D=M
  @R1
  D=M+D  
  @temp // RAM[17]
  M=D
  @R6
  D=M 
  @temp
  A=M
  M=D
(END)
  @END
  0;JMP
