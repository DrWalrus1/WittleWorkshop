export function GetDockerContainers(dockerBaseUrl: string) {
    return fetch(dockerBaseUrl + '/docker/containers');
}

export function GetDockerImages(dockerBaseUrl: string) {
    return fetch(dockerBaseUrl + '/docker/images');
}