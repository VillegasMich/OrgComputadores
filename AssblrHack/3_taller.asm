  @num //RAM[16]
  D=M
  @R1
  M=M+D //RAM[1] + RAM[16]
  A=M
  D=M
  @acc //RAM[17]
  M=D
  @i //RAM[18]
  M=0
  @quo
  M=0 //quotent=0
(LOOP)
  @i
  D=M
  @10
  D=D-A
  @FINISH
  D;JGE
  @10
  D=D+A
  @num
  M=M+1
  @i
  M=M+1
  @LOOP
  0;JMP
(FINISH)
  @10
  D=A
  @acc
  M=M-D
  D=M
  @FINAL
  D;JLT
  @quo
  M=M+1
  D=M
  @FINISH
  0;JMP
(FINAL)
  @10
  D=D+A
  @quo
  D=M
  @acc
  M=D
(END)
  @END
  0;JMP

