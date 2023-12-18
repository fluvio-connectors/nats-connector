cloud_e2e_test:
	bats ./tests/cloud-consumes-data-from-nats.bats

test_fluvio_install:
	sleep 10
	fluvio version
	fluvio topic list
	fluvio topic create foobar
	sleep 3
	echo foo | fluvio produce foobar
	fluvio consume foobar -B -d
