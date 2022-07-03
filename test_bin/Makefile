TARGETS = add
MEM_CFG = memcfg/ramrom.cfg

all: clean $(TARGETS)

add: add.o
	ld65 -o add.bin -C $(MEM_CFG) add.o

add.o:
	ca65 add.s

clean:
	$(RM) *.bin *.o
