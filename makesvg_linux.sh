echo -1 | sudo tee /proc/sys/kernel/perf_event_paranoid
cargo flamegraph --package binrunner
echo 4 | sudo tee /proc/sys/kernel/perf_event_paranoid