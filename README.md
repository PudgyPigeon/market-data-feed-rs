# How to Run

After the nix-shell starts up you should see a prompt telling you to run the "dev" command.

```
dev app/assets/mdf-kospi200.20110216-0.pcap 
```
Enter the above command to run the command with the associated `.pcap` file.

You can also add an optional `-r` flag before or after the input path to reverse the order.

```
dev -r app/assets/mdf-kospi200.20110216-0.pcap 

# Or

dev app/assets/mdf-kospi200.20110216-0.pcap -r

# Both work!
```
