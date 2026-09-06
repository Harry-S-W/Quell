CC = clang

TARGET = quellc

SRCS = src/main.s src/tokenizer.s src/parser.s src/primitives.s

all: $(TARGET)

$(TARGET):
	$(CC) $(SRCS) -o $(TARGET)

clean:
	rm -f $(TARGET)