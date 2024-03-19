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
  M=M+D // RAM[1] + RAM[16]  
  @R6
  D=M // RAM[6]
  @R1
  A=M
  M=D
  @num
  D=M
  @R1
  M=M-D
(END)
  @END
  0;JMP
