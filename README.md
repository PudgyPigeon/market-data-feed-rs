## Notes TODO
Clean up codebase - it's a bit ugly

# How to Run
If you have Nix and DirEnv on your Linux or WSL distro ready to go, the nix-shell will start automatically.

If not, you might just have to run `nix-shell` in your terminal at the root of this repo.

After the nix-shell starts up you should see a prompt telling you to run the "dev" or "release" command.

There are hardcoded defaults inside of the various config objects but you can run the program as follows:
- Note: Running just cargo build or cargo run without Nix would probably work but not tested

```
# In your terminal
dev app/assets/mdf-kospi200.20110216-0.pcap 

# OR 

dev 
# If you just want to use the default hardcoded struct field
```
Enter the above command to run the command with the associated `.pcap` file.

You can also add an optional `-r` flag before or after the input path to reorder.

```
dev -r app/assets/mdf-kospi200.20110216-0.pcap 

# Or

dev app/assets/mdf-kospi200.20110216-0.pcap -r

# Or

dev -r

# All three work!
```

You can also run the release build but it's fairly similar in performance:
```
release
# OR
release -r
# OR
release > /dev/null
```

## How to output to CSV and check line numbers
```
dev > output.csv
dev -r > output.csv

## And then
wc -l output.csv
```
Should be 16016

## How to run Perf
```
perf stat release > /dev/null # For no printing

perf stat release -r # For printing etc

taskset -c 0 ./target/release/market-data-feed-rs > /dev/null
Total execution time: 1.895392ms

```

## Running target/release
```
./target/release/market-data-feed-rs > /dev/null
Total execution time: 2.561191ms

perf stat ./target/release/market-data-feed-rs > /dev/null
```

## Stress-NG
```
# 1. Start the 'heater' on Core 1 (different from your app's core)
taskset -c 1 stress-ng --cpu 1 & 

# 2. Run your app on Core 0
taskset -c 0 ./target/release/market-data-feed-rs > /dev/null

# 3. Stop the heater
pkill stress-ng
```