#!/bin/bash
sudo modprobe vcan

sudo ip link delete vcan0 type vcan 2>/dev/null
sudo ip link delete vcan1 type vcan 2>/dev/null

sudo ip link add dev vcan0 type vcan
sudo ip link add dev vcan1 type vcan
sudo ip link set up vcan0
sudo ip link set up vcan1
