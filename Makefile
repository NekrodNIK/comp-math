BUILD_DIR = build

.PHONY: prepare clean task1

prepare:
	mkdir -p $(BUILD_DIR)

task1: task1.cc prepare
	$(CXX) $< -o $(BUILD_DIR)/task1 -std=c++23 -O3
	./build/task1

clean:
	rm -rf $(BUILD_DIR)
