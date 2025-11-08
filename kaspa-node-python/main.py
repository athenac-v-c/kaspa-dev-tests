"""
import requests
import json

RPC_URL = "http://127.0.0.1:18110"

def get_block_count():
    payload = {
        'jsonrpc':"2.0",
        "id":"test",
        "method":"getBlockCount",
        "params":[]
    }
    response = requests.post(RPC_URL, json = payload)
    data = response.json()
    return data

if __name__ == "__main__":
    result = get_block_count()
    print(result)
"""
import asyncio
import kaspa


async def main():

    client = kaspa.RpcClient(
        url="ws://127.0.0.1:17210", 
        resolver=None
    )


    await client.connect()


    info = await client.get_server_info()
    print(info)

 
    block_count = await client.get_block_count()
    print(block_count)

    await client.disconnect()


if __name__ == "__main__":
    asyncio.run(main())






