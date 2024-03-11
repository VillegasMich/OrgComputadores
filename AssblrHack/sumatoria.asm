  @sum
  M=0
  @i
  M=1
(LOOP)
  @i
  D=M
  @100
  D=D-A // i-100
  @FINISH
  D;JGT
  @100
  D=D+A // i+100
  @i
  M=M+1 // i++
  @sum
  M=D+M // sum+=i
  @LOOP
  0;JMP
(FINISH)
  @sum
  D=M
  @R1
  M=D
(END)
  @END
  0;JMP

