export BUILD_IMG_TAG=0.0.0001

sh .docker/check-docker.sh

cp .docker/Dockerfile ./

docker buildx build \
   --platform=linux/amd64 \
   --tag ccr.ccs.tencentyun.com/mythosma/rust-jx3-server:$BUILD_IMG_TAG \
   --push -t ccr.ccs.tencentyun.com/mythosma/rust-jx3-server:$BUILD_IMG_TAG \
   --no-cache \
   .

rm -f Dockerfile
rm -rf dist

