BUILD_DIR = build
CARGO = cargo

.PHONY: prepare clean task1

prepare:
	mkdir -p $(BUILD_DIR)

task1: task1.cc prepare
	$(CXX) $< -o $(BUILD_DIR)/task1 -std=c++23 -O3
	./build/task1

task2: task2.rs prepare
	$(CARGO) run task2
	
task3: task3.rs prepare
	$(CARGO) run task3

clean:
	rm -rf $(BUILD_DIR)
