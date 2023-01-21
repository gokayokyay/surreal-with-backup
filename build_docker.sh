./build_surreal.sh
./build_backman.sh

docker build -t gokayokyay/surreal-backup:latest .

if [ $# -eq 0 ]
  then
    echo "No arguments supplied"
    echo "Only using tag 'latest'"
    exit 0
fi

docker tag gokayokyay/surreal-backup:latest gokayokyay/surreal-backup:$1
