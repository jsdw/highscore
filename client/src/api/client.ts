/** A simple API client */
export async function client(name: string, params?: any) {
    const headers: Record<string,string> = {
        'Content-Type': 'application/json'
    }

    const method = typeof params === "undefined"
        ? "GET"
        : "POST"

    const res = await fetch(`/api/${name}`, {
        method,
        headers,
        body: params ? JSON.stringify(params) : undefined
    })

    if (!res.ok) {
        return Promise.reject(new Error(`Response to ${name} was (${res.status}):\n\n${res.body}`))
    }

    const res_json = await res.json()
    return res_json
}