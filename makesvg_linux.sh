echo "DO NOT STOP MID PROCESS, IF YOU DO, RUN THE PROCESS BELOW"
echo "echo 4 | sudo tee /proc/sys/kernel/perf_event_paranoid"
echo -1 | sudo tee /proc/sys/kernel/perf_event_paranoid
cargo flamegraph --package binrunner
echo 4 | sudo tee /proc/sys/kernel/perf_event_paranoid