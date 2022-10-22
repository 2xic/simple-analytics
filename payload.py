import requests
import concurrent.futures
import time

def get_status():
    return requests.post(
        "http://localhost:8080",
        data="hello world!"
    ).status_code

with concurrent.futures.ThreadPoolExecutor() as executor:

    futures = []

    for _ in range(10):
        futures.append(executor.submit(get_status))

    for future in concurrent.futures.as_completed(futures):
        print(future.result())
