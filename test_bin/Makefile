# You'll need cc65 installed to compile the example assembler programs
# `brew install cc65` on a mac 

TARGETS = add
MEM_CFG = memcfg/ramrom.cfg

all: clean $(TARGETS)

add: add.o
	ld65 -o add.bin -C $(MEM_CFG) add.o

add.o:
	ca65 add.s

clean:
	$(RM) *.bin *.o
