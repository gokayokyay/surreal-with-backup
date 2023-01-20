echo "WARNING!"
echo "This script is meant for vms or containers"
echo "Please just install surrealdb official way instead of compiling it..."

rm build_out.log 2> /dev/null

# https://github.com/soedinglab/hh-suite/issues/280#issuecomment-1076146671
sudo mkdir -p /var/cache/swap/
sudo fallocate -l 16G /var/cache/swap/swapfile
sudo chmod 600 /var/cache/swap/swapfile
sudo mkswap /var/cache/swap/swapfile

(cd surrealdb && make build >> ../build_out.log 2>&1)

sudo swapoff --all
