#!/bin/bash

#virsh -c 'qemu:///system' list --all

virsh -c 'qemu:///system' start 'ubuntu20'
ssh ubuntu20.oneiric
