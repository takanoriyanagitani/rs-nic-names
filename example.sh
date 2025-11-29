#!/bin/sh

run_only_virtual() {
  echo "--- Running nic-names --only-virtual ---"
  ./target/release/nic-names --only-virtual
  echo
}

run_only_physical() {
  echo "--- Running nic-names --only-physical ---"
  ./target/release/nic-names --only-physical
  echo
}

run_conflicting_options_test() {
  echo "--- Testing conflicting options (should fail) ---"
  if ! ./target/release/nic-names --only-virtual --only-physical 2>&1 | grep -q 'error:'; then
    echo "Test failed: Conflicting options did not produce an error"
    exit 1
  fi
  echo "Test passed: Conflicting options correctly produced an error."
  echo
}

run_only_virtual
run_only_physical
run_conflicting_options_test
