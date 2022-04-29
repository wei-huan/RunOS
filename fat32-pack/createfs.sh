sudo rm -rf ./fat32.img
dd if=/dev/zero of=fat32.img bs=512KB count=256
sudo mkfs.vfat -F 32 fat32.img
sudo chmod -R 777 fat32.img
