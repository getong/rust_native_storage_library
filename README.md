# rust_native_storage_library
A library which compiles to `.so` and `.dylib` and facilitates the native storage (and retrieval) of key:value pairs on the host system

# Deployment
To use this software, please compile into libraries (for which SecondState WebAssembly runtime software can interoperate). These libraries will reside on the same machine as SSVM suite. Here is an example of how to set up a host machine (in this case, an EC2 instance with sufficient storage to facilitate SSVM and a database etc.).

# Setting up a machine and OS
This installation uses the AWS i3.xlarge instance because it has a solid state hard drive which is large enough
- i3.xlarge ($0.374 per Hour)
- 1 x 950 NVMe SSD Storage
- 4 CPU
- 30.5 GB RAM

# Software (OS)
```
sudo apt-get update
sudo apt-get -y upgrade
sudo apt-get install -y build-essential
```

# Auxillary hardware
View NVMe volume (which is not yet mounted/mapped/formatted)
```bash
lsblk
```
There is a ~300Gb drive with the name of nvme0n1
```bash
nvme0n1     259:2    0 279.4G  0 disk 
```
Create a file system
```bash
sudo mkfs -t ext4 /dev/nvme0n1 
```
Part of the output from the above mkfs command will include the Filesystem UUID. Cut and paste this UUID because it will be used in an upcoming command.
```bash
Filesystem UUID: 6f6177fe-947a-46a2-b593-c36dfaaa8407
```
Create an easily accesible mount point on the main drive (where the operating system runs) and then set the permissions of this mount point to the ubuntu user.
```bash
sudo mkdir /media/nvme
sudo chown -R ubuntu:ubuntu /media/nvme/
```
Ensure that this drive is mounted each time the system is restarted. Add this line to the */etc/fstab* file (remember the UUID from the previous step?).
```bash
UUID=553acce1-4e42-4ea3-85da-c57ed3dd82b5 /media/nvme ext4 defaults 0 0
```
Once the above commands have succeeded, reboot the instance.
```bash
sudo shutdown -r now
```
After the reboot, see the mounted ~300Gb NVMe SSD using the df command
```bash
df -h
/dev/nvme0n1    275G   65M  260G   1% /media/nvme
```
```bash
#ensure that the /media/nvme directory is owned by ubuntu by typing ls -la /media/nvme If it is not then type the following command
sudo chown -R ubuntu:ubuntu /media/nvme/
```

# Installing database (RocksDB)
Create directory for database
```bash
mkdir /media/nvme/ssvm_database
```

Install system requirements
```
sudo apt-get -y install devscripts debhelper build-essential fakeroot zlib1g-dev libbz2-dev libsnappy-dev libgflags-dev libzstd-dev make clang pkg-config libssl-dev
```
Install RocksDB
```
git clone https://github.com/ulikoehler/deb-buildscripts.git
cd deb-buildscripts
./deb-rocksdb.py
```
Configure RocksDB - this (setting the data dir and options) is all done in the source code, as per the example below. There is nothing to do here ... please move to the next section.
```
let path = "/media/nvme/ssvm_database";
let db = DB::open_default(path).unwrap();
let mut opts = Options::default();
opts.increase_parallelism(3);
opts.create_if_missing(true);
```

# Compiling rust_native_storage_library to system level executables
Fetch this software
```
cd ~
git clone https://github.com/second-state/rust_native_storage_library.git
```
Add this target to your system
```
rustup target add x86_64-unknown-linux-gnu
```
Edit your ~/.cargo/conf file
```
[target.x86_64-unknown-linux-gnu]
linker = "x86_64-unknown-linux-gnu-gcc"
```
Build
```
cd ~/rust_native_storage_library
cargo build --release --target x86_64-unknown-linux-gnu
```
Check the newly compiled dynamic library to see if functions have been successfully exported
```
nm --defined-only  -D target/x86_64-unknown-linux-gnu/release/librust_native_storage_library.so
```
As you can see, our Rust code has made the following functions available via the `.so`
```
0000000000043ea0 T free
0000000000043b30 T load_data
0000000000043700 T store_data
```
## Testing the executables, call using C++
If you want to call these functions from within a C++ application, please use the following method.
Write the following C++ code which will provide you with access to the `.so` functions.
```

```
Compile this C++ code using the following command
```
g++ -g implementation_examples/c++_implementation.cpp -o c++_implementation -lrust_native_storage_library -Ltarget/x86_64-unknown-linux-gnu/release

```
Allow the linker to find the dynamic library (update the standard paths for shared libs)
```
LD_LIBRARY_PATH=target/x86_64-unknown-linux-gnu/release ldd c++_implementation
```
Execute the Rust code via the `.so` via our C++_implementation
```
LD_LIBRARY_PATH=target/x86_64-unknown-linux-gnu/release ./c++_implementation
```
## Testing the executables, call using Python
You can just run the following Python file to test this software. You will notice that the location of the dynamic library is already configured in the Python file. If you want to call this from other applications, please note that the library is built (and can therefore be found) in the following location `~/rust_native_storage_library/target/x86_64-unknown-linux-gnu/release/librust_native_storage_library.so`
```
cd ~/rust_native_storage_library
python3.6 implementation_examples/python_implementation.py
```
