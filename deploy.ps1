# ./deploy ('host1', 'host2')
Param($deploy_targets)

$IMAGE_NAME = "morse"

docker buildx build --platform=linux/arm/v7 -t $IMAGE_NAME .
Write-Output "> save image to ${IMAGE_NAME}.tar.gz"
docker save -o "${IMAGE_NAME}.tar.gz" $IMAGE_NAME

foreach ($target in $deploy_targets) {
    Write-Output "> copy to host $target"
    scp "${IMAGE_NAME}.tar.gz" pi@${target}:/home/pi
    Write-Output "> load image"
    ssh pi@${target} docker load -i "${IMAGE_NAME}.tar.gz"
    ssh pi@${target} 'docker stop morsec && docker rm morsec'
    ssh pi@${target} docker run --name morsec --privileged -d -p '80:80' morse
}