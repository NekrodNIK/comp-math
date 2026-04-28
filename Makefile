BUILD_DIR = build
CARGO = cargo
GHC = ghc

.PHONY: prepare clean task1 task2 task3 task4 task5

prepare:
	mkdir -p $(BUILD_DIR)

task1: task1.cc prepare
	$(CXX) $< -o $(BUILD_DIR)/task1 -std=c++23 -O3
	./build/task1

task2: task2.rs prepare
	$(CARGO) run --bin task2
	
task3: task3.rs prepare
	$(CARGO) run --bin task3

task4: task4.rs prepare
	$(CARGO) run --bin task4 --release

task5: task5.hs prepare
	$(GHC) $< -odir $(BUILD_DIR) -hidir $(BUILD_DIR) -o $(BUILD_DIR)/task5
	./build/task5

clean:
	rm -rf $(BUILD_DIR)
