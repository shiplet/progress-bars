import signal
import sys
import time


RANGE = 100


def signal_handler(signum, frame):
	print("\nThanks for playing!")
	sys.exit(0)


signal.signal(signal.SIGINT, signal_handler)


def outstr(index, total, scale=1.0):
	width = round(100 * scale)
	current = round((index/total * 100.0) * scale)
	percentage = round(index/total * 100.0) + 1
	return "\r[{}>{}] {}%".format("=" * current, " " * round(width - current), percentage)


for i in reversed(range(5)):
	for j in range(RANGE):
		sys.stdout.write(outstr(j, RANGE, float(i + 1)/10))
		sys.stdout.flush()
		time.sleep(0.01)
	print()
