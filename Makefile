# Thanks https://stackoverflow.com/a/79685650
# Modified by me to also compile and link rust code

SRC_DIR   := src
HDR_DIR   := src
OUT_DIR   := out
BIN_DIR   := $(OUT_DIR)/bin
TARGET    := $(BIN_DIR)/pact

CC        := gcc
CFLAGS    := -Werror -Wall -Wextra -Wpedantic -I $(HDR_DIR)

SOURCES   := $(shell find $(SRC_DIR) -type f -name '*.c')
OBJS      := $(patsubst $(SRC_DIR)/%.c,$(OUT_DIR)/%.o,$(SOURCES))

RUSTC     := rustc
RUSTFLAGS := -C panic="abort" -C opt-level="s" -C lto="fat" -C codegen-units=1 -C strip="symbols" -C debuginfo=0 -C force-unwind-tables=no --print=native-static-libs --crate-type=staticlib
RUST_SRCS := $(shell find $(SRC_DIR) -type f -name '*.rs')
RUST_LIBS := $(patsubst $(SRC_DIR)/%.rs,$(OUT_DIR)/%.a,$(RUST_SRCS))

all: $(TARGET)

$(TARGET): $(OBJS) $(RUST_LIBS) | $(BIN_DIR)
		$(CC) $^ -o $@

$(OUT_DIR)/%.o: $(SRC_DIR)/%.c
		mkdir -p $(@D) && $(CC) $(CFLAGS) -c $^ -o $@

$(OUT_DIR)/%.a: $(SRC_DIR)/%.rs
		mkdir -p $(@D) && $(RUSTC) $(RUSTFLAGS) $^ -o $@

clean:
		rm -f $(TARGET) $(OBJS) $(RUST_LIBS)

$(BIN_DIR):
		mkdir -p $@
