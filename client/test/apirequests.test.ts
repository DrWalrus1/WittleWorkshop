import { expect, test } from 'vitest';
import { GetDockerContainers, GetDockerImages } from '../src/apirequests';


test('Get Docker Containers JSON', async () => {
    const response = await GetDockerContainers('http://127.0.0.1:8000');
    const data = await response.json();
    console.log(data);
    // assert that the response is anything
    expect(data).toBeDefined();
});
