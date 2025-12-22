# Benchmarking

## Pre-Requisites
- [wrk](https://github.com/wg/wrk)

## Endpoint `/auth/register`
- Run the command
```bash
wrk -t4 -c100 -d30s --timeout 5s -s bench_register.lua http://localhost:8080/auth/register
```
- Results
```bash
Running 30s test @ http://localhost:8080/auth/register
  4 threads and 100 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     2.90s   610.97ms   4.92s    80.72%
    Req/Sec    10.66      7.10    40.00     65.72%
  979 requests in 30.03s, 186.43KB read
  Socket errors: connect 0, read 0, write 0, timeout 4
Requests/sec:     32.60
Transfer/sec:      6.21KB
```
```bash
Transactions
A transaction fingerprint represents one or more SQL transactions by replacing the literal values (e.g., numbers and strings) with underscores (_). To view additional details of a SQL transaction fingerprint, click the fingerprint to open the Transaction Details page.
Execution Count
Cumulative number of executions of transactions with this fingerprint within the specified time interval.The bar indicates the ratio of runtime success (gray) to retries (red) for the SQL statement fingerprint.
Application Name
Rows Processed
Average (mean) number of rows read from and written to disk per execution for transactions with this fingerprint within the specified time interval.
Bytes Read
Aggregation of all bytes read from disk across all operators for transactions with this fingerprint within the specified time interval.The gray bar indicates the mean number of bytes read from disk. The blue bar indicates one standard deviation from the mean.
Transaction Time
Average planning and execution time of transactions with this fingerprint within the specified time interval. The gray bar indicates the mean latency. The blue bar indicates one standard deviation from the mean.
Commit Latency
Average commit latency of this transaction. The gray bar indicates the mean latency. The blue bar indicates one standard deviation from the mean.
Contention Time
Average time transactions with this fingerprint were in contention with other transactions within the specified time interval.The gray bar indicates mean contention time. The blue bar indicates one standard deviation from the mean. This time does not include the time it takes to stream results back to the client.
SQL CPU Time
Average SQL CPU time spent executing within the specified time interval. It does not include SQL planning time nor KV execution time. The gray bar indicates mean SQL CPU time. The blue bar indicates one standard deviation from the mean.
Max Memory
Maximum memory used by a transaction with this fingerprint at any time during its execution within the specified time interval. The gray bar indicates the average max memory usage. The blue bar indicates one standard deviation from the mean.
Network
Amount of data transferred over the network (e.g., between regions and nodes) for transactions with this fingerprint within the specified time interval.If this value is 0, the statement was executed on a single node.The gray bar indicates the mean number of bytes sent over the network. The blue bar indicates one standard deviation from the mean.
Retries
Cumulative number of retries (including internal and automatic retries) of transactions with this fingerprint within the specified time interval.
Statements
The number of statements being executed on this transaction fingerprint
INSERT INTO users(user_id, userna...)
INSERT INTO users(user_id, username, password, email, birth_date, created_at) VALUES (_, _, _, _, _, now()) RETURNING user_id
6k
(unset)	0 Reads / 1 Writes	
0 B
4.2 ms
3.8 ms
4.2 µs
357.8 µs
10.0 KiB
0 B
0
1
```

## Hardware 
- CPU
```
Architecture:                x86_64
  CPU op-mode(s):            32-bit, 64-bit
  Address sizes:             39 bits physical, 48 bits virtual
  Byte Order:                Little Endian
CPU(s):                      16
  On-line CPU(s) list:       0-15
Vendor ID:                   GenuineIntel
  Model name:                13th Gen Intel(R) Core(TM) i5-13450HX
    CPU family:              6
    Model:                   183
    Thread(s) per core:      2
    Core(s) per socket:      10
    Socket(s):               1
    Stepping:                1
    CPU(s) scaling MHz:      43%
    CPU max MHz:             4600.0000
    CPU min MHz:             800.0000
    BogoMIPS:                5222.40
    Flags:                   fpu vme de pse tsc msr pae mce cx8 apic sep mtrr pge mca cmov pat pse36 clflush dts acpi mmx fxsr s
                             se sse2 ss ht tm pbe syscall nx pdpe1gb rdtscp lm constant_tsc art arch_perfmon pebs bts rep_good n
                             opl xtopology nonstop_tsc cpuid aperfmperf tsc_known_freq pni pclmulqdq dtes64 monitor ds_cpl vmx e
                             st tm2 ssse3 sdbg fma cx16 xtpr pdcm pcid sse4_1 sse4_2 x2apic movbe popcnt tsc_deadline_timer aes 
                             xsave avx f16c rdrand lahf_lm abm 3dnowprefetch cpuid_fault epb ssbd ibrs ibpb stibp ibrs_enhanced 
                             tpr_shadow flexpriority ept vpid ept_ad fsgsbase tsc_adjust bmi1 avx2 smep bmi2 erms invpcid rdseed
                              adx smap clflushopt clwb intel_pt sha_ni xsaveopt xsavec xgetbv1 xsaves split_lock_detect user_shs
                             tk avx_vnni dtherm ida arat pln pts hwp hwp_notify hwp_act_window hwp_epp hwp_pkg_req hfi vnmi umip
                              pku ospke waitpkg gfni vaes vpclmulqdq rdpid movdiri movdir64b fsrm md_clear serialize arch_lbr ib
                             t flush_l1d arch_capabilities
Virtualization features:     
  Virtualization:            VT-x
Caches (sum of all):         
  L1d:                       416 KiB (10 instances)
  L1i:                       448 KiB (10 instances)
  L2:                        9.5 MiB (7 instances)
  L3:                        20 MiB (1 instance)
NUMA:                        
  NUMA node(s):              1
  NUMA node0 CPU(s):         0-15
Vulnerabilities:             
  Gather data sampling:      Not affected
  Ghostwrite:                Not affected
  Indirect target selection: Not affected
  Itlb multihit:             Not affected
  L1tf:                      Not affected
  Mds:                       Not affected
  Meltdown:                  Not affected
  Mmio stale data:           Not affected
  Old microcode:             Not affected
  Reg file data sampling:    Mitigation; Clear Register File
  Retbleed:                  Not affected
  Spec rstack overflow:      Not affected
  Spec store bypass:         Mitigation; Speculative Store Bypass disabled via prctl
  Spectre v1:                Mitigation; usercopy/swapgs barriers and __user pointer sanitization
  Spectre v2:                Mitigation; Enhanced / Automatic IBRS; IBPB conditional; PBRSB-eIBRS SW sequence; BHI BHI_DIS_S
  Srbds:                     Not affected
  Tsa:                       Not affected
  Tsx async abort:           Not affected
  Vmscape:                   Mitigation; IBPB before exit to userspace
```

- GPU
```
Driver Version                                         : 580.119.02
CUDA Version                                           : 13.0

Attached GPUs                                          : 1
GPU 00000000:01:00.0
    Product Name                                       : NVIDIA GeForce RTX 3050 6GB Laptop GPU
    Product Brand                                      : GeForce
    Product Architecture                               : Ampere
    Display Mode                                       : Requested functionality has been deprecated
    Display Attached                                   : No
    Display Active                                     : Disabled
    Persistence Mode                                   : Disabled
    Addressing Mode                                    : None
    MIG Mode
        Current                                        : N/A
        Pending                                        : N/A
    Accounting Mode                                    : Disabled
    Accounting Mode Buffer Size                        : 4000
    Driver Model
        Current                                        : N/A
        Pending                                        : N/A
    Serial Number                                      : N/A
    GPU UUID                                           : GPU-c2511218-82b7-8df6-6091-4dda966d3868
    GPU PDI                                            : 0x498c820810d7a8e7
    Minor Number                                       : 0
    VBIOS Version                                      : 94.07.8B.00.4C
    MultiGPU Board                                     : No
    Board ID                                           : 0x100
    Board Part Number                                  : N/A
    GPU Part Number                                    : 25EC-730-A1
    FRU Part Number                                    : N/A
    Platform Info
        Chassis Serial Number                          : N/A
        Slot Number                                    : N/A
        Tray Index                                     : N/A
        Host ID                                        : N/A
        Peer Type                                      : N/A
        Module Id                                      : 1
        GPU Fabric GUID                                : N/A
    Inforom Version
        Image Version                                  : G001.0000.94.01
        OEM Object                                     : 2.0
        ECC Object                                     : N/A
        Power Management Object                        : N/A
    Inforom BBX Object Flush
        Latest Timestamp                               : N/A
        Latest Duration                                : N/A
    GPU Operation Mode
        Current                                        : N/A
        Pending                                        : N/A
    GPU C2C Mode                                       : N/A
    GPU Virtualization Mode
        Virtualization Mode                            : None
        Host VGPU Mode                                 : N/A
        vGPU Heterogeneous Mode                        : N/A
    GPU Recovery Action                                : None
    GSP Firmware Version                               : 580.119.02
    IBMNPU
        Relaxed Ordering Mode                          : N/A
    PCI
        Bus                                            : 0x01
        Device                                         : 0x00
        Domain                                         : 0x0000
        Base Classcode                                 : 0x3
        Sub Classcode                                  : 0x0
        Device Id                                      : 0x25EC10DE
        Bus Id                                         : 00000000:01:00.0
        Sub System Id                                  : 0x3D0617AA
        GPU Link Info
            PCIe Generation
                Max                                    : 4
                Current                                : 4
                Device Current                         : 4
                Device Max                             : 4
                Host Max                               : 5
            Link Width
                Max                                    : 16x
                Current                                : 8x
        Bridge Chip
            Type                                       : N/A
            Firmware                                   : N/A
        Replays Since Reset                            : 0
        Replay Number Rollovers                        : 0
        Tx Throughput                                  : 488 KB/s
        Rx Throughput                                  : 390 KB/s
        Atomic Caps Outbound                           : N/A
        Atomic Caps Inbound                            : N/A
    Fan Speed                                          : N/A
    Performance State                                  : P0
    Clocks Event Reasons
        Idle                                           : Active
        Applications Clocks Setting                    : Not Active
        SW Power Cap                                   : Not Active
        HW Slowdown                                    : Not Active
            HW Thermal Slowdown                        : Not Active
            HW Power Brake Slowdown                    : Not Active
        Sync Boost                                     : Not Active
        SW Thermal Slowdown                            : Not Active
        Display Clock Setting                          : Not Active
    Clocks Event Reasons Counters
        SW Power Capping                               : 38610 us
        Sync Boost                                     : 0 us
        SW Thermal Slowdown                            : 0 us
        HW Thermal Slowdown                            : 0 us
        HW Power Braking                               : 0 us
    Sparse Operation Mode                              : N/A
    FB Memory Usage
        Total                                          : 6144 MiB
        Reserved                                       : 340 MiB
        Used                                           : 12 MiB
        Free                                           : 5793 MiB
    BAR1 Memory Usage
        Total                                          : 8192 MiB
        Used                                           : 13 MiB
        Free                                           : 8179 MiB
    Conf Compute Protected Memory Usage
        Total                                          : 0 MiB
        Used                                           : 0 MiB
        Free                                           : 0 MiB
    Compute Mode                                       : Default
    Utilization
        GPU                                            : 6 %
        Memory                                         : 0 %
        Encoder                                        : 0 %
        Decoder                                        : 0 %
        JPEG                                           : 0 %
        OFA                                            : 0 %
    Encoder Stats
        Active Sessions                                : 0
        Average FPS                                    : 0
        Average Latency                                : 0
    FBC Stats
        Active Sessions                                : 0
        Average FPS                                    : 0
        Average Latency                                : 0
    DRAM Encryption Mode
        Current                                        : N/A
        Pending                                        : N/A
    ECC Mode
        Current                                        : N/A
        Pending                                        : N/A
    ECC Errors
        Volatile
            SRAM Correctable                           : N/A
            SRAM Uncorrectable Parity                  : N/A
            SRAM Uncorrectable SEC-DED                 : N/A
            DRAM Correctable                           : N/A
            DRAM Uncorrectable                         : N/A
        Aggregate
            SRAM Correctable                           : N/A
            SRAM Uncorrectable Parity                  : N/A
            SRAM Uncorrectable SEC-DED                 : N/A
            DRAM Correctable                           : N/A
            DRAM Uncorrectable                         : N/A
            SRAM Threshold Exceeded                    : N/A
        Aggregate Uncorrectable SRAM Sources
            SRAM L2                                    : N/A
            SRAM SM                                    : N/A
            SRAM Microcontroller                       : N/A
            SRAM PCIE                                  : N/A
            SRAM Other                                 : N/A
        Channel Repair Pending                         : No
        TPC Repair Pending                             : No
    Retired Pages
        Single Bit ECC                                 : N/A
        Double Bit ECC                                 : N/A
        Pending Page Blacklist                         : N/A
    Remapped Rows                                      : N/A
    Temperature
        GPU Current Temp                               : 42 C
        GPU T.Limit Temp                               : N/A
        GPU Shutdown Temp                              : 100 C
        GPU Slowdown Temp                              : 97 C
        GPU Max Operating Temp                         : 105 C
        GPU Target Temperature                         : 87 C
        Memory Current Temp                            : N/A
        Memory Max Operating Temp                      : N/A
    GPU Power Readings
        Average Power Draw                             : 1.00 W
        Instantaneous Power Draw                       : 749.67 W
        Current Power Limit                            : 95.00 W
        Requested Power Limit                          : 95.00 W
        Default Power Limit                            : 60.00 W
        Min Power Limit                                : 1.00 W
        Max Power Limit                                : 95.00 W
    GPU Memory Power Readings 
        Average Power Draw                             : N/A
        Instantaneous Power Draw                       : N/A
    Module Power Readings
        Average Power Draw                             : N/A
        Instantaneous Power Draw                       : N/A
        Current Power Limit                            : N/A
        Requested Power Limit                          : N/A
        Default Power Limit                            : N/A
        Min Power Limit                                : N/A
        Max Power Limit                                : N/A
    Power Smoothing                                    : N/A
    Workload Power Profiles
        Requested Profiles                             : N/A
        Enforced Profiles                              : N/A
    Clocks
        Graphics                                       : 1492 MHz
        SM                                             : 1492 MHz
        Memory                                         : 7001 MHz
        Video                                          : 1312 MHz
    Applications Clocks
        Graphics                                       : N/A
        Memory                                         : N/A
    Default Applications Clocks
        Graphics                                       : N/A
        Memory                                         : N/A
    Deferred Clocks
        Memory                                         : N/A
    Max Clocks
        Graphics                                       : 2100 MHz
        SM                                             : 2100 MHz
        Memory                                         : 7001 MHz
        Video                                          : 1950 MHz
    Max Customer Boost Clocks
        Graphics                                       : N/A
    Clock Policy
        Auto Boost                                     : N/A
        Auto Boost Default                             : N/A
    Fabric
        State                                          : N/A
        Status                                         : N/A
        CliqueId                                       : N/A
        ClusterUUID                                    : N/A
        Health
            Summary                                    : N/A
            Bandwidth                                  : N/A
            Route Recovery in progress                 : N/A
            Route Unhealthy                            : N/A
            Access Timeout Recovery                    : N/A
            Incorrect Configuration                    : N/A
    Processes
        GPU instance ID                   : N/A
        Compute instance ID               : N/A
        Process ID                        : 2355
            Type                          : G
            Name                          : /usr/bin/gnome-shell
            Used GPU Memory               : 1 MiB
    Capabilities
        EGM                                            : disabled


```