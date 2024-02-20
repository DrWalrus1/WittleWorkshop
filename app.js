async function CallContainerApi(command) {
    let request = await fetch("http://localhost:8000/container",
        {
            method: "POST",
            mode: "cors",
            headers: {
                "Content-Type": "application/json",
                "Access-Control-Allow-Origin": "*",
                "Access-Control-Allow-Headers": "*"
            },
            body: JSON.stringify({
                command: command
            })
        }).then(function (result) {
            return result;
        });

        console.log(await request.json())
}