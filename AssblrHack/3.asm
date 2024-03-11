// Initialize variables
// Multiply two numbers
  @sum // @16
  M=0 // @16 = 0
  @R0 
  D=M // D = RAM[0]
  @i // @17
  M=D // RAM[17] = D; i=D
(LOOP)
  @i
  MD=M-1 // Decrement counter
  @FINISH
  D;JLT
  @R1
  D=M
  @sum
  M=D+M
  @LOOP
  0;JMP
(FINISH)
  @sum
  D=M
  @R2
  M=D // RAM[2] = @sum
(END)
  @END
  0;JMP

