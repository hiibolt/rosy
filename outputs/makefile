CXX = g++
CXXFLAGS = -Wall -Werror -ansi -pedantic -std=c++14 -g 
OUTPUT_NAME = result
ENTRYPOINT = main

$(ENTRYPOINT): $(ENTRYPOINT).o
	$(CXX) $(CXXFLAGS) -o $(OUTPUT_NAME) $(ENTRYPOINT).o

$(ENTRYPOINT).o: $(ENTRYPOINT).cpp

debug:
	gdb --silent $(OUTPUT_NAME)

run:
	./$(OUTPUT_NAME)

clean:
	rm -f *.o
	rm -f $(OUTPUT_NAME)