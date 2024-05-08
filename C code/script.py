import subprocess
import threading
import numpy as np

values = list(np.linspace(0.17, 0.4, 300))

max_processes = 12


lock = threading.Lock()


def run_subprocess(value):
    command = ['./a.out', '-A', str(value), '-N', str(1000), '-a', str(0.85)]
    subprocess.call(command)


def run_processes():
    while True:
        lock.acquire()

        if len(values) == 0:
            lock.release()
            break

        value = values.pop(0)

        lock.release()

        run_subprocess(value)

threads = []
for _ in range(max_processes):
    thread = threading.Thread(target=run_processes)
    thread.start()
    threads.append(thread)

for thread in threads:
    thread.join()