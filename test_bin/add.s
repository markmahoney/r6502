CLC           ; Clear carry bit
CLD           ; Clear decimal bit
ADR1 = $6100  ; Set addresses
ADR2 = $6101
ADR3 = $6102
LDA #01
STA ADR1      ; Load ADR1 with the value 1
LDA #02
STA ADR2      ; Load ADR2 with the value 2
LDA ADR1      ; Load ADR1 into accumulator
ADC ADR2      ; Add ADR2 with accumulator
STA ADR3      ; Transfer accumulator to ADR3
RTS
