import signal
import sys
import time
import random
import shutil
import timeit


RANGE = 100
COLUMNS = shutil.get_terminal_size().columns


def signal_handler(signum, frame):
	print("\nThanks for playing!")
	sys.exit(0)


signal.signal(signal.SIGINT, signal_handler)


def outstr(index, total, scale=1.0):
	width = round(100 * scale)
	current = round((index/total * 100.0) * scale)
	percentage = round(index/total * 100.0)
	return "[{}>{}] {}%".format("=" * current, " " * round(width - current), percentage)


def start(count):
	all_progress_bars = [0] * count
	sys.stdout.write('\n' * count)

	while any(x < RANGE for x in all_progress_bars):
		loading = [(i, v) for (i, v) in enumerate(all_progress_bars) if v < 100]
		index, _ = random.choice(loading)
		all_progress_bars[index] += 1

		sys.stdout.write('\u001b[{}D'.format(COLUMNS))
		sys.stdout.write('\u001b[{}A'.format(count))

		for bar in all_progress_bars:
			print(outstr(bar, RANGE, 1.0))


begin = timeit.default_timer()
start(30)
end = timeit.default_timer() - begin 

print("Time: {}s".format(end))

