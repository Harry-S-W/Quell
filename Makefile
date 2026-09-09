CC = clang

TARGET = quellc

SRCS = src/main.S src/tokenizer.S src/parser.S src/primitives.S

all: $(TARGET)

$(TARGET):
	$(CC) $(SRCS) -o $(TARGET)

clean:
	rm -f $(TARGET)