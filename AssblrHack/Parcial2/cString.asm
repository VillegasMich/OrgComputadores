(LOOP)
  @R0
  A=M
  D=M
  @END
  D;JEQ
  @R1
  D=M
  @R0
  A=M
  D=M-D
  @REPLACE
  D;JEQ
  @R0
  M=M+1
  @LOOP
  0;JMP
(REPLACE)
  @R2
  D=M
  @R0
  A=M
  M=D
  @R0
  M=M+1
  @LOOP
  0;JMP
(END)
  @END
  0;JMP
